use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use tokio_postgres::{Row, NoTls, Error};

type ConnectionPool = Pool<PostgresConnectionManager<NoTls>>;

pub struct Database {
    pool: ConnectionPool,
}

impl Database {
    pub async fn new() -> Self {
        let database_url = "postgresql://mdAdmin:1337asdf@db:5432/mdDatabase";
        
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

    pub async fn get_projects_from_category(&self, category: &str) -> Result<Vec<Row>, Error> {
        let connection = self.pool.get()
        .await
        .expect("Failed to get a connection from the pool");

        let query = format!("SELECT projects.id, projects.name, projects.image, projects.status
            FROM projects
            WHERE projects.category_id=(
                SELECT project_categories.id 
                FROM project_categories
                WHERE project_categories.category='{}')",
        category);

        connection.query(&query.to_string() ,&[])
        .await
    }

    pub async fn get_project_summary(&self, id: &str) -> Result<Vec<Row>, Error> {
        let connection = self.pool.get()
        .await
        .expect("Failed to get a connection from the pool");

        let query = format!("SELECT projects.id, projects.name, projects.image, projects.status
            FROM projects
            WHERE projects.id='{}'",
        id);

        connection.query(&query.to_string(), &[])
        .await
    }

    pub async fn get_project_content(&self, id: &str) -> Result<Vec<Row>, Error> {
        let connection = self.pool.get()
        .await
        .expect("Failed to get a connection from the pool");

        let query = format!("SELECT projects.content 
            FROM projects 
            WHERE projects.id='{}'",
        id);

        connection.query(&query.to_string(),&[])
        .await
    }

    pub async fn get_posts(&self, tags: Vec<&str>) -> Result<Vec<Row>, Error> {
        let connection = self.pool.get()
        .await
        .expect("Failed to get a connection from the pool");

        let mut query = format!("SELECT 
            posts.id, 
            posts.name, 
            posts.image, 
            TO_CHAR(posts.date, 'yyyy-mm-dd'), 
            posts.description,
            posts.tags,
            posts.project_id
            FROM posts");

        if tags.is_empty() == false {
            query = format!("{} WHERE", query);

            for (i, tag) in tags.iter().enumerate() {
                if i == 0 {
                    query = format!("{} posts.tags LIKE \'%{}%\'", query, tag);
                }
                else {
                    query = format!("{} OR posts.tags LIKE \'%{}%\'", query, tag);
                }
            }
        }

        query = format!("{} ORDER BY posts.date DESC", query);
           
        connection.query(&query.to_string(),&[])
        .await
    }

    pub async fn get_post_summary(&self, id: &str) -> Result<Vec<Row>, Error> {
        let connection = self.pool.get()
        .await
        .expect("Failed to get a connection from the pool");

        let query = format!("SELECT 
            posts.id, 
            posts.name, 
            posts.image, 
            TO_CHAR(posts.date, 'yyyy-mm-dd'), 
            posts.description,
            posts.tags,
            posts.project_id
            FROM posts
            WHERE posts.id='{}'",
        id);

        connection.query(&query.to_string(), &[])
        .await
    }

    pub async fn get_post_content(&self, id: &str) -> Result<Vec<Row>, Error> {
        let connection = self.pool.get()
        .await
        .expect("Failed to get a connection from the pool");

        let query = format!("SELECT posts.content 
            FROM posts 
            WHERE posts.id='{}'",
        id);

        connection.query(&query.to_string(),&[])
        .await
    }
}