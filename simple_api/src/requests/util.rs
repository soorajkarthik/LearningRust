use postgres::Row;
use rocket::http::Status;
use rocket_contrib::json::Json;
use serde::Serialize;
use serde_json::Value;

#[get("/")]
pub fn test_connection() -> &'static str {
    "Ping successful!"
}

pub fn build_response<T: Serialize>(status: Status, val: Option<T>) -> Json<Value> {
    Json(
        json!({
            "status": status.code,
            "message" : status.reason,
            "value": val,
        })
        .into(),
    )
}

pub fn build_simple_response(status: Status) -> Json<Value> {
    Json(
        json!({
            "status": status.code,
            "message" : status.reason,
        })
        .into(),
    )
}

pub fn rows_to_values<T: From<postgres::Row>>(rows: Vec<Row>) -> Vec<T> {
    let mut values = Vec::new();
    for row in rows {
        values.push(T::from(row));
    }
    values
}
