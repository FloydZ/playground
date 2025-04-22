#![allow(unused_imports)]
#![allow(non_snake_case)]

use crate::api_error::ApiError;
use crate::db;
use crate::schema::cluster_wckey_table;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::{NaiveDateTime, Utc};

#[derive(Debug, Serialize, Deserialize, Queryable, QueryableByName)]
#[table_name="cluster_wckey_table"]
pub struct ClusterWckey {
    /*
+---------------+---------------------+------+-----+---------+----------------+
| Field         | Type                | Null | Key | Default | Extra          |
+---------------+---------------------+------+-----+---------+----------------+
| creation_time | bigint(20) unsigned | NO   |     | NULL    |                |
| mod_time      | bigint(20) unsigned | NO   |     | 0       |                |
| deleted       | tinyint(4)          | NO   |     | 0       |                |
| is_def        | tinyint(4)          | NO   |     | 0       |                |
| id_wckey      | int(10) unsigned    | NO   | PRI | NULL    | auto_increment |
| wckey_name    | tinytext            | NO   | MUL | NULL    |                |
| user          | tinytext            | NO   |     | NULL    |                |
+---------------+---------------------+------+-----+---------+----------------+
    */
    pub creation_time: u64,
    pub mod_time: u64,
    pub deleted: i8,

    pub is_def: i8,
    pub id_wckey: u32,
    pub wckey_name: String,
    pub user: String
}

impl ClusterWckey {
    pub fn find_all() -> Result<Vec<Self>, ApiError> {
        let conn = db::connection()?;

        let a = cluster_wckey_table::table
            .load::<ClusterWckey>(&conn)?;

        Ok(a)
    }

    pub fn find(id_wckey: u32) -> Result<Self, ApiError> {
        let conn = db::connection()?;

        let a = cluster_wckey_table::table
            .filter(cluster_wckey_table::id_wckey.eq(id_wckey))
            .first(&conn)?;

        Ok(a)
    }
}