use crate::database::Database;
use actix_web::http::header::ContentType;
use actix_web::{get, web, HttpRequest, HttpResponse, Responder, Result};
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize, Debug)]
struct ProjectSummary {
    id: Uuid,
    name: String,
    image: String,
    status: i32,
}

#[derive(Serialize, Debug)]
struct ProjectCategory {
    title: String,
    description: String,
    children: Vec<ProjectSummary>,
}

#[derive(Serialize, Debug)]
pub struct ProjectOverview {
    categories: Vec<ProjectCategory>,
}

pub fn create_test_overview() -> ProjectOverview {
    let ps = ProjectSummary {
        id: Uuid::parse_str("1e33f43d-0193-460a-9128-bffb1d12e57c").unwrap(),
        name: String::from("TestName"),
        image: String::from("TestImage.jpg"),
        status: 0,
    };

    let pc = ProjectCategory {
        title: String::from("TestTitle"),
        description: String::from("TestDescription"),
        children: Vec::from([ps]),
    };

    let po = ProjectOverview {
        categories: Vec::from([pc]),
    };

    return po;
}

#[get("/projectoverview")]
async fn project_overview(db: web::Data<Database>) -> Result<impl Responder> {
    let mut overview = ProjectOverview {
        categories: Vec::new(),
    };

    let result = db.get_project_categories().await;
    for row in result.unwrap() {
        let mut category = ProjectCategory {
            title: row.get(0),
            description: row.get(1),
            children: Vec::new(),
        };

        let result = db.get_projects_from_category(&category.title).await;
        for row in result.unwrap() {
            let project = ProjectSummary {
                id: row.get(0),
                name: row.get(1),
                image: row.get(2),
                status: row.get(3),
            };

            category.children.push(project);
        }

        overview.categories.push(category);
    }

    Ok(web::Json(overview))
}

#[get("/projectcontent/{id}")]
async fn project_content(db: web::Data<Database>, req: HttpRequest) -> Result<impl Responder> {
    let id: String = req.match_info().query("id").parse().unwrap();
    match db.get_project_content(&id).await {
        Ok(content) => Ok(HttpResponse::Ok().body(content)),
        Err(_) => Ok(HttpResponse::BadRequest().body("Bad request")),
    }
}

#[get("/projectsummary/{id}")]
async fn project_summary(db: web::Data<Database>, req: HttpRequest) -> Result<impl Responder> {
    let id: String = req.match_info().query("id").parse().unwrap();
    match db.get_project_summary(&id).await {
        Ok(project) => Ok(web::Json(serde_json::to_value(&project).unwrap())),
        Err(_) => {
            let error_response = serde_json::json!({
                "error": "Bad request",
                "message": "Unable to fetch the post summary."
            });
            Ok(web::Json(error_response))
        }
    }
}
