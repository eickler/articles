#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate diesel;
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;

use dotenv::{dotenv,var};
use std::collections::HashMap;
use rocket::config::{Config, Environment, Value, LoggingLevel};
use rocket_contrib::databases;

pub mod auth;
pub mod articles;
pub mod schema;

#[database("diesel_postgres_pool")]
pub struct Conn(databases::diesel::PgConnection);

fn db_config(url: String) -> HashMap<String,Value> {
    let mut database_config = HashMap::new();
    let mut databases = HashMap::new();
    database_config.insert(String::from("url"), url);
    databases.insert(String::from("diesel_postgres_pool"), Value::from(database_config));
    databases
}

fn main() {
    dotenv().ok();

    let db_url = var("DATABASE_URL").unwrap();
    let config = Config::build(Environment::Development)
        .log_level(LoggingLevel::Debug)
        .extra("databases", db_config(db_url))
        .finalize()
        .unwrap();

    rocket::custom(config)
        .mount("/articles", articles::routes())
        .attach(Conn::fairing())
        // Auth is probably attached in the same way as pools.
        .launch();
}
