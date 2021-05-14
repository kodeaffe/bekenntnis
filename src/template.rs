use askama::Template;

#[derive(Template)]
#[template(path = "home.html")]
pub struct Home {
    pub bekenntnis: String,
    pub total_bekenntnisse: i64,
}