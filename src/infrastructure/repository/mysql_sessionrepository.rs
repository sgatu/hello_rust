use chrono::NaiveDateTime;
use mysql::Pool;

use mysql::params;
use mysql::error::Error::MySqlError;
use mysql::prelude::Queryable;
use mysql_common::Row;

use crate::domain::model::SessionData;
use crate::domain::model::User;
use crate::domain::repository::ResultSession;
use crate::domain::repository::ResultUser;
use crate::domain::repository::SessionError;
use crate::domain::repository::SessionRepository;
use crate::domain::repository::UserRegistrationError;

pub struct MysqlSessionRepository {
    connection_pool: Pool,
}
impl SessionRepository for MysqlSessionRepository {
    fn get_session(&self, token: &str) -> ResultSession<SessionData> {
        if token.len() == 0 {
            return Result::Err(SessionError::Missing);
        }
        let mut connection = self
            .connection_pool
            .get_conn()
            .map_err(|_e| SessionError::Error)?;
        let stm = connection
            .prep("SELECT * from session where token = :token")
            .unwrap();
        let result = connection.exec_iter(stm, params! {"token"=>token});
        match result {
            Ok(mut rows) => {
                let row = rows
                    .next()
                    .ok_or(SessionError::Invalid)?
                    .map_err(|_e| SessionError::Invalid)?;
                self.session_from_row(row)
            }
            _ => Err(SessionError::Invalid),
        }
    }
    fn create_session(&self, email: &str, password: &str) -> ResultSession<SessionData> {
        if email.len() == 0 || password.len() == 0 {
            return Err(SessionError::Invalid);
        }
        let user = self.get_user_by_email(email)?;
        let password_verification = user.verify_password(password);
        match password_verification {
            Ok(s) if s => {
                let session = SessionData::new(user, 600);
                self.save(session)
            }
            _ => Err(SessionError::Invalid),
        }
    }
    fn save(&self, session: SessionData) -> ResultSession<SessionData> {
        let mut connection = self
            .connection_pool
            .get_conn()
            .map_err(|_e| SessionError::Error)?;
        let stm = connection
            .prep("INSERT into session VALUES(:user_id, :token, :created, :expire) ON DUPLICATE KEY UPDATE expire = :expire")
            .unwrap();
        connection.exec_drop(
            stm,
            params! {"user_id"=>session.user.id, "token"=>session.token.to_string(), "created"=>session.created, "expire" => session.expire},
        ).map_err(|_e| SessionError::Error)?;
        Ok(session)
    }

    fn register_user(&self, user: User) -> ResultUser<User> {
        let mut connection = self
            .connection_pool
            .get_conn()
            .map_err(|_e| UserRegistrationError::Error)?;
        let stm = connection
            .prep("INSERT into user VALUES(:id, :name, :email, :created, :password)")
            .unwrap();
        connection.exec_drop(
            stm,
            params! {"id"=>user.id, "name"=>user.name.to_string(), "created"=>user.created, "email"=>user.email.to_string(), "password"=>user.password.to_string()},
        ).map_err(|_e| {
            match _e {
                MySqlError(err) if err.message.contains("Duplicate entry") => UserRegistrationError::Existing,
                _ =>  UserRegistrationError::Error 
            }
        })?;
        Ok(user)
    }
}
impl MysqlSessionRepository {
    pub fn new(connection_pool: Pool) -> Self {
        MysqlSessionRepository {
            connection_pool: connection_pool,
        }
    }
    fn get_user_by_email(&self, email: &str) -> ResultSession<User> {
        let mut connection = self
            .connection_pool
            .get_conn()
            .map_err(|_e| SessionError::Error)?;
        let stm = connection
            .prep("SELECT * from user where email = :email")
            .unwrap();
        let result = connection.exec_iter(stm, params! {"email"=>email});
        match result {
            Ok(mut rows) => {
                let mut row = rows
                    .next()
                    .ok_or(SessionError::Invalid)?
                    .map_err(|_e| SessionError::Invalid)?;
                Self::user_from_row(row)
            }
            _ => Err(SessionError::Invalid),
        }
    }
    fn get_user_by_id(&self, user_id: u64) -> ResultSession<User> {
        let mut connection = self
            .connection_pool
            .get_conn()
            .map_err(|_e| SessionError::Error)?;
        let stm = connection
            .prep("SELECT * from user where id = :uid")
            .unwrap();
        let result = connection.exec_iter(stm, params! {"uid"=>user_id});
        match result {
            Ok(mut rows) => {
                let mut row = rows
                    .next()
                    .ok_or(SessionError::Invalid)?
                    .map_err(|_e| SessionError::Invalid)?;
                Self::user_from_row(row)
            }
            _ => Err(SessionError::Invalid),
        }
    }
    fn user_from_row(mut row: Row) -> ResultSession<User> {
        let created = row
            .take::<NaiveDateTime, _>("created")
            .ok_or_else(|| SessionError::Error)?;
        Ok(User::from(
            row.take("id").unwrap(),
            row.take::<String, _>("name").unwrap().as_str(),
            row.take::<String, _>("email").unwrap().as_str(),
            row.take::<String, _>("password").unwrap().as_str(),
            created,
        ))
    }
    fn session_from_row(&self, mut row: Row) -> ResultSession<SessionData> {
        let user_id: u64 = row.take("user_id").unwrap();

        let created = row
            .take::<NaiveDateTime, _>("created")
            .ok_or_else(|| SessionError::Error)?;
        let expire = row
            .take::<NaiveDateTime, _>("expire")
            .ok_or_else(|| SessionError::Error)?;
        let user = self.get_user_by_id(user_id)?;
        Ok(SessionData::from(
            row.take("token").unwrap(),
            user,
            expire,
            created,
        ))
    }
}
