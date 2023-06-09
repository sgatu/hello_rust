mod mysql_sessionrepository;
mod mysql_connection;

use mysql::Pool;
pub use mysql_connection::MysqlConnection;
pub use mysql_sessionrepository::MysqlSessionRepository;
use rocket::{Rocket, Build};

pub fn manage(rocket: Rocket<Build>, pool: Pool) -> Rocket<Build> {
    rocket.manage(MysqlSessionRepository::new(pool))
}