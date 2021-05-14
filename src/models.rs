use crate::schema::bekenntnis;
use diesel::{Queryable, Insertable};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize)]
pub struct Bekenntnis {
    pub id: i32,
    pub content: String,
}


#[derive(Insertable, Deserialize)]
#[table_name="bekenntnis"]
pub struct InsertBekenntnis {
    pub content: String,
}