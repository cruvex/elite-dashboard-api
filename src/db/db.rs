use crate::config::DatabaseConfig;
use deadpool_postgres::Runtime::Tokio1;
use deadpool_postgres::tokio_postgres::NoTls;
use deadpool_postgres::{Config, CreatePoolError, ManagerConfig, Pool, PoolConfig};

pub async fn init_db(database_config: &DatabaseConfig) -> Result<Pool, CreatePoolError> {
    let database_url = url::Url::parse(&database_config.url).expect("Failed to parse DATABASE_URL");

    let mut config = Config::new();
    config.user = Some(database_url.username().into());
    config.password = Some(database_url.password().expect("No Password found").into());
    config.dbname = Some(database_url.path().trim_start_matches('/').into());
    config.host = Some(database_url.host_str().expect("No Host found").into());
    config.port = database_url.port();

    config.manager = Some(ManagerConfig::default());

    let pool_cfg = PoolConfig::new(database_config.max_pool_size);
    config.pool = Some(pool_cfg);

    config.create_pool(Some(Tokio1), NoTls)
}
