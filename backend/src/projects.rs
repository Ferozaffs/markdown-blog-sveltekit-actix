use crate::database::Database;
use actix_web::{get, web, HttpRequest, HttpResponse, Responder, Result};

pub fn create_test_overview() -> shared::ProjectOverview {
    let ps = shared::ProjectSummary {
        id: uuid::Uuid::new_v4(),
        title: String::from("TestName"),
        image: String::from("TestImage.jpg"),
        status: 0,
        category: uuid::Uuid::new_v4(),
    };

    let pc = shared::ProjectCategory {
        id: uuid::Uuid::new_v4(),
        title: String::from("TestTitle"),
        description: String::from("TestDescription"),
        children: Vec::from([ps]),
    };

    let po = shared::ProjectOverview {
        categories: Vec::from([pc]),
    };

    return po;
}

#[get("/projectoverview")]
async fn project_overview(db: web::Data<Database>) -> Result<impl Responder> {
    let mut overview = shared::ProjectOverview {
        categories: Vec::new(),
    };

    let result = db.get_project_categories().await;
    for row in result.unwrap() {
        let mut category = shared::ProjectCategory {
            id: row.get(0),
            title: row.get(1),
            description: row.get(2),
            children: Vec::new(),
        };

        let result = db.get_projects_from_category(&category.title).await;
        for row in result.unwrap() {
            let s: i32 = row.get(3);
            let project = shared::ProjectSummary {
                id: row.get(0),
                title: row.get(1),
                image: row.get(2),
                status: s.try_into().unwrap(),
                category: row.get(4),
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
