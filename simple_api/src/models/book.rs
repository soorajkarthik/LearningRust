#[derive(Debug, Serialize)]
pub struct Book {
    pub id: i32,
    pub title: String,
    #[serde(rename = "authorId")]
    pub author_id: i32,
    pub published: bool,
}

impl From<postgres::Row> for Book {
    fn from(row: postgres::Row) -> Self {
        Book {
            id: row.get(0),
            title: row.get(1),
            author_id: row.get(2),
            published: row.get(3),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct NewBook {
    pub title: String,
    #[serde(rename = "authorId")]
    pub author_id: i32,
    pub published: bool,
}
