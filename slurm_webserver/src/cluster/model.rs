#![allow(unused_imports)]
#![allow(non_snake_case)]

use crate::api_error::ApiError;
use crate::db;
use crate::schema::cluster_table;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::{NaiveDateTime, Utc};

#[derive(Debug, Serialize, Deserialize, Queryable, QueryableByName)]
#[table_name="cluster_table"]
pub struct Cluster {
    /*
    +------------------+----------------------+------+-----+---------+-------+
    | Field            | Type                 | Null | Key | Default | Extra |
    +------------------+----------------------+------+-----+---------+-------+
    | creation_time    | bigint(20) unsigned  | NO   |     | NULL    |       |
    | mod_time         | bigint(20) unsigned  | NO   |     | 0       |       |
    | deleted          | tinyint(4)           | YES  |     | 0       |       |
    | name             | tinytext             | NO   | PRI | NULL    |       |
    | control_host     | tinytext             | NO   |     | NULL    |       |
    | control_port     | int(10) unsigned     | NO   |     | 0       |       |
    | last_port        | int(10) unsigned     | NO   |     | 0       |       |
    | rpc_version      | smallint(5) unsigned | NO   |     | 0       |       |
    | classification   | smallint(5) unsigned | YES  |     | 0       |       |
    | dimensions       | smallint(5) unsigned | YES  |     | 1       |       |
    | plugin_id_select | smallint(5) unsigned | YES  |     | 0       |       |
    | flags            | int(10) unsigned     | YES  |     | 0       |       |
    | federation       | tinytext             | NO   |     | NULL    |       |
    | features         | text                 | NO   |     | NULL    |       |
    | fed_id           | int(10) unsigned     | NO   |     | 0       |       |
    | fed_state        | smallint(5) unsigned | NO   |     | NULL    |       |
    +------------------+----------------------+------+-----+---------+-------+
    */

    pub creation_time: u64,
    pub mod_time: u64,
    pub deleted: Option<i8>,
    pub name: String,
    pub control_host: String,
    pub control_port: u32,
    pub last_port: u32,
    pub rpc_version: u16,
    pub classification: Option<u16>,
    pub dimensions: Option<u16>,
    pub plugin_id_select: Option<u16>,
    pub flags: Option<u32>,
    pub federation: String,
    pub features: String,
    pub fed_id: u32,
    pub fed_state: u16,
}

impl Cluster {
    pub fn find_all() -> Result<Vec<Self>, ApiError> {
        let conn = db::connection()?;

        let a = cluster_table::table
            .load::<Cluster>(&conn)?;

        Ok(a)
    }

    pub fn find(name: String) -> Result<Self, ApiError> {
        let conn = db::connection()?;

        let a = cluster_table::table
            .filter(cluster_table::name.eq(name))
            .first(&conn)?;

        Ok(a)
    }

}