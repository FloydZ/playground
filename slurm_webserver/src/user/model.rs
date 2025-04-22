#![allow(unused_imports)]
use crate::api_error::ApiError;
use crate::db;
use crate::schema::user_table;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::{NaiveDateTime, Utc};

#[derive(Debug, Serialize, Deserialize, Queryable, QueryableByName)]
#[table_name="user_table"]
pub struct User {
    // you cannot belief this. Slurm implements time as a big int.
    pub creation_time: i64,
    pub mod_time: i64,

    pub deleted: i8,
    pub name: String,
    pub admin_level: i8
}

impl User {
    pub fn find_all() -> Result<Vec<Self>, ApiError> {
        let conn = db::connection()?;

        let users = user_table::table
            .load::<User>(&conn)?;

        Ok(users)
    }

    pub fn find(name: String) -> Result<Self, ApiError> {
        let conn = db::connection()?;

        let user = user_table::table
            .filter(user_table::name.eq(name))
            .first(&conn)?;

        Ok(user)
    }

    /*
    Ich will erhlich gesagt nur lesend auf den ganzen spaß zugreifen
    pub fn create(user: UserMessage) -> Result<Self, ApiError> {
        let conn = db::connection()?;

        let user = User::from(user);
        let user = diesel::insert_into(user::table)
            .values(user)
            .get_result(&conn)?;

        Ok(user)
    }

    pub fn update(id: i64, user: UserMessage) -> Result<Self, ApiError> {
        let conn = db::connection()?;

        let user = diesel::update(user::table)
            .filter(user::id.eq(id))
            .set(user)
            .get_result(&conn)?;

        Ok(user)
    }

    pub fn delete(id: i64) -> Result<usize, ApiError> {
        let conn = db::connection()?;

        let res = diesel::delete(
                user::table
                    .filter(user::id.eq(id))
            )
            .execute(&conn)?;

        Ok(res)
    }
    */
}

/*
impl From<UserMessage> for User {

    // hardcode the user 'root'

    fn from(user: UserMessage) -> Self {
        User {
            id: 0,
            creation_time: Utc::now().naive_utc(),
            mod_time: None,

            deleted: 0,
            name: "root".to_string(),
            admin_level: 3
        }
    }
}
*/