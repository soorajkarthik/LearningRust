use rocket::http::Status;
use rocket_contrib::json::Json;
use serde_json::Value;

use crate::models::book::{Book, NewBook};
use crate::models::client_wrapper::DBConnection;
use crate::requests::util;

#[get("/")]
pub fn all(mut db_conn: DBConnection) -> Json<Value> {
    let (status, books) = match db_conn.client.query("select * from book", &[]) {
        Ok(rows) => (Status::Ok, Some(util::rows_to_values::<Book>(rows))),
        Err(_) => (Status::NotFound, None),
    };

    util::build_response(status, books)
}

#[get("/<id>")]
pub fn by_id(id: i32, mut db_conn: DBConnection) -> Json<Value> {
    let (status, book) = match db_conn
        .client
        .query_one("select * from book where id = $1", &[&id])
    {
        Ok(row) => (Status::Ok, Some(Book::from(row))),
        Err(_) => (Status::NotFound, None),
    };

    util::build_response(status, book)
}

#[get("/published/<published>")]
pub fn by_published(published: bool, mut db_conn: DBConnection) -> Json<Value> {
    let (status, books) = match db_conn
        .client
        .query("select * from book where published = $1", &[&published])
    {
        Ok(rows) => (Status::Ok, Some(util::rows_to_values::<Book>(rows))),
        Err(_) => (Status::NotFound, None),
    };

    util::build_response(status, books)
}

#[get("/by/<id>")]
pub fn by_author_id(id: i32, mut db_conn: DBConnection) -> Json<Value> {
    let (status, books) = match db_conn
        .client
        .query("select * from book where authorId = $1", &[&id])
    {
        Ok(rows) => (Status::Ok, Some(util::rows_to_values::<Book>(rows))),
        Err(_) => (Status::NotFound, None),
    };

    util::build_response(status, books)
}

#[put("/add", format = "application/json", data = "<book>")]
pub fn add(book: Json<NewBook>, mut db_conn: DBConnection) -> Json<Value> {
    let status = match db_conn.client.execute(
        "insert into book values (default, $1, $2, $3)",
        &[&book.title, &book.author_id, &book.published],
    ) {
        Ok(_) => Status::Ok,
        Err(_) => Status::NotModified,
    };

    util::build_simple_response(status)
}

#[post("/update/<id>", format = "application/json", data = "<book>")]
pub fn update(id: i32, book: Json<NewBook>, mut db_conn: DBConnection) -> Json<Value> {
    let status = match db_conn.client.execute(
        "update book set (title, authorId, published) = ($1, $2, $3) where id = $4",
        &[&book.title, &book.author_id, &book.published, &id],
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
        .execute("delete from book where id = $1", &[&id])
    {
        Ok(_) => Status::Ok,
        Err(_) => Status::NotModified,
    };

    util::build_simple_response(status)
}

#[delete("/delete/by/<id>")]
pub fn delete_by_author(id: i32, mut db_conn: DBConnection) -> Json<Value> {
    let status = match db_conn
        .client
        .execute("delete from book where authorId = $1", &[&id])
    {
        Ok(_) => Status::Ok,
        Err(_) => Status::NotModified,
    };

    util::build_simple_response(status)
}
