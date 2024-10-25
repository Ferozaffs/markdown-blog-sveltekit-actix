use bb8::{Pool, PooledConnection};
use bb8_postgres::PostgresConnectionManager;
use chrono::{DateTime, Utc};
use rand::{distributions::Alphanumeric, Rng};
use shared::MetaData;
use tokio_postgres::{Error, NoTls, Row};

type ConnectionPool = Pool<PostgresConnectionManager<NoTls>>;

pub struct Database {
    pool: ConnectionPool,
}

impl Database {
    fn generate_api_key() -> String {
        let key: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(32)
            .map(char::from)
            .collect();
        key
    }

    pub async fn new() -> Self {
        let mut database_url = "postgresql://mdAdmin:1337asdf@db:5432/mdDatabase";
        if cfg!(debug_assertions) {
            database_url = "postgresql://mdAdmin:1337asdf@localhost:5432/mdDatabase";
        }

        let manager = PostgresConnectionManager::new_from_stringlike(database_url, NoTls)
            .expect("Failed to create connection manager");

        let pool: ConnectionPool = bb8::Pool::builder()
            .build(manager)
            .await
            .expect("Failed to create pool.");

        Database { pool }
    }

    pub async fn get_connection(&self) -> PooledConnection<PostgresConnectionManager<NoTls>> {
        self.pool
            .get()
            .await
            .expect("Failed to get a connection from the pool")
    }

    pub async fn get_project_categories(&self) -> Result<Vec<Row>, Error> {
        let connection = self.get_connection().await;

        connection.query("SELECT project_categories.category, project_categories.description FROM project_categories",&[])
        .await
    }

    pub async fn get_projects_from_category(&self, category: &str) -> Result<Vec<Row>, Error> {
        let connection = self.get_connection().await;

        let query = format!(
            "SELECT projects.id, projects.title, projects.image, projects.status
            FROM projects
            WHERE projects.category_id=(
                SELECT project_categories.id 
                FROM project_categories
                WHERE project_categories.category='{}')",
            category
        );

        connection.query(&query.to_string(), &[]).await
    }

    pub async fn get_project_summary(&self, id: &str) -> Result<shared::ProjectSummary, Error> {
        let connection = self.get_connection().await;

        let query = format!(
            "SELECT 
            projects.id, 
            projects.title, 
            projects.image, 
            projects.status
            FROM projects
            WHERE projects.id='{}'",
            id
        );

        match connection.query(&query.to_string(), &[]).await {
            Ok(row) => match row.get(0) {
                Some(data) => Ok(shared::ProjectSummary {
                    id: data.get(0),
                    title: data.get(1),
                    image: data.get(2),
                    status: data.get(3),
                }),
                None => Ok(shared::ProjectSummary::default()),
            },
            Err(e) => {
                println!("ERROR: {}", e.to_string());
                Err(e)
            }
        }
    }

    pub async fn get_project_content(&self, id: &str) -> Result<String, Error> {
        let connection = self.get_connection().await;

        let query = format!(
            "SELECT 
                projects.content 
                FROM projects
                WHERE projects.id='{}'",
            id
        );

        match connection.query(&query.to_string(), &[]).await {
            Ok(row) => match row.get(0) {
                Some(content) => Ok(content.get(0)),
                None => Ok(String::from("")),
            },
            Err(e) => {
                println!("ERROR: {}", e.to_string());
                Err(e)
            }
        }
    }

    pub async fn get_posts(&self, tags: Vec<&str>) -> Result<Vec<Row>, Error> {
        let connection = self.get_connection().await;

        let mut query = format!(
            "SELECT 
            posts.id, 
            posts.title, 
            posts.image, 
            TO_CHAR(posts.date, 'yyyy-mm-dd'), 
            posts.description,
            posts.tags,
            posts.project_id
            FROM posts"
        );

        if tags.is_empty() == false {
            query = format!("{} WHERE", query);

            for (i, tag) in tags.iter().enumerate() {
                if i == 0 {
                    query = format!("{} posts.tags LIKE \'%{}%\'", query, tag);
                } else {
                    query = format!("{} OR posts.tags LIKE \'%{}%\'", query, tag);
                }
            }
        }

        query = format!("{} ORDER BY posts.date DESC", query);

        connection.query(&query.to_string(), &[]).await
    }

    pub async fn get_post_summary(&self, id: &str) -> Result<shared::PostSummary, Error> {
        let connection = self.get_connection().await;

        let query = format!(
            "SELECT 
            posts.id, 
            posts.title, 
            posts.image, 
            TO_CHAR(posts.date, 'yyyy-mm-dd'), 
            posts.description,
            posts.tags,
            posts.project_id
            FROM posts
            WHERE posts.id='{}'",
            id
        );

        match connection.query(&query.to_string(), &[]).await {
            Ok(row) => match row.get(0) {
                Some(data) => Ok(shared::PostSummary {
                    id: data.get(0),
                    title: data.get(1),
                    image: data.get(2),
                    date: data.get(3),
                    description: data.get(4),
                    tags: data.get(5),
                    project_id: data.get(6),
                }),
                None => Ok(shared::PostSummary::default()),
            },
            Err(e) => {
                println!("ERROR: {}", e.to_string());
                Err(e)
            }
        }
    }

    pub async fn get_post_content(&self, id: &str) -> Result<String, Error> {
        let connection = self.get_connection().await;

        let query = format!(
            "SELECT posts.content 
            FROM posts 
            WHERE posts.id='{}'",
            id
        );

        match connection.query(&query.to_string(), &[]).await {
            Ok(row) => match row.get(0) {
                Some(content) => Ok(content.get(0)),
                None => Ok(String::from("")),
            },
            Err(_) => Ok(String::from("")),
        }
    }

    pub async fn save_post(
        &self,
        md: MetaData,
        markdown: &str,
        image_fingerprint: &str,
    ) -> Result<u64, Error> {
        let connection = self.get_connection().await;

        let query: String;
        if md.post_type == 1 {
            query = format!(
                "INSERT INTO projects (id, title, image, status, category_id, content)
                VALUES ($1, $2, $3, $4, $5, $6)"
            );

            let status = 0;
            let category = uuid::Uuid::nil();

            match connection
                .execute(
                    &query,
                    &[
                        &md.id,
                        &md.title,
                        &image_fingerprint,
                        &status,
                        &category,
                        &markdown,
                    ],
                )
                .await
            {
                Ok(v) => Ok(v),
                Err(e) => {
                    println!("ERROR: {}", e.to_string());
                    Err(e)
                }
            }
        } else {
            let mut tags = String::from("");
            for (i, tag) in md.tags.iter().enumerate() {
                if tag.len() > 0 {
                    if i == tags.len() - 1 {
                        tags.push_str(format!("{}", tag).as_str());
                    } else {
                        tags.push_str(format!("{},", tag).as_str());
                    }
                }
            }

            query = format!(
                "INSERT INTO posts (id, title, image, project_id, tags, content, date, description)
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8)"
            );

            let now: DateTime<Utc> = Utc::now();
            match connection
                .execute(
                    &query,
                    &[
                        &md.id,
                        &md.title,
                        &image_fingerprint,
                        &md.project,
                        &tags,
                        &markdown,
                        &now.naive_utc().date(),
                        &md.description,
                    ],
                )
                .await
            {
                Ok(v) => Ok(v),
                Err(e) => {
                    println!("ERROR: {}", e.to_string());
                    Err(e)
                }
            }
        }
    }

    pub async fn check_api_keys(&self) -> Result<bool, Error> {
        let connection = self.get_connection().await;

        let result = connection.query("SELECT * FROM keys", &[]).await?;
        Ok(!result.is_empty())
    }

    pub async fn create_api_key(&self) -> Result<String, Error> {
        let connection = self.get_connection().await;

        let key = Database::generate_api_key();

        connection
            .query("INSERT INTO keys (key) VALUES ($1)", &[&key])
            .await?;
        Ok(key)
    }

    pub async fn is_auth_valid(&self, key: &str) -> Result<bool, Error> {
        let connection = self.get_connection().await;

        let result = connection
            .query("SELECT * FROM keys WHERE keys.key=$1", &[&key])
            .await?;

        Ok(!result.is_empty())
    }
}
