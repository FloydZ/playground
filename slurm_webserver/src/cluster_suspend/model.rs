#![allow(unused_imports)]
#![allow(non_snake_case)]

use crate::api_error::ApiError;
use crate::db;
use crate::schema::cluster_suspend_table;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::{NaiveDateTime, Utc};

#[derive(Debug, Serialize, Deserialize, Queryable, QueryableByName)]
#[table_name="cluster_suspend_table"]
pub struct ClusterSuspend {
    /*
    +------------+---------------------+------+-----+---------+-------+
    | Field      | Type                | Null | Key | Default | Extra |
    +------------+---------------------+------+-----+---------+-------+
    | job_db_inx | bigint(20) unsigned | NO   | PRI | NULL    |       |
    | id_assoc   | int(11)             | NO   |     | NULL    |       |
    | time_start | bigint(20) unsigned | NO   | PRI | 0       |       |
    | time_end   | bigint(20) unsigned | NO   |     | 0       |       |
    +------------+---------------------+------+-----+---------+-------+
    */

    pub job_db_inx: u64,
    pub id_assoc: i32,
    pub time_start: u64,
    pub time_end: u64,
}

impl ClusterSuspend {
    pub fn find_all() -> Result<Vec<Self>, ApiError> {
        let conn = db::connection()?;

        let a = cluster_suspend_table::table
            .load::<ClusterSuspend>(&conn)?;

        Ok(a)
    }

    pub fn findByTimeStart(time_start: u64) -> Result<Self, ApiError> {
        let conn = db::connection()?;

        let a = cluster_suspend_table::table
            .filter(cluster_suspend_table::time_start.eq(time_start))
            .first(&conn)?;

        Ok(a)
    }

    pub fn findById(job_db_inx: u64) -> Result<Self, ApiError> {
        let conn = db::connection()?;

        let a = cluster_suspend_table::table
            .filter(cluster_suspend_table::job_db_inx.eq(job_db_inx))
            .first(&conn)?;

        Ok(a)
    }
}