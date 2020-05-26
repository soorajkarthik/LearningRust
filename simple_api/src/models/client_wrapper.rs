use postgres::{Client, NoTls};
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use rocket::Request;

pub struct DBConnection {
    pub client: postgres::Client,
}

impl DBConnection {
    fn is_valid_key(key: &str) -> bool {
        key == "valid key"
    }
}

#[derive(Debug)]
pub enum ConnectionError {
    WrongKey,
}

impl<'a, 'r> FromRequest<'a, 'r> for DBConnection {
    type Error = ConnectionError;

    fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        let api_key: String = request.headers().get("api-key").collect();
        if DBConnection::is_valid_key(&api_key) {
            Outcome::Success(DBConnection {
                client: Client::connect(
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
                .unwrap(),
            })
        } else {
            Outcome::Failure((Status::BadRequest, ConnectionError::WrongKey))
        }
    }
}
