use rocket::http::Status;
use rocket_contrib::json::Json;
use serde_json::Value;

use crate::models::author::{Author, NewAuthor};
use crate::models::client_wrapper::DBConnection;
use crate::requests::util;

#[get("/")]
pub fn all(mut db_conn: DBConnection) -> Json<Value> {
    let (status, authors) = match db_conn.client.query("select * from author", &[]) {
        Ok(rows) => (Status::Ok, Some(util::rows_to_values::<Author>(rows))),
        Err(_) => (Status::NotFound, None),
    };

    util::build_response(status, authors)
}

#[get("/<id>")]
pub fn by_id(id: i32, mut db_conn: DBConnection) -> Json<Value> {
    let (status, author) = match db_conn
        .client
        .query_one("select * from author where id = $1", &[&id])
    {
        Ok(row) => (Status::Ok, Some(Author::from(row))),
        Err(_) => (Status::NotFound, None),
    };

    util::build_response(status, author)
}

#[put("/add", format = "application/json", data = "<author>")]
pub fn add(author: Json<NewAuthor>, mut db_conn: DBConnection) -> Json<Value> {
    let status = match db_conn.client.execute(
        "insert into author values (default, $1, $2)",
        &[&author.name, &author.country],
    ) {
        Ok(_) => Status::Ok,
        Err(_) => Status::NotModified,
    };

    util::build_simple_response(status)
}

#[post("/update/<id>", format = "application/json", data = "<author>")]
pub fn update(id: i32, author: Json<NewAuthor>, mut db_conn: DBConnection) -> Json<Value> {
    let status = match db_conn.client.execute(
        "update author set (name, country) = ($1, $2) where id = $3",
        &[&author.name, &author.country, &id],
    ) {
        Ok(_) => Status::Ok,
        Err(_) => Status::NotModified,
    };

    util::build_simple_response(status)
}

#[delete("/delete/<id>")]
pub fn delete(id: i32, mut db_conn: DBConnection) -> Json<Value> {
    let status = match db_conn
        .client
        .execute("delete from author where id = $1", &[&id])
    {
        Ok(_) => Status::Ok,
        Err(_) => Status::NotModified,
    };

    util::build_simple_response(status)
}
