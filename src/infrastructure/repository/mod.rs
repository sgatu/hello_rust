mod mysql_sessionrepository;

use mysql::Pool;
pub use mysql_sessionrepository::MysqlSessionRepository;
use rocket::{Build, Rocket};

use crate::domain::repository::SessionRepository;

pub fn manage(rocket: Rocket<Build>, pool: Pool) -> Rocket<Build> {
    rocket.manage(Box::new(MysqlSessionRepository::new(pool)) as Box<dyn SessionRepository>)
}
