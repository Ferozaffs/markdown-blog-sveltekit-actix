use actix_web::{get, web, Responder, Result};
use serde::Serialize;

#[derive(Serialize)]
struct Project {
    name: String,
}

#[derive(Serialize)]
struct ProjectSummary {
    name: String,
    status: String,
}

#[derive(Serialize)]
struct ProjectCategory {
    title: String,
    description: String,
    children: Vec<ProjectSummary>
}

#[derive(Serialize)]
pub struct ProjectOverview {
    categories: Vec<ProjectCategory>
}

pub fn create_test_overview() -> ProjectOverview {
    let ps = ProjectSummary{
        name: String::from("TestName"),
        status: String::from("TestDesc")
    };

    let pc = ProjectCategory {
        title: String::from("TestTitle"),
        description: String::from("TestDescription"),
        children: Vec::from([ps])
    };

    let po = ProjectOverview {
        categories: Vec::from([pc])
    };

    return po;
}

#[get("/projectoverview")]
async fn project_overview() -> Result<impl Responder> {
    Ok(web::Json(create_test_overview()))
}


