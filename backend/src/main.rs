pub mod projects;
use actix_cors::Cors;
use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use tokio_postgres::NoTls;

async fn get_projects() -> Result<(), postgres::Error> {
    let (client, connection) =
    tokio_postgres::connect("postgresql://postgres:1337asdf@localhost:5432/postgres", NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });
        
    for row in client.query("SELECT id, name FROM projects", &[]).await? {
        let name: &str = row.get(1);
        
        println!("found project: {}", name);
    };

    Ok(())
}

#[get("/hello")]
async fn hello() -> impl Responder {
    let result = get_projects().await;
    match result {
        Ok(()) => println!("Ok"),
        Err(e) => println!("{}", e)
    }

    return HttpResponse::Ok().body("Hello world!");
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::default()
        .allow_any_origin()
        .send_wildcard();

        App::new()
            .wrap(cors)
            .service(hello)
            .service(projects::project_overview)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}