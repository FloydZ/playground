#![allow(unused_imports)]
#![allow(non_snake_case)]

use crate::api_error::ApiError;
use crate::db;
use crate::schema::cluster_usage_hour_table;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::{NaiveDateTime, Utc};

#[derive(Debug, Serialize, Deserialize, Queryable, QueryableByName)]
#[table_name="cluster_usage_hour_table"]
pub struct ClusterUsageHour {
    /*
    +---------------+---------------------+------+-----+---------+-------+
    | Field         | Type                | Null | Key | Default | Extra |
    +---------------+---------------------+------+-----+---------+-------+
    | creation_time | bigint(20) unsigned | NO   |     | NULL    |       |
    | mod_time      | bigint(20) unsigned | NO   |     | 0       |       |
    | deleted       | tinyint(4)          | NO   |     | 0       |       |
    | id_tres       | int(11)             | NO   | PRI | NULL    |       |
    | time_start    | bigint(20) unsigned | NO   | PRI | NULL    |       |
    | count         | bigint(20) unsigned | NO   |     | 0       |       |
    | alloc_secs    | bigint(20) unsigned | NO   |     | 0       |       |
    | down_secs     | bigint(20) unsigned | NO   |     | 0       |       |
    | pdown_secs    | bigint(20) unsigned | NO   |     | 0       |       |
    | idle_secs     | bigint(20) unsigned | NO   |     | 0       |       |
    | resv_secs     | bigint(20) unsigned | NO   |     | 0       |       |
    | over_secs     | bigint(20) unsigned | NO   |     | 0       |       |
    +---------------+---------------------+------+-----+---------+-------+
     */

    pub creation_time: u64,
    pub mod_time: u64,
    pub deleted: i8,
    pub id_tres: i32,
    pub time_start: u64,
    pub count: u64,
    pub alloc_secs: u64,
    pub down_secs: u64,
    pub pdown_secs: u64,
    pub idle_secs: u64,
    pub resv_secs: u64,
    pub over_secs: u64,
}

impl ClusterUsageHour {
    pub fn find_all() -> Result<Vec<Self>, ApiError> {
        let conn = db::connection()?;

        let a = cluster_usage_hour_table::table
            .load::<ClusterUsageHour>(&conn)?;

        Ok(a)
    }

    pub fn findById(id_tres: i32) -> Result<Self, ApiError> {
        let conn = db::connection()?;

        let a = cluster_usage_hour_table::table
            .filter(cluster_usage_hour_table::id_tres.eq(id_tres))
            .first(&conn)?;

        Ok(a)
    }

    pub fn findByTimeStarts(time_start: u64) -> Result<Self, ApiError> {
        let conn = db::connection()?;

        let a = cluster_usage_hour_table::table
            .filter(cluster_usage_hour_table::time_start.eq(time_start))
            .first(&conn)?;

        Ok(a)
    }
}