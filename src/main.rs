extern crate saturn;

#[rocket::main]
async fn main() {
    saturn::rocket().launch().await.unwrap();
}