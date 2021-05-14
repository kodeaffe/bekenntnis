use rocket::response::status::NotFound;
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
