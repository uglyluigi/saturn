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

    //Build config
    let mut figment = Figment::from(rocket::Config::default())
        .merge(("address", "0.0.0.0"))
        .merge(("port", 443))
        .merge(("databases", map!["saturn" => db]))
        .merge(("secret_key", env::var("SECRET_KEY").expect("TLS_CERT_PATH must be set")))
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
        //Startup
        .mount("/api/", routes![
            controllers::clubs::get::get_all,
            controllers::clubs::create::create,
            controllers::clubs::delete::delete_admin,
            controllers::clubs::delete::delete_user,
            controllers::auth::login::login,
            controllers::auth::logout::logout,
            controllers::auth::details::details_admin,
            controllers::auth::details::details_user,
            controllers::auth::details::details_guest,
        ])
        .mount("/.well-known", FileServer::from(relative!(".well-known")))
        .mount("/", FileServer::from(relative!("src/clientapp/dist")).rank(-1))
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
#[derive(Debug, Serialize, Deserialize)]
struct GoogleClaims {
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
        let mut email = None;
        let mut picture = String::new();
        let mut first_name = String::new();
        let mut last_name = String::new();

        let jwt = req.cookies().get_private("user_jwt").map(|cookie| Box::new(cookie.value().to_owned()));
        let validation = Validation::new(Algorithm::RS256);

        let body = reqwest::get("https://www.googleapis.com/oauth2/v3/certs").await.unwrap().json::<HashMap<String, Vec<GoogleKey>>>().await.unwrap();

        println!("User get keys.");

        for key in body.get("keys").unwrap(){
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

#[derive(Deserialize)]
struct GoogleKey {
    n: String,
    e: String,
    alg: String,
    kty: String,
    r#use: String,
    kid: String
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
        //use crate::schema::users::dsl::{users, email};
        //let db = try_outcome!(req.guard::<Db>().await);
        let auth = try_outcome!(req.guard::<User>().await);
        
        if auth.is_admin {
            Outcome::Success(Admin(auth))
        } else {
            Outcome::Forward(())
        }


    }
}