#[macro_use] extern crate diesel;
#[macro_use] extern crate rocket;

mod dbpool;
mod error;
mod models;
mod routes;
mod schema;

use dbpool::DBPool;
use dotenv;
use rocket::figment::value::{Map, Value};
use rocket::figment::util::map;
use routes::{get_bekenntnis, home, post_bekenntnis, static_files};
use std::env;


#[launch]
fn rocket() -> _ {
    dotenv::dotenv().ok();

    let db_url = env::var("DATABASE_URL").unwrap();
    let db: Map<_, Value> = map! {
        "url" => db_url.into(),
        "pool_size" => 10.into()
    };
    let figment = rocket::Config::figment().merge(
        ("databases", map!["bekenntnis_db" => db]));

    rocket::custom(figment)
        .mount("/", routes![home, static_files])
        .mount("/api", routes![get_bekenntnis, post_bekenntnis])
        .attach(DBPool::fairing())
}