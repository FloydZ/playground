#![allow(unused_imports)]
#![allow(non_snake_case)]

use crate::api_error::ApiError;
use crate::db;
use crate::schema::cluster_event_table;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::{NaiveDateTime, Utc};

#[derive(Debug, Serialize, Deserialize, Queryable, QueryableByName)]
#[table_name="cluster_event_table"]
pub struct ClusterEvent {
    /*
    +---------------+----------------------+------+-----+------------+-------+
    | Field         | Type                 | Null | Key | Default    | Extra |
    +---------------+----------------------+------+-----+------------+-------+
    | time_start    | bigint(20) unsigned  | NO   | PRI | NULL       |       |
    | time_end      | bigint(20) unsigned  | NO   |     | 0          |       |
    | node_name     | tinytext             | NO   | PRI | NULL       |       |
    | cluster_nodes | text                 | NO   |     | NULL       |       |
    | reason        | tinytext             | NO   |     | NULL       |       |
    | reason_uid    | int(10) unsigned     | NO   |     | 4294967294 |       |
    | state         | smallint(5) unsigned | NO   |     | 0          |       |
    | tres          | text                 | NO   |     | NULL       |       |
    +---------------+----------------------+------+-----+------------+-------+
    */

    pub time_start: i64,
    pub time_end: i64,

    pub node_name: String,
    pub cluster_nodes: String,

    pub reason: String,
    pub reason_uid: i32,

    pub state: i8,
    pub tres: String,

}

impl ClusterEvent {
    pub fn find_all() -> Result<Vec<Self>, ApiError> {
        let conn = db::connection()?;

        let a = cluster_event_table::table
            .load::<ClusterEvent>(&conn)?;

        Ok(a)
    }

    pub fn findByTimeStart(time_start: i64) -> Result<Self, ApiError> {
        let conn = db::connection()?;

        let a = cluster_event_table::table
            .filter(cluster_event_table::time_start.eq(time_start))
            .first(&conn)?;

        Ok(a)
    }

    pub fn findByNodeName(node_name: String) -> Result<Self, ApiError> {
        let conn = db::connection()?;

        let a = cluster_event_table::table
            .filter(cluster_event_table::node_name.eq(node_name))
            .first(&conn)?;

        Ok(a)
    }
}