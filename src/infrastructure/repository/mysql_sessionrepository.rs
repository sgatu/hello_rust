
use chrono::NaiveDateTime;
use mysql::Pool;

use mysql::params;
use mysql::prelude::Queryable;

use crate::domain::model::SessionData;
use crate::domain::model::User;
use crate::domain::repository::SessionRepository;
use crate::domain::repository::Result;
use crate::domain::repository::SessionError;

pub struct MysqlSessionRepository { 
    connection_pool: Pool
}
impl SessionRepository for MysqlSessionRepository {
    fn get_session(&self, token: &str) -> Result<SessionData> {
        if token.len() == 0 {
            return Result::Err(SessionError::Missing);
        }
        let connection = self.connection_pool.get_conn();
        match connection {
            Ok(mut conn) => {
                let stm = conn.prep("SELECT * from session where token = :token");
                match stm {
                    Ok(stm) => {
                        let result = conn.exec_iter(stm, params! {"token"=>token});
                        match result {
                            Ok(mut rows) =>{
                                let row = rows.next();
                                match row {
                                    Some(r) =>{
                                        let mut row = r.unwrap();
                                        let user_id: u64 = row.take("user_id").unwrap();
                                        let created = row.take::<NaiveDateTime,_>("created").unwrap();
                                        let expire = row.take::<NaiveDateTime,_>("expire").unwrap();
                                        Ok(SessionData {
                                            created: created,
                                            expire: expire,
                                            token: row.take("token").unwrap(),
                                            user: self.get_user_by_id(user_id).unwrap()
                                        })
                                    },
                                    _ => Err(SessionError::Invalid)
                                }
                               
                            },
                            _ => Err(SessionError::Invalid)
                        }
                    },
                    _ => Err(SessionError::Error)
                }
            },
            _ => Err(SessionError::Error)
        }

    }

}
impl MysqlSessionRepository {
    pub fn new(connection_pool: Pool) -> Self {
        MysqlSessionRepository { connection_pool: connection_pool }
    }
    fn get_user_by_id(&self, user_id: u64) -> Option<User>{
        let connection = self.connection_pool.get_conn();
        match connection {
            _ => Some(User::new("Stefan Gatu", "stefan.gatu@gmail.com", "abcABC"))
        }
    }
}