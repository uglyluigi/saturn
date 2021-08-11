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
    let figment = Figment::from(rocket::Config::default())
        .merge(("address", "0.0.0.0"))
        .merge(("port", 8000))
        .merge(("databases", map!["saturn" => db]))
    ;

    //Build rocket object
    rocket::custom(figment)
        //Diesel
        .attach(Db::fairing())
        .attach(AdHoc::on_ignite("Diesel Migrations", run_migrations))
        //Startup
        .mount("/", routes![controllers::index::index])
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
