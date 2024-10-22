use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone)]
pub struct MetaData {
    pub id: uuid::Uuid,
    pub title: String,
    pub description: String,
    pub post_type: usize,
    pub project: uuid::Uuid,
    pub tags: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PostSummary {
    pub id: Uuid,
    pub title: String,
    pub image: String,
    pub date: String,
    pub description: String,
    pub tags: String,
    pub project_id: Uuid,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectSummary {
    pub id: Uuid,
    pub name: String,
    pub image: String,
    pub status: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectCategory {
    pub title: String,
    pub description: String,
    pub children: Vec<ProjectSummary>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectOverview {
    pub categories: Vec<ProjectCategory>,
}
