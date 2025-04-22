use crate::api_error::ApiError;
use diesel::r2d2::ConnectionManager;
use diesel::mysql::MysqlConnection;
use lazy_static::lazy_static;
use r2d2;
use std::env;

type Pool = r2d2::Pool<ConnectionManager<MysqlConnection>>;
pub type DbConnection = r2d2::PooledConnection<ConnectionManager<MysqlConnection>>;

lazy_static! {
    static ref POOL: Pool = {
        let db_url = env::var("DATABASE_URL").unwrap();

        let manager = ConnectionManager::<MysqlConnection>::new(db_url);
        Pool::new(manager).expect("Failed to create db pool")
    };
}

///
/// so actually one would init here the migrations. But we dont want to do this, because the whole
/// DB will already be initialised by the slurmdbd
pub fn init() {
    info!("Initializing DB");

    lazy_static::initialize(&POOL);
}

pub fn connection() -> Result<DbConnection, ApiError> {
    POOL.get()
        .map_err(|e| ApiError::new(500, format!("Failed getting db connection: {}", e)))
}
