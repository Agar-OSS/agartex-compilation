use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Project {
    pub project_id: i32,
    pub main_document_id: i32,
    pub owner: i32,
    pub name: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Document {
    pub document_id: i32,
    pub project_id: i32,
    pub name: String,
}
