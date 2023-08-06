use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use tokio_postgres::{Row, NoTls, Error};

type ConnectionPool = Pool<PostgresConnectionManager<NoTls>>;

pub struct Database {
    pool: ConnectionPool,
}

impl Database {
    pub async fn new() -> Self {
        let database_url = "postgresql://postgres:1337asdf@localhost:5432/postgres";
        
        let manager = PostgresConnectionManager::new_from_stringlike(database_url, NoTls)
            .expect("Failed to create connection manager");

        let pool: ConnectionPool = bb8::Pool::builder()
            .build(manager)
            .await
            .expect("Failed to create pool.");

        Database { pool }
    }

    pub async fn get_project_categories(&self) -> Result<Vec<Row>, Error> {
        let connection = self.pool.get()
        .await
        .expect("Failed to get a connection from the pool");

        connection.query("SELECT project_categories.category, project_categories.description FROM project_categories",&[])
        .await
    }

    pub async fn get_projects(&self) -> Result<Vec<Row>, Error> {
        let connection = self.pool.get()
        .await
        .expect("Failed to get a connection from the pool");

        connection.query("SELECT projects.name, projects.image, projects.status FROM projects",&[])
        .await
    }

    pub async fn get_projects_from_category(&self, category: &str) -> Result<Vec<Row>, Error> {
        let connection = self.pool.get()
        .await
        .expect("Failed to get a connection from the pool");

        let query = format!("SELECT projects.name, projects.image, projects.status
            FROM projects
            WHERE projects.category_id=(
                SELECT project_categories.id 
                FROM project_categories
                WHERE project_categories.category='{}')",
        category);

        connection.query(&query.to_string() ,&[])
        .await
    }
}