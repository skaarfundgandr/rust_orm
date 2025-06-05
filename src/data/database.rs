use diesel_async:: {
    pooled_connection:: {
        deadpool:: { 
            Object, Pool, PoolError 
        },
        AsyncDieselConnectionManager
    },
    AsyncConnection, AsyncMysqlConnection
};
use dotenvy::dotenv;
use once_cell::sync::Lazy;
use std::env;

// Unpooled db connection
pub async fn connect_to_db_async() -> AsyncMysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("Invalid database url!!");

    let res = AsyncMysqlConnection::establish(&database_url).await;

    return match res {
        Ok(conn) => conn,
        Err(_) => panic!("Connnection to database failed"),
    };
}
// Pooled db connection
pub async fn connect_from_pool() -> Result<Object<AsyncMysqlConnection>, PoolError> {
    return DB_POOL.get().await;
}
// Create connection pool using lazy initialization
static DB_POOL: Lazy<Pool<AsyncMysqlConnection>> = Lazy::new(|| {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("Invalid database url!!");
    let config = AsyncDieselConnectionManager::<AsyncMysqlConnection>::new(database_url);

    return Pool::builder(config)
        .build()
        .expect("Failed to create pool");
});