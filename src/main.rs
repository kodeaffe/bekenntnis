#[macro_use]
extern crate rocket;

mod routes;
use routes::{home, static_files};


#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![home, static_files])
}