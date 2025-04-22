#![allow(unused_imports)]
#![allow(non_snake_case)]

use crate::api_error::ApiError;
use crate::db;
use crate::schema::cluster_assoc_table;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::{NaiveDateTime, Utc};

#[derive(Debug, Serialize, Deserialize, Queryable, QueryableByName)]
#[table_name="cluster_assoc_table"]
pub struct ClusterAssoc {
    /*
    +-------------------+---------------------+------+-----+---------+----------------+
    | Field             | Type                | Null | Key | Default | Extra          |
    +-------------------+---------------------+------+-----+---------+----------------+
    | creation_time     | bigint(20) unsigned | NO   |     | NULL    |                |
    | mod_time          | bigint(20) unsigned | NO   |     | 0       |                |
    | deleted           | tinyint(4)          | NO   |     | 0       |                |
    | is_def            | tinyint(4)          | NO   |     | 0       |                |
    | id_assoc          | int(10) unsigned    | NO   | PRI | NULL    | auto_increment |
    | user              | tinytext            | NO   | MUL | NULL    |                |
    | acct              | tinytext            | NO   | MUL | NULL    |                |
    | partition         | tinytext            | NO   |     | NULL    |                |
    | parent_acct       | tinytext            | NO   |     | NULL    |                |
    | lft               | int(11)             | NO   | MUL | NULL    |                |
    | rgt               | int(11)             | NO   |     | NULL    |                |
    | shares            | int(11)             | NO   |     | 1       |                |
    | max_jobs          | int(11)             | YES  |     | NULL    |                |
    | max_submit_jobs   | int(11)             | YES  |     | NULL    |                |
    | max_tres_pj       | text                | NO   |     | NULL    |                |
    | max_tres_pn       | text                | NO   |     | NULL    |                |
    | max_tres_mins_pj  | text                | NO   |     | NULL    |                |
    | max_tres_run_mins | text                | NO   |     | NULL    |                |
    | max_wall_pj       | int(11)             | YES  |     | NULL    |                |
    | grp_jobs          | int(11)             | YES  |     | NULL    |                |
    | grp_submit_jobs   | int(11)             | YES  |     | NULL    |                |
    | grp_tres          | text                | NO   |     | NULL    |                |
    | grp_tres_mins     | text                | NO   |     | NULL    |                |
    | grp_tres_run_mins | text                | NO   |     | NULL    |                |
    | grp_wall          | int(11)             | YES  |     | NULL    |                |
    | def_qos_id        | int(11)             | YES  |     | NULL    |                |
    | qos               | blob                | NO   |     | NULL    |                |
    | delta_qos         | blob                | NO   |     | NULL    |                |
    +-------------------+---------------------+------+-----+---------+----------------+
    */
    pub creation_time: i64,
    pub mod_time: i64,
    pub deleted: i8,

    pub is_def: i8,
    pub id_assoc: i32,
    pub user: String,
    pub acct: String,
    pub partition: String,
    pub parent_acct: String,
    pub lft: i32,
    pub rgt: i32,
    pub shares: i32,

    pub max_jobs: Option<i32>,
    pub max_submit_jobs: Option<i32>,
    pub max_tres_pj: String,
    pub max_tres_pn: String,
    pub max_tres_mins_pj: String,
    pub max_tres_run_mins: String,
    pub max_wall_pj: Option<i32>,

    pub grp_jobs: Option<i32>,
    pub grp_submit_jobs: Option<i32>,
    pub grp_tres: String,
    pub grp_tres_mins: String,
    pub grp_tres_run_mins: String,
    pub grp_wall: Option<i32>,
    pub def_qos_id: Option<i32>,
    pub qos: Vec<u8>,
    pub delta_qos: Vec<u8>,


}

impl ClusterAssoc {
    pub fn find_all() -> Result<Vec<Self>, ApiError> {
        let conn = db::connection()?;

        let a = cluster_assoc_table::table
            .load::<ClusterAssoc>(&conn)?;

        Ok(a)
    }

    pub fn find(id_assoc: i32) -> Result<Self, ApiError> {
        let conn = db::connection()?;

        let a = cluster_assoc_table::table
            .filter(cluster_assoc_table::id_assoc.eq(id_assoc))
            .first(&conn)?;

        Ok(a)
    }
}