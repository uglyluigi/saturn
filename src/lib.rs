//Meta Modules
pub mod prelude;
pub mod schema;

//Domain Modules
pub mod models;
pub mod controllers;

//Macro Imports
#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_migrations;

//Meta Imports
use prelude::*;

//Macro Calls
embed_migrations!();

//Meta Structs
#[database("saturn")]
pub struct Db(diesel::PgConnection);

//Meta Types
pub type Result<T, E = Debug<diesel::result::Error>> = std::result::Result<T, E>;

//---
//Meta Functions
//---
pub fn rocket() -> Rocket<Build>{
    //Env initialization
    dotenv().ok();

    //DB url initialization
    let db: Map<_, Value> = map! {
        "url" => env::var("DATABASE_URL").expect("DATABASE_URL must be set").into()
    };

    //Set Limits
    let limits = Limits::default()
        .limit("file", 0.kibibytes())
        .limit("file/png", 1.megabytes())
    ;

    //Build config
    let mut figment = Figment::from(rocket::Config::default())
        .merge(("address", "0.0.0.0"))
        .merge(("port", 443))
        .merge(("databases", map!["saturn" => db]))
        .merge(("secret_key", env::var("SECRET_KEY").expect("TLS_CERT_PATH must be set")))
        .merge(("limits", limits))
    ;

    if env::var("IN_PRODUCTION").expect("TLS_CERT_PATH must be set") == "TRUE"{
        figment = figment
            .merge(("tls.certs", env::var("TLS_CERT_PATH").expect("TLS_CERT_PATH must be set")))
            .merge(("tls.key", env::var("TLS_KEY_PATH").expect("TLS_KEY_PATH must be set")))
    }

    //Build rocket object
    rocket::custom(figment)
        //Diesel
        .attach(Db::fairing())
        .attach(AdHoc::on_ignite("Diesel Migrations", run_migrations))
        //GoogleKeyState
        .manage(GoogleKeysState {
            lock: Arc::new(RwLock::new(GoogleKeys {
                keys: Vec::new(),
                expires: chrono::offset::Utc::now(),
            }))
        })
        //Startup
        .mount("/api/", routes![
            controllers::clubs::get::get_all,
            controllers::clubs::get::get_clubs_by_membership,
            controllers::clubs::get::get_clubs_by_moderatorship,
            controllers::clubs::create::create,
            controllers::clubs::update::renew,
            controllers::clubs::update::join,
            controllers::clubs::update::leave,
            controllers::clubs::update::appoint,
            controllers::clubs::update::upload,
            controllers::clubs::delete::delete_admin,
            controllers::clubs::delete::delete_user,
            controllers::auth::login::login,
            controllers::auth::logout::logout,
            controllers::auth::details::details_admin,
            controllers::auth::details::details_user,
        ])
        .register("/api", catchers![
            controllers::auth::details::forbidden_or_details_guest
        ])
        .mount("/.well-known", FileServer::from(relative!(".well-known")))
        .mount("/", FileServer::from(relative!("src/clientapp/dist")).rank(-1))
        .mount("/assets/clubs", FileServer::from(relative!("uploads")).rank(-2))
        .attach(AdHoc::on_response("404 Redirector", |_req, res| Box::pin(async move {
            if res.status() == Status::NotFound {
                let body = std::fs::read_to_string("src/clientapp/dist/index.html").expect("Index file can't be found.");

                res.set_status(Status::Ok);
                res.set_header(ContentType::HTML);
                res.set_sized_body(body.len(), Cursor::new(body));
            }
            return
        })))
}

async fn run_migrations(rocket: Rocket<Build>) -> Rocket<Build> {
    // This macro from `diesel_migrations` defines an `embedded_migrations`
    // module containing a function named `run` that runs the migrations in the
    // specified directory, initializing the database.
    embed_migrations!();

    let conn = Db::get_one(&rocket).await.expect("database connection");
    conn.run(|c| embedded_migrations::run(c)).await.expect("diesel migrations");

    rocket
}




//Stufff
pub struct GoogleKeysState {
    lock: Arc<RwLock<GoogleKeys>>,
}

impl GoogleKeysState {
    pub async fn fetch_keys(&self) -> Vec<GoogleKey>{
        {
            let keys = self.lock.read().unwrap();
            if (*keys).expires > chrono::offset::Utc::now(){
                return (*keys).keys.clone();
            }
        }
        let request = reqwest::get("https://www.googleapis.com/oauth2/v3/certs").await.unwrap();
        let expiry_date = chrono::DateTime::parse_from_rfc2822(request.headers().get("expires").unwrap().to_str().unwrap()).unwrap();
        let body = request.json::<HashMap<String, Vec<GoogleKey>>>().await.unwrap();
        let new_keys = body.get("keys").unwrap();
        {
            let mut keys = self.lock.write().unwrap();
            (*keys).expires = expiry_date.into();
            (*keys).keys = new_keys.to_vec();
        }
        new_keys.clone()
    }
}

struct GoogleKeys {
    keys: Vec<GoogleKey>,
    expires: chrono::DateTime<Utc>
}

#[allow(dead_code)]
#[derive(Deserialize, Clone)]
pub struct GoogleKey {
    n: String,
    e: String,
    alg: String,
    kty: String,
    r#use: String,
    kid: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GoogleClaims {
    iss: String,
    nbf: usize,
    aud: String,
    sub: String,
    hd: String,
    email: String,
    email_verified: bool,
    azp: String,
    name: String,
    picture: String,
    given_name: String,
    family_name: String,
    iat: usize,
    exp: usize,
    jti: String
}

pub struct UserAuthenticator {
    email: Box<String>,
    picture: Box<String>,
    first_name: Box<String>,
    last_name: Box<String>,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for UserAuthenticator {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        //Create variables to hold potential output.
        let mut email = None;
        let mut picture = String::new();
        let mut first_name = String::new();
        let mut last_name = String::new();

        //Fetch the jwt token from the user's computer and set our validation algorithm.
        let jwt = req.cookies().get_private("user_jwt").map(|cookie| Box::new(cookie.value().to_owned()));
        let validation = Validation::new(Algorithm::RS256);

        //Retrieve google's public keys to verify signature.
        let key_state = try_outcome!(req.guard::<&State<GoogleKeysState>>().await);
        
        //Load each key and check the signature and claims.
        for key in key_state.fetch_keys().await{
            let jwt = jwt.clone();
            if let Some(jwt) = jwt{
                match decode::<GoogleClaims>(&jwt, &DecodingKey::from_rsa_components(&key.n, &key.e), &validation) {
                    Ok(c) => {email = Some(c.claims.email); picture=c.claims.picture; first_name=c.claims.given_name; last_name=c.claims.family_name;},
                    Err(e) => {println!("{:?}", e)}
                };
            }

            if email.is_some() {break}
        }

        if email.is_none() {req.cookies().remove_private(Cookie::named("user_jwt"));}

        //If we were able to decode the email proceed otherwise return forbidden status.
        match email {
            None => Outcome::Failure((Status::Forbidden, ())),
            Some(email) => Outcome::Success(
                UserAuthenticator{
                    email: Box::new(email),
                    picture: Box::new(picture),
                    first_name: Box::new(first_name),
                    last_name: Box::new(last_name)
                }),
        }
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for User{

    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        use crate::schema::users::dsl::{users, email};
        let db = try_outcome!(req.guard::<Db>().await);
        let auth = try_outcome!(req.guard::<UserAuthenticator>().await);

        //Clone fields for creating users.
        let used_email = auth.email.clone();
        let used_picture = auth.picture.clone();
        let used_first_name = auth.first_name.clone();
        let used_last_name = auth.last_name.clone();

        //Search the database users.
        let user = db.run(move |conn| {
            users
                .filter(email.eq(&auth.email as &str))
                .first::<User>(conn)
                .optional()
        }).await.unwrap();

        //Does user exist? Return it. Otherwise create them.
        if let Some(mut user) = user {
            //If no changes to email and name just return it.
            if 
                &user.email == used_email.as_str() &&
                &user.picture == used_picture.as_str() &&
                &user.first_name == used_first_name.as_str() &&
                &user.last_name == used_last_name.as_str()
            {
                Outcome::Success(user)

            //Otherwise update the record and return that.
            } else {
                user.picture=used_picture.to_string();
                user.first_name=used_first_name.to_string();
                user.last_name=used_last_name.to_string();
            let user = db.run(move |conn| {
                diesel::update(users).set(&user).get_result(conn)
            }).await.unwrap();
                Outcome::Success(user)
            }
        } else {
            //User didn't exist so we're creating them.
            let user = db.run(move |conn| {
                let new_user = NewUser {
                    email: &used_email,
                    picture: &used_picture,
                    first_name: &used_first_name,
                    last_name: &used_last_name,
                    is_admin: &false
                };
                insert_into(users)
                    .values(&new_user.clone())
                    .get_result(conn)
                    .optional()
            }).await.unwrap();
            Outcome::Success(user.unwrap())
        }
    }
}


#[rocket::async_trait]
impl<'r> FromRequest<'r> for Admin{

    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let auth = try_outcome!(req.guard::<User>().await);
        
        //Pretty straight forward just load the user and check if they're an admin.
        if auth.is_admin {
            Outcome::Success(Admin(auth))
        } else {
            Outcome::Forward(())
        }


    }
}

#[derive(Serialize)]
pub struct JsonError{
    pub error: String,
}