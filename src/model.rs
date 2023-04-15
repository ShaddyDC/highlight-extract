use chrono::NaiveDateTime;

#[derive(Debug, PartialEq, Eq)]
pub struct Metadata {
    pub title: String,
    pub author: String,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Highlight {
    pub timestamp: NaiveDateTime,
    pub page: u32,
    pub highlight: String,
    pub note: Option<String>,
}
#[derive(Debug, PartialEq, Eq)]
pub enum Section {
    HL(Highlight),
    Chapter(String),
}

#[derive(Debug, PartialEq, Eq)]
pub struct BooxFile {
    pub metadata: Metadata,
    pub sections: Vec<Section>,
}
