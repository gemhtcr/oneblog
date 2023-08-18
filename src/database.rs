use sea_orm::ConnectOptions;
use sea_orm::Database;
use sea_orm::DatabaseConnection;

const DATABASE_URL: &str = "mysql://root@127.0.0.1:3306/oneblog";
pub async fn init() -> Result<DatabaseConnection, sea_orm::DbErr> {
    // max connection is based on hardware core number
    let ret = std::thread::available_parallelism().map(|inner|inner.get()).unwrap_or(1);
	let mut opt = ConnectOptions::new(DATABASE_URL);
	opt.max_connections(ret as u32)
		//.min_connections(5)
		//.connect_timeout(std::time::Duration::from_secs(8))
		//.acquire_timeout(std::time::Duration::from_secs(8))
		//.idle_timeout(std::time::Duration::from_secs(8))
		//.max_lifetime(std::time::Duration::from_secs(8))
		.sqlx_logging(false);
		//.sqlx_logging_level(log::LevelFilter::Info)
		//.set_schema_search_path("my_schema"); // Setting default PostgreSQL schema
    tracing::info!(?opt);
	let db = Database::connect(opt).await?;

    Ok(db)
}
