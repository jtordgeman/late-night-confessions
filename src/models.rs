use crate::schema::confessions;
use serde::{Serialize};

#[derive(Queryable, Serialize)]
pub struct Confession {
    pub id: i32,
    pub confession: String,
}

#[derive(Insertable)]
#[table_name = "confessions"]
pub struct NewConfession<'a> {
    pub confession: &'a str,
}
