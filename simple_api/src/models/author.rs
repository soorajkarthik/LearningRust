use std::convert::From;

#[derive(Debug, Serialize, Deserialize)]
pub struct Author {
    pub id: i32,
    pub name: String,
    pub country: String,
}

impl From<postgres::Row> for Author {
    fn from(row: postgres::Row) -> Self {
        Author {
            id: row.get(0),
            name: row.get(1),
            country: row.get(2),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct NewAuthor {
    pub name: String,
    pub country: String,
}
