use chrono::NaiveDateTime;

#[derive(Debug)]
pub struct Metadata {
    pub title: String,
    pub author: String,
}

#[derive(Debug)]
pub struct Note {
    pub timestamp: NaiveDateTime,
    pub page: u32,
    pub highlight: String,
    pub note: Option<String>,
}

#[derive(Debug)]
pub enum Section<'a> {
    N(Note),
    Chapter(&'a str),
}
