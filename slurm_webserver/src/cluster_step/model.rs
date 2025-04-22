#![allow(unused_imports)]
#![allow(non_snake_case)]

use crate::api_error::ApiError;
use crate::db;
use crate::schema::cluster_step_table;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::{NaiveDateTime, Utc};

#[derive(Debug, Serialize, Deserialize, Queryable, QueryableByName)]
#[table_name="cluster_step_table"]
pub struct ClusterStep {
    /*
    +---------------------+----------------------+------+-----+------------+-------+
    | Field               | Type                 | Null | Key | Default    | Extra |
    +---------------------+----------------------+------+-----+------------+-------+
    | job_db_inx          | bigint(20) unsigned  | NO   | PRI | NULL       |       |
    | deleted             | tinyint(4)           | NO   |     | 0          |       |
    | exit_code           | int(11)              | NO   |     | 0          |       |
    | id_step             | int(11)              | NO   | PRI | NULL       |       |
    | kill_requid         | int(11)              | NO   |     | -1         |       |
    | nodelist            | text                 | NO   |     | NULL       |       |
    | nodes_alloc         | int(10) unsigned     | NO   |     | NULL       |       |
    | node_inx            | text                 | YES  |     | NULL       |       |
    | state               | smallint(5) unsigned | NO   |     | NULL       |       |
    | step_name           | text                 | NO   |     | NULL       |       |
    | task_cnt            | int(10) unsigned     | NO   |     | NULL       |       |
    | task_dist           | smallint(6)          | NO   |     | 0          |       |
    | time_start          | bigint(20) unsigned  | NO   |     | 0          |       |
    | time_end            | bigint(20) unsigned  | NO   |     | 0          |       |
    | time_suspended      | bigint(20) unsigned  | NO   |     | 0          |       |
    | user_sec            | int(10) unsigned     | NO   |     | 0          |       |
    | user_usec           | int(10) unsigned     | NO   |     | 0          |       |
    | sys_sec             | int(10) unsigned     | NO   |     | 0          |       |
    | sys_usec            | int(10) unsigned     | NO   |     | 0          |       |
    | max_pages           | int(10) unsigned     | NO   |     | 0          |       |
    | max_pages_task      | int(10) unsigned     | NO   |     | 0          |       |
    | max_pages_node      | int(10) unsigned     | NO   |     | 0          |       |
    | ave_pages           | double unsigned      | NO   |     | 0          |       |
    | max_rss             | bigint(20) unsigned  | NO   |     | 0          |       |
    | max_rss_task        | int(10) unsigned     | NO   |     | 0          |       |
    | max_rss_node        | int(10) unsigned     | NO   |     | 0          |       |
    | ave_rss             | double unsigned      | NO   |     | 0          |       |
    | max_vsize           | bigint(20) unsigned  | NO   |     | 0          |       |
    | max_vsize_task      | int(10) unsigned     | NO   |     | 0          |       |
    | max_vsize_node      | int(10) unsigned     | NO   |     | 0          |       |
    | ave_vsize           | double unsigned      | NO   |     | 0          |       |
    | min_cpu             | int(10) unsigned     | NO   |     | 4294967294 |       |
    | min_cpu_task        | int(10) unsigned     | NO   |     | 0          |       |
    | min_cpu_node        | int(10) unsigned     | NO   |     | 0          |       |
    | ave_cpu             | double unsigned      | NO   |     | 0          |       |
    | act_cpufreq         | double unsigned      | NO   |     | 0          |       |
    | consumed_energy     | bigint(20) unsigned  | NO   |     | 0          |       |
    | req_cpufreq_min     | int(10) unsigned     | NO   |     | 0          |       |
    | req_cpufreq         | int(10) unsigned     | NO   |     | 0          |       |
    | req_cpufreq_gov     | int(10) unsigned     | NO   |     | 0          |       |
    | max_disk_read       | double unsigned      | NO   |     | 0          |       |
    | max_disk_read_task  | int(10) unsigned     | NO   |     | 0          |       |
    | max_disk_read_node  | int(10) unsigned     | NO   |     | 0          |       |
    | ave_disk_read       | double unsigned      | NO   |     | 0          |       |
    | max_disk_write      | double unsigned      | NO   |     | 0          |       |
    | max_disk_write_task | int(10) unsigned     | NO   |     | 0          |       |
    | max_disk_write_node | int(10) unsigned     | NO   |     | 0          |       |
    | ave_disk_write      | double unsigned      | NO   |     | 0          |       |
    | tres_alloc          | text                 | NO   |     | NULL       |       |
    +---------------------+----------------------+------+-----+------------+-------+
    */

    pub job_db_inx: u64,
    pub deleted: i8,
    pub exit_code: i32,
    pub id_step: i32,
    pub kill_requid: i32,
    pub nodelist: String,
    pub nodes_alloc: i32,
    pub node_inx: Option<String>,
    pub state: u16,
    pub step_name: String,

    pub task_cnt: u32,
    pub task_dist: i16,

    pub time_start: u64,
    pub time_end: u64,
    pub time_suspend: u64,

    pub user_sec: u32,
    pub user_usec: u32,

    pub sys_sec: u32,
    pub sys_usec: u32,

    pub max_pages: u32,
    pub max_pages_task: u32,
    pub max_pages_node: u32,
    pub ave_pages: f64,

    pub max_rss: u64,
    pub max_rss_task: u32,
    pub max_rss_node: u32,
    pub ave_rss: f64,

    pub max_vsize: u64,
    pub max_visze_task: u32,
    pub max_vsize_node: u32,
    pub ave_vsize: f64,

    pub min_cpu: u32,
    pub min_cpu_task: u32,
    pub min_cou_node: u32,
    pub ave_cpu: f64,
    pub act_cpufreq: f64,

    pub consumed_energy: u64,

    pub req_cpufreq_min: u32,
    pub req_cpufreq: u32,
    pub req_cpufreq_gov: u32,

    pub max_disk_read: f64,
    pub max_disk_read_task: u32,
    pub max_disk_read_node: u32,
    pub ave_disk_read: f64,
    pub max_disk_write: f64,
    pub max_disk_write_task: u32,
    pub max_disk_write_node: u32,
    pub ave_disk_write: f64,

    pub res_alloc: String,
}

impl ClusterStep {
    pub fn find_all() -> Result<Vec<Self>, ApiError> {
        let conn = db::connection()?;

        let a = cluster_step_table::table
            .load::<ClusterStep>(&conn)?;

        Ok(a)
    }

    pub fn findIdDB(job_db_inx: u64) -> Result<Self, ApiError> {
        let conn = db::connection()?;

        let a = cluster_step_table::table
            .filter(cluster_step_table::job_db_inx.eq(job_db_inx))
            .first(&conn)?;

        Ok(a)
    }

    pub fn findIdStep(id_step: i32) -> Result<Self, ApiError> {
        let conn = db::connection()?;

        let a = cluster_step_table::table
            .filter(cluster_step_table::id_step.eq(id_step))
            .first(&conn)?;

        Ok(a)
    }

}