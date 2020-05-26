#![feature(proc_macro_hygiene, decl_macro)]
extern crate postgres;
extern crate postgres_types;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use std::fs;

use postgres::{Client, Error, NoTls};

mod models;
mod requests;

fn main() -> Result<(), Error> {
    dotenv::dotenv().ok();

    match validate_db_connection() {
        Ok(mut client) => {
            client.batch_execute(&fs::read_to_string("src/sql/init_tables.sql").unwrap())?
        }
        Err(error) => return Err(error),
    }

    rocket::ignite()
        .mount("/test", routes![requests::util::test_connection])
        .mount(
            "/books",
            routes![
                requests::book::all,
                requests::book::by_id,
                requests::book::by_published,
                requests::book::by_author_id,
                requests::book::add,
                requests::book::update,
                requests::book::delete,
                requests::book::delete_by_author
            ],
        )
        .mount(
            "/authors",
            routes![
                requests::author::all,
                requests::author::by_id,
                requests::author::add,
                requests::author::update,
                requests::author::delete
            ],
        )
        .launch();

    Ok(())
}

fn validate_db_connection() -> Result<Client, Error> {
    Client::connect(
        &format!(
            "{}://{}:{}@{}/{}",
            dotenv::var("DATABASE_TYPE").unwrap(),
            dotenv::var("DATABASE_USERNAME").unwrap(),
            dotenv::var("DATABASE_PASSWORD").unwrap(),
            dotenv::var("DATABASE_HOST").unwrap(),
            dotenv::var("DATABASE_NAME").unwrap()
        ),
        NoTls,
    )
}
