use std::path::PathBuf;

use diesel::pg::PgConnection;
use diesel::query_dsl::{QueryDsl, RunQueryDsl};
use rocket_contrib::json::Json;
use rocket::response::content;
use rocket::response::status::{Created, NotFound};
use rocket::response::NamedFile;

use crate::dbpool::DBPool;
use crate::error::CustomError;
use crate::models::{Bekenntnis, InsertBekenntnis};
use crate::schema;
use crate::template::Home;


// No random ordering within diesel?
no_arg_sql_function!(RANDOM, (), "Represents the sql RANDOM() function");

fn get_random_bekenntnis(conn: &PgConnection) -> Result<Bekenntnis, diesel::result::Error> {
    schema::bekenntnis::table
        .order(RANDOM)
        .limit(1)
        .first::<Bekenntnis>(conn)
}


#[get("/")]
pub async fn home(conn: DBPool) -> Result<content::Html<String>, CustomError> {
    let bekenntnis: Bekenntnis = conn.run(|c| get_random_bekenntnis(c)).await?;
    let total_bekenntnisse: i64 = conn.run(|c| {
        schema::bekenntnis::table.count().get_result(c)
    }).await?;
    let template = Home {
        bekenntnis: bekenntnis.content,
        total_bekenntnisse,
    };
    let response = content::Html(template.to_string());
    Ok(response)
}


#[get("/static/<path..>")]
pub async fn static_files(path: PathBuf) -> Result<NamedFile, NotFound<String>> {
    let path = PathBuf::from("static").join(path);
    NamedFile::open(path).await.map_err(|e| NotFound(e.to_string()))
}


#[post("/bekenntnis", format = "json", data = "<data>")]
pub async fn post_bekenntnis(
    conn: DBPool,
    data: Json<InsertBekenntnis>,
) -> Result<Created<Json<Bekenntnis>>, CustomError> {
    let bekenntnis: Bekenntnis = conn
        .run(move |c| {
            diesel::insert_into(schema::bekenntnis::table)
                .values(data.into_inner())
                .get_result(c)
        })
        .await?;
    Ok(Created::new("/bekenntnis").body(Json(bekenntnis)))
}


#[get("/bekenntnis", format = "json")]
pub async fn get_bekenntnis(conn: DBPool) -> Result<Json<Bekenntnis>, CustomError> {
    let bekenntnis: Bekenntnis = conn.run(|c| get_random_bekenntnis(c)).await?;
    Ok(Json(bekenntnis))
}