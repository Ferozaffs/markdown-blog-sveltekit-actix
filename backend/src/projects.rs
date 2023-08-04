use actix_web::{get, web, Responder, Result};
use serde::Serialize;
use crate::database::Database;

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
async fn project_overview(db: web::Data<Database>) -> Result<impl Responder> {
    let result = db.get_projects().await;
    for row in result.unwrap() {
        let name: &str = row.get(1);
        
        println!("found project: {}", name);
    };

    Ok(web::Json(create_test_overview()))
}
 


