use serde::{Deserialize, Serialize};
use std::str::FromStr;
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

pub fn load_meta_data(text: &str) -> (MetaData, usize) {
    let mut meta_data = MetaData::default();

    let mut count = 0;
    for line in text.lines() {
        if !line.starts_with('@') && !line.is_empty() {
            break;
        }

        if let Some(data) = line.strip_prefix("@ID: ") {
            match uuid::Uuid::from_str(data.trim().to_string().as_str()) {
                Ok(v) => {
                    if v != uuid::Uuid::nil() {
                        meta_data.id = v
                    }
                }
                Err(_) => (),
            }
        } else if let Some(data) = line.strip_prefix("@TYPE: ") {
            match data.trim().to_string().parse::<usize>() {
                Ok(v) => meta_data.post_type = v,
                Err(_) => (),
            }
        } else if let Some(data) = line.strip_prefix("@PROJECT: ") {
            match uuid::Uuid::from_str(data.trim().to_string().as_str()) {
                Ok(v) => {
                    if meta_data.post_type == 0 {
                        meta_data.project = v
                    }
                }
                Err(_) => (),
            }
        } else if let Some(data) = line.strip_prefix("@TITLE: ") {
            meta_data.title = data.trim().to_string();
        } else if let Some(data) = line.strip_prefix("@DESCRIPTION: ") {
            meta_data.description = data.trim().to_string();
        } else if let Some(data) = line.strip_prefix("@TAGS: ") {
            meta_data.tags = data.trim().split(',').map(|s| s.to_string()).collect();
        }
        count += 1;
    }

    return (meta_data, count);
}

pub fn store_meta_data(text: &mut String, meta_data: MetaData) {
    let mut clean = false;
    if let Some(first_line) = text.lines().next() {
        if first_line.starts_with('@') {
            clean = true;
        }
    }

    let mut md_meta: String = "@META\n".to_string();
    md_meta.push_str(format!("@ID: {}\n", meta_data.id).as_str());
    md_meta.push_str(format!("@TITLE: {}\n", meta_data.title).as_str());
    md_meta.push_str(format!("@DESCRIPTION: {}\n", meta_data.description).as_str());
    md_meta.push_str(format!("@TYPE: {}\n", meta_data.post_type).as_str());
    md_meta.push_str(format!("@PROJECT: {}\n", meta_data.project).as_str());
    md_meta.push_str("@TAGS: ");

    let tags = &meta_data.tags;
    for (i, tag) in tags.iter().enumerate() {
        if tag.len() > 0 {
            if i == tags.len() - 1 {
                md_meta.push_str(format!("{}", tag).as_str());
            } else {
                md_meta.push_str(format!("{},", tag).as_str());
            }
        }
    }

    let mut cloned_text = text.clone();
    if clean == true {
        cloned_text = cloned_text
            .lines()
            .skip(8)
            .collect::<Vec<&str>>()
            .join("\n");
    }

    text.clear();
    text.push_str(&format!("{}\n\n{}", md_meta, cloned_text));
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
    pub title: String,
    pub image: String,
    pub status: i32,
}

impl Default for ProjectSummary {
    fn default() -> Self {
        ProjectSummary {
            id: uuid::Uuid::nil(),
            title: String::from(""),
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
