use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use dotenv_codegen::dotenv;

pub type PgPool = Pool<ConnectionManager<PgConnection>>;

const DATABASE_URL: &str = dotenv!("DATABASE_URL");

pub fn create_pool() -> PgPool {
    let manager = ConnectionManager::<PgConnection>::new(DATABASE_URL);
    Pool::builder()
        .build(manager)
        .expect("Failed to create pool")
}
