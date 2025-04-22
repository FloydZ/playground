#![allow(unused_imports)]
#![allow(non_snake_case)]

use crate::api_error::ApiError;
use crate::db;
use crate::schema::cluster_job_table;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::{NaiveDateTime, Utc};

#[derive(Debug, Serialize, Deserialize, Queryable, QueryableByName)]
#[table_name="cluster_job_table"]
pub struct ClusterJob {
    /*
    +--------------------+---------------------+------+-----+------------+----------------+
    | Field              | Type                | Null | Key | Default    | Extra          |
    +--------------------+---------------------+------+-----+------------+----------------+
    | job_db_inx         | bigint(20) unsigned | NO   | PRI | NULL       | auto_increment |
    | mod_time           | bigint(20) unsigned | NO   |     | 0          |                |
    | deleted            | tinyint(4)          | NO   |     | 0          |                |
    | account            | tinytext            | YES  |     | NULL       |                |
    | admin_comment      | text                | YES  |     | NULL       |                |
    | array_task_str     | text                | YES  |     | NULL       |                |
    | array_max_tasks    | int(10) unsigned    | NO   |     | 0          |                |
    | array_task_pending | int(10) unsigned    | NO   |     | 0          |                |
    | cpus_req           | int(10) unsigned    | NO   |     | NULL       |                |
    | derived_ec         | int(10) unsigned    | NO   |     | 0          |                |
    | derived_es         | text                | YES  |     | NULL       |                |
    | exit_code          | int(10) unsigned    | NO   |     | 0          |                |
    | job_name           | tinytext            | NO   |     | NULL       |                |
    | id_assoc           | int(10) unsigned    | NO   | MUL | NULL       |                |
    | id_array_job       | int(10) unsigned    | NO   | MUL | 0          |                |
    | id_array_task      | int(10) unsigned    | NO   |     | 4294967294 |                |
    | id_block           | tinytext            | YES  |     | NULL       |                |
    | id_job             | int(10) unsigned    | NO   | MUL | NULL       |                |
    | id_qos             | int(10) unsigned    | NO   | MUL | 0          |                |
    | id_resv            | int(10) unsigned    | NO   | MUL | NULL       |                |
    | id_wckey           | int(10) unsigned    | NO   | MUL | NULL       |                |
    | id_user            | int(10) unsigned    | NO   | MUL | NULL       |                |
    | id_group           | int(10) unsigned    | NO   |     | NULL       |                |
    | pack_job_id        | int(10) unsigned    | NO   | MUL | NULL       |                |
    | pack_job_offset    | int(10) unsigned    | NO   |     | NULL       |                |
    | kill_requid        | int(11)             | NO   |     | -1         |                |
    | mcs_label          | tinytext            | YES  |     | NULL       |                |
    | mem_req            | bigint(20) unsigned | NO   |     | 0          |                |
    | nodelist           | text                | YES  |     | NULL       |                |
    | nodes_alloc        | int(10) unsigned    | NO   | MUL | NULL       |                |
    | node_inx           | text                | YES  |     | NULL       |                |
    | partition          | tinytext            | NO   |     | NULL       |                |
    | priority           | int(10) unsigned    | NO   |     | NULL       |                |
    | state              | int(10) unsigned    | NO   |     | NULL       |                |
    | timelimit          | int(10) unsigned    | NO   |     | 0          |                |
    | time_submit        | bigint(20) unsigned | NO   |     | 0          |                |
    | time_eligible      | bigint(20) unsigned | NO   | MUL | 0          |                |
    | time_start         | bigint(20) unsigned | NO   |     | 0          |                |
    | time_end           | bigint(20) unsigned | NO   | MUL | 0          |                |
    | time_suspended     | bigint(20) unsigned | NO   |     | 0          |                |
    | gres_req           | text                | NO   |     | NULL       |                |
    | gres_alloc         | text                | NO   |     | NULL       |                |
    | gres_used          | text                | NO   |     | NULL       |                |
    | wckey              | tinytext            | NO   |     | NULL       |                |
    | work_dir           | text                | NO   |     | NULL       |                |
    | track_steps        | tinyint(4)          | NO   |     | NULL       |                |
    | tres_alloc         | text                | NO   |     | NULL       |                |
    | tres_req           | text                | NO   |     | NULL       |                |
    +--------------------+---------------------+------+-----+------------+----------------+
    */

    pub job_db_inx: u64,
    pub mod_time: u64,
    pub deleted: i8,
    pub account: Option<String>,
    pub admin_comment: Option<String>,
    pub array_task_str: Option<String>,
    pub array_max_tasks: u32,
    pub array_task_pending: u32,
    pub cpus_req: u32,
    pub derived_ec: u32,
    pub derived_es: Option<String>,
    pub exit_code: u32,
    pub job_name: String,
    pub id_assoc: u32,
    pub id_array_job: u32,
    pub id_array_task: u32,
    pub id_block: Option<String>,
    pub id_job: u32,
    pub id_qos: u32,
    pub id_resv: u32,
    pub id_wckey: u32,
    pub id_user: u32,
    pub id_group: u32,
    pub pack_job_id: u32,
    pub pack_job_offset: u32,
    pub kill_requid: i32,
    pub mcs_label: Option<String>,
    pub mem_req: u64,
    pub nodelist: Option<String>,
    pub nodes_alloc: u32,
    pub node_inx: Option<String>,

    pub partition: String,
    pub priority: u32,
    pub state: u32,

    pub timelimit: u32,
    pub time_submit: u64,
    pub time_eligible: u64,
    pub time_start: u64,
    pub time_end: u64,
    pub time_suspended: u64,

    pub gres_req: String,
    pub gres_alloc: String,
    pub gres_used: String,
    pub wckey: String,
    pub work_dir: String,
    pub track_steps: i8,
    pub tres_alloc: String,
    pub tres_req: String,
}

impl ClusterJob {
    pub fn find_all() -> Result<Vec<Self>, ApiError> {
        let conn = db::connection()?;

        let a = cluster_job_table::table
            .load::<ClusterJob>(&conn)?;

        Ok(a)
    }

    pub fn find(job_db_inx: u64) -> Result<Self, ApiError> {
        let conn = db::connection()?;

        let a = cluster_job_table::table
            .filter(cluster_job_table::job_db_inx.eq(job_db_inx))
            .first(&conn)?;

        Ok(a)
    }
}