#![allow(unused_imports)]
use crate::api_error::ApiError;
use crate::db;
use crate::schema::cluster_last_ran_table;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::{NaiveDateTime, Utc};

#[derive(Debug, Serialize, Deserialize, Queryable, QueryableByName)]
#[table_name="cluster_last_ran_table"]
pub struct ClusterLastRun {
    // you cannot belief this. Slurm implements time as a big int.
    pub hourly_rollup: u64,
    pub daily_rollup: u64,
    pub monthly_rollup: u64,

}

impl ClusterLastRun {
    pub fn find_all() -> Result<Vec<Self>, ApiError> {
        let conn = db::connection()?;

        let users = cluster_last_ran_table::table
            .load::<ClusterLastRun>(&conn)?;

        Ok(users)
    }

}

