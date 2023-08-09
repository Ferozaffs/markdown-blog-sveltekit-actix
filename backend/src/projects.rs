use actix_web::{get, web, Responder, Result, HttpRequest, HttpResponse};
use actix_web::http::header::ContentType;
use serde::Serialize;
use crate::database::Database;

#[derive(Serialize, Debug)]
struct ProjectSummary {
    name: String,
    image: String,
    status: i32,
}

#[derive(Serialize, Debug)]
struct ProjectCategory {
    title: String,
    description: String,
    children: Vec<ProjectSummary>
}

#[derive(Serialize, Debug)]
pub struct ProjectOverview {
    categories: Vec<ProjectCategory>
}

pub fn create_test_overview() -> ProjectOverview {
    let ps = ProjectSummary{
        name: String::from("TestName"),
        image: String::from("TestImage.jpg"),
        status: 0
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
    let mut overview = ProjectOverview{
        categories: Vec::new()
    };
        
    
    let result = db.get_project_categories().await;
    for row in result.unwrap() {
        let mut category = ProjectCategory {
            title: row.get(0),
            description: row.get(1),
            children: Vec::new()
        };

        let result = db.get_projects_from_category(&category.title).await;
        for row in result.unwrap() {
            let project = ProjectSummary {
                name: row.get(0),
                image: row.get(1),
                status: row.get(2)
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
    let result = db.get_project_content(&id).await;
    let content: String = result.unwrap().get(0).unwrap().get(0);

    Ok(HttpResponse::Ok()
        .content_type(ContentType::plaintext())
        .insert_header(("X-Hdr", "sample"))
        .body(content))
}

#[get("/projectsummary/{id}")]
async fn project_summary(db: web::Data<Database>, req: HttpRequest) -> Result<impl Responder> {
    let id: String = req.match_info().query("id").parse().unwrap();
    let result = db.get_project_summary(&id).await;
    let row = result.unwrap();
    let data = row.get(0).unwrap();
    let summary = ProjectSummary {
        name: data.get(0),
        image: data.get(1),
        status: data.get(2)
    };

    Ok(web::Json(summary))
}
 


