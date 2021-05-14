use crate::dbpool::DBPool;
use crate::error::CustomError;
use crate::models::{Bekenntnis, InsertBekenntnis};
use crate::schema;
use diesel::query_dsl::{QueryDsl, RunQueryDsl};
use rocket_contrib::json::Json;
use rocket::response::status::{Created, NotFound};
use rocket::response::NamedFile;
use std::path::PathBuf;


#[get("/")]
pub async fn home() -> Result<NamedFile, NotFound<String>> {
    NamedFile::open("templates/home.html")
        .await
        .map_err(|e| NotFound(e.to_string()))
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

// No random ordering within diesel?
no_arg_sql_function!(RANDOM, (), "Represents the sql RANDOM() function");

#[get("/bekenntnis", format = "json")]
pub async fn get_bekenntnis(conn: DBPool) -> Result<Json<Bekenntnis>, CustomError> {
    let bekenntnis: Bekenntnis = conn
        .run(|c| {
            schema::bekenntnis::table
                .order(RANDOM)
                .limit(1)
                .first::<Bekenntnis>(c)
        })
        .await?;
    Ok(Json(bekenntnis))
}