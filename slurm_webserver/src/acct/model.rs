#![allow(unused_imports)]
#![allow(non_snake_case)]

use crate::api_error::ApiError;
use crate::db;
use crate::schema::acct_table;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::{NaiveDateTime, Utc};

#[derive(Debug, Serialize, Deserialize, Queryable, QueryableByName)]
#[table_name="acct_table"]
pub struct Acct {
    /*
    +---------------+---------------------+------+-----+---------+-------+
    | Field         | Type                | Null | Key | Default | Extra |
    +---------------+---------------------+------+-----+---------+-------+
    | creation_time | bigint(20) unsigned | NO   |     | NULL    |       |
    | mod_time      | bigint(20) unsigned | NO   |     | 0       |       |
    | deleted       | tinyint(4)          | YES  |     | 0       |       |
    | name          | tinytext            | NO   | PRI | NULL    |       |
    | description   | text                | NO   |     | NULL    |       |
    | organization  | text                | NO   |     | NULL    |       |
    +---------------+---------------------+------+-----+---------+-------+
    */

    pub creation_time: i64,
    pub mod_time: i64,
    pub deleted: Option<i8>,
    pub name: String,
    pub description: String,
    pub organization: String,
}

impl Acct {
    pub fn find_all() -> Result<Vec<Self>, ApiError> {
        let conn = db::connection()?;

        let a = acct_table::table
            .load::<Acct>(&conn)?;

        Ok(a)
    }

    pub fn find(name: String) -> Result<Self, ApiError> {
        let conn = db::connection()?;

        let a = acct_table::table
            .filter(acct_table::name.eq(name))
            .first(&conn)?;

        Ok(a)
    }
}