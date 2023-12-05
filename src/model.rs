use chrono::NaiveDateTime;
use serde::Serialize;

#[derive(Debug, PartialEq, Eq, Serialize)]
pub struct Metadata {
    pub title: String,
    pub author: String,
}

#[derive(Debug, PartialEq, Eq, Serialize)]
pub struct Highlight {
    pub timestamp: NaiveDateTime,
    pub page: u32,
    pub highlight: String,
    pub note: Option<String>,
}

#[derive(Debug, PartialEq, Eq, Serialize)]
pub enum Section {
    HL(Highlight),
    Chapter(String),
}

#[derive(Debug, PartialEq, Eq, Serialize)]
pub struct BooxFile {
    pub metadata: Metadata,
    pub sections: Vec<Section>,
}
