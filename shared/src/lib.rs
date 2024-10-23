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

impl Default for MetaData {
    fn default() -> Self {
        MetaData {
            id: uuid::Uuid::nil(),
            title: String::from(""),
            description: String::from(""),
            post_type: 0,
            project: uuid::Uuid::nil(),
            tags: vec![],
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct PostSummary {
    pub id: Uuid,
    pub title: String,
    pub image: String,
    pub date: String,
    pub description: String,
    pub tags: String,
    pub project_id: Uuid,
}

impl Default for PostSummary {
    fn default() -> Self {
        PostSummary {
            id: uuid::Uuid::nil(),
            title: String::from(""),
            image: String::from(""),
            date: String::from(""),
            description: String::from(""),
            tags: String::from(""),
            project_id: uuid::Uuid::nil(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct ProjectSummary {
    pub id: Uuid,
    pub name: String,
    pub image: String,
    pub status: i32,
}

impl Default for ProjectSummary {
    fn default() -> Self {
        ProjectSummary {
            id: uuid::Uuid::nil(),
            name: String::from(""),
            image: String::from(""),
            status: 0,
        }
    }
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
