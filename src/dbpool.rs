use diesel::pg::PgConnection;
use rocket_contrib::databases::database;

#[database("bekenntnis_db")]
pub struct DBPool(PgConnection);
