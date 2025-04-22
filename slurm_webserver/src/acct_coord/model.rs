#![allow(unused_imports)]
#![allow(non_snake_case)]

use crate::api_error::ApiError;
use crate::db;
use crate::schema::acct_coord_table;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::{NaiveDateTime, Utc};

#[derive(Debug, Serialize, Deserialize, Queryable, QueryableByName)]
#[table_name="acct_coord_table"]
pub struct AcctCoord {
    /*
    +---------------+---------------------+------+-----+---------+-------+
    | Field         | Type                | Null | Key | Default | Extra |
    +---------------+---------------------+------+-----+---------+-------+
    | creation_time | bigint(20) unsigned | NO   |     | NULL    |       |
    | mod_time      | bigint(20) unsigned | NO   |     | 0       |       |
    | deleted       | tinyint(4)          | YES  |     | 0       |       |
    | acct          | tinytext            | NO   | PRI | NULL    |       |
    | user          | tinytext            | NO   | PRI | NULL    |       |
    +---------------+---------------------+------+-----+---------+-------+
    */

    pub creation_time: i64,
    pub mod_time: i64,

    pub deleted: Option<i8>,
    pub acct: String,
    pub user: String
}

impl AcctCoord {
    pub fn find_all() -> Result<Vec<Self>, ApiError> {
        let conn = db::connection()?;

        let a = acct_coord_table::table
            .load::<AcctCoord>(&conn)?;

        Ok(a)
    }

    pub fn findByAcct(acct: String) -> Result<Self, ApiError> {
        let conn = db::connection()?;

        let a = acct_coord_table::table
            .filter(acct_coord_table::acct.eq(acct))
            .first(&conn)?;

        Ok(a)
    }

    pub fn findByUser(user: String) -> Result<Self, ApiError> {
        let conn = db::connection()?;

        let a = acct_coord_table::table
            .filter(acct_coord_table::user.eq(user))
            .first(&conn)?;

        Ok(a)
    }

}