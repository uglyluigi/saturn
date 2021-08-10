pub mod prelude;
pub mod controllers;

#[macro_use] extern crate rocket;

use prelude::*;

pub fn rocket() -> Rocket<Build>{
    rocket::build()
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