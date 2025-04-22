#![allow(unused_imports)]
#![allow(non_snake_case)]


use crate::api_error::ApiError;
use crate::db;
use crate::schema::cluster_assoc_usage_hour_table;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::{NaiveDateTime, Utc};

#[derive(Debug, Serialize, Deserialize, Queryable, QueryableByName)]
#[table_name="cluster_assoc_usage_hour_table"]
pub struct ClusterAssocUsageHour {
    /*
    +---------------+---------------------+------+-----+---------+-------+
    | Field         | Type                | Null | Key | Default | Extra |
    +---------------+---------------------+------+-----+---------+-------+
    | creation_time | bigint(20) unsigned | NO   |     | NULL    |       |
    | mod_time      | bigint(20) unsigned | NO   |     | 0       |       |
    | deleted       | tinyint(4)          | NO   |     | 0       |       |
    | id            | int(10) unsigned    | NO   | PRI | NULL    |       |
    | id_tres       | int(11)             | NO   | PRI | 1       |       |
    | time_start    | bigint(20) unsigned | NO   | PRI | NULL    |       |
    | alloc_secs    | bigint(20) unsigned | NO   |     | 0       |       |
    +---------------+---------------------+------+-----+---------+-------+
    */

    pub creation_time: i64,
    pub mod_time: i64,
    pub deleted: i8,
    pub id: i32,
    pub id_tres: i32,
    pub time_start: i64,
    pub alloc_secs: i64,
}

impl ClusterAssocUsageHour {
    pub fn find_all() -> Result<Vec<Self>, ApiError> {
        let conn = db::connection()?;

        let a = cluster_assoc_usage_hour_table::table
            .load::<ClusterAssocUsageHour>(&conn)?;

        Ok(a)
    }

    pub fn findById(id: i32) -> Result<Self, ApiError> {
        let conn = db::connection()?;

        let a = cluster_assoc_usage_hour_table::table
            .filter(cluster_assoc_usage_hour_table::id.eq(id))
            .first(&conn)?;

        Ok(a)
    }

    pub fn findByTimeStarts(time_start: i64) -> Result<Self, ApiError> {
        let conn = db::connection()?;

        let a = cluster_assoc_usage_hour_table::table
            .filter(cluster_assoc_usage_hour_table::time_start.eq(time_start))
            .first(&conn)?;

        Ok(a)
    }

    pub fn findByIdTres(id_tres: i32) -> Result<Self, ApiError> {
        let conn = db::connection()?;

        let a = cluster_assoc_usage_hour_table::table
            .filter(cluster_assoc_usage_hour_table::id_tres.eq(id_tres))
            .first(&conn)?;

        Ok(a)
    }
}