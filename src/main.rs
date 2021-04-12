#[macro_use]
extern crate rocket;

#[macro_use]
extern crate diesel;

mod error;
mod models;
mod schema;

use rocket::Rocket;

use rocket::figment::map;
use rocket::figment::value::{Map, Value};

use rocket::response::content;
use rocket::response::status::{Created, NotFound};
use rocket::response::NamedFile;

use rocket_contrib::databases::database;
use rocket_contrib::json::Json;

use serde::{Deserialize, Serialize};

use diesel::pg::PgConnection;
use diesel::prelude::*;

use dotenv::dotenv;

use std::env;
use std::path::PathBuf;

use error::CustomError;

use models::{Confession, NewConfession};

use askama::Template;

no_arg_sql_function!(RANDOM, (), "Represents the sql RANDOM() function");

#[derive(Template)]
#[template(path = "index.html")]
struct HomepageTemplate {
    confession: String,
    total_confessions: i64,
}

#[derive(Deserialize, Debug)]
struct ConfessionJSON {
    content: String,
}

#[derive(Serialize)]
struct NewConfessionResponse {
    confession: Confession,
}

#[database("confessions_db")]
pub struct DBPool(PgConnection);

fn get_random_confession(conn: &PgConnection) -> Result<Confession, diesel::result::Error> {
    schema::confessions::table
        .order(RANDOM)
        .limit(1)
        .first::<Confession>(conn)
}

#[get("/")]
async fn root(conn: DBPool) -> Result<content::Html<String>, CustomError> {
    let confession_from_db: Confession = conn.run(|c| get_random_confession(c)).await?;
    let confession_count: i64 = conn
        .run(|c| schema::confessions::table.count().get_result(c))
        .await?;

    let template = HomepageTemplate {
        confession: confession_from_db.confession,
        total_confessions: confession_count,
    };

    let response = content::Html(template.to_string());
    Ok(response)
}

#[post("/confession", format = "json", data = "<confession>")]
async fn post_confession(
    conn: DBPool,
    confession: Json<ConfessionJSON>,
) -> Result<Created<Json<NewConfessionResponse>>, CustomError> {
    let new_confession: Confession = conn
        .run(move |c| {
            diesel::insert_into(schema::confessions::table)
                .values(NewConfession {
                    confession: &confession.content,
                })
                .get_result(c)
        })
        .await?;

    let response = NewConfessionResponse {
        confession: new_confession,
    };

    Ok(Created::new("/confession").body(Json(response)))
}

#[get("/confession", format = "json")]
async fn get_confession(conn: DBPool) -> Result<Json<Confession>, CustomError> {
    let confession: Confession = conn.run(|c| get_random_confession(c)).await?;

    Ok(Json(confession))
}

#[get("/<path..>")]
async fn static_files(path: PathBuf) -> Result<NamedFile, NotFound<String>> {
    let path = PathBuf::from("site").join(path);
    NamedFile::open(path)
        .await
        .map_err(|e| NotFound(e.to_string()))
}

#[launch]
fn rocket() -> Rocket {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").unwrap();
    let db: Map<_, Value> = map! {
        "url" => db_url.into(),
        "pool_size" => 10.into()
    };
    let figment = rocket::Config::figment().merge(("databases", map!["confessions_db" => db]));

    rocket::custom(figment)
        .mount("/", routes![root, static_files])
        .mount("/api", routes![post_confession, get_confession])
        .attach(DBPool::fairing())
}
