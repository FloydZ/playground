#![allow(unused_imports)]
#![allow(non_snake_case)]

use crate::api_error::ApiError;
use crate::db;
use crate::schema::cluster_resv_table;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::{NaiveDateTime, Utc};

#[derive(Debug, Serialize, Deserialize, Queryable, QueryableByName)]
#[table_name="cluster_resv_table"]
pub struct ClusterResv {
    /*
    +-------------+----------------------+------+-----+---------+-------+
    | Field       | Type                 | Null | Key | Default | Extra |
    +-------------+----------------------+------+-----+---------+-------+
    | id_resv     | int(10) unsigned     | NO   | PRI | 0       |       |
    | deleted     | tinyint(4)           | NO   |     | 0       |       |
    | assoclist   | text                 | NO   |     | NULL    |       |
    | flags       | smallint(5) unsigned | NO   |     | 0       |       |
    | nodelist    | text                 | NO   |     | NULL    |       |
    | node_inx    | text                 | NO   |     | NULL    |       |
    | resv_name   | text                 | NO   |     | NULL    |       |
    | time_start  | bigint(20) unsigned  | NO   | PRI | 0       |       |
    | time_end    | bigint(20) unsigned  | NO   |     | 0       |       |
    | tres        | text                 | NO   |     | NULL    |       |
    | unused_wall | double unsigned      | NO   |     | 0       |       |
    +-------------+----------------------+------+-----+---------+-------+
    */

    pub id_resv: u32,
    pub deleted: i8,
    pub assoclist: String,
    pub flags: u16,
    pub nodelist: String,
    pub node_inx: String,
    pub resv_name: String,
    pub time_start: u64,
    pub time_end: u64,
    pub tres: String,
    pub unused_wall: f64,
}

impl ClusterResv {
    pub fn find_all() -> Result<Vec<Self>, ApiError> {
        let conn = db::connection()?;

        let a = cluster_resv_table::table
            .load::<ClusterResv>(&conn)?;

        Ok(a)
    }

    pub fn findById(id: u32) -> Result<Self, ApiError> {
        let conn = db::connection()?;

        let a = cluster_resv_table::table
            .filter(cluster_resv_table::id_resv.eq(id))
            .first(&conn)?;

        Ok(a)
    }

    pub fn findByTimeStart(time_start: u64) -> Result<Self, ApiError> {
        let conn = db::connection()?;

        let a = cluster_resv_table::table
            .filter(cluster_resv_table::time_start.eq(time_start))
            .first(&conn)?;

        Ok(a)
    }
}