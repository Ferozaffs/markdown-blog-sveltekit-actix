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

    pub async fn get_projects(&self) -> Result<Vec<Row>, Error> {
        let connection = self.pool.get()
        .await
        .expect("Failed to get a connection from the pool");

        connection.query("SELECT id, name FROM projects", &[])
        .await
    }
}