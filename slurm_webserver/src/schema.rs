
table! {
    acct_coord_table (acct, user){
        creation_time -> BigInt,
        mod_time -> BigInt,
        deleted -> Nullable<TinyInt>,
        acct -> Text,
        user -> Text,
    }
}

table! {
    acct_table (name) {
        creation_time -> BigInt,
        mod_time -> BigInt,
        deleted -> Nullable<TinyInt>,
        name -> Text,
        description -> Text,
        organization -> Text,
    }
}

table! {
    cluster_assoc_table (id_assoc) {
        creation_time -> BigInt,
        mod_time -> BigInt,
        deleted -> TinyInt,
        is_def -> TinyInt,
        id_assoc ->Integer,
        user -> Text,
        acct -> Text,
        partition -> Text,
        parent_acct -> Text,
        lft -> Integer,
        rgt -> Integer,
        shares -> Integer,

        max_jobs -> Nullable<Integer>,
        max_submit_jobs -> Nullable<Integer>,
        max_tres_pj -> Text,
        max_tres_pn -> Text,
        max_tres_mins_pj -> Text,
        max_tres_run_mins -> Text,
        max_wall_pj -> Nullable<Integer>,

        grp_jobs -> Nullable<Integer>,
        grp_submit_jobs -> Nullable<Integer>,
        grp_tres -> Text,
        grp_tres_mins -> Text,
        grp_tres_run_mins -> Text,
        grp_wall -> Nullable<Integer>,

        def_qos_id -> Nullable<Integer>,
        qos -> Blob,
        delta_qos -> Blob,
    }
}

table! {
    cluster_assoc_usage_day_table(id, id_tres, time_start){
        creation_time -> BigInt,
        mod_time -> BigInt,
        deleted -> TinyInt,
        id -> Integer,
        id_tres -> Integer,
        time_start ->BigInt,
        alloc_secs -> BigInt,
    }
}

table! {
    cluster_assoc_usage_hour_table(id, id_tres, time_start){
        creation_time -> BigInt,
        mod_time -> BigInt,
        deleted -> TinyInt,
        id -> Integer,
        id_tres -> Integer,
        time_start ->BigInt,
        alloc_secs -> BigInt,
    }
}

table! {
    cluster_assoc_usage_month_table(id, id_tres, time_start){
        creation_time -> BigInt,
        mod_time -> BigInt,
        deleted -> TinyInt,
        id -> Integer,
        id_tres -> Integer,
        time_start ->BigInt,
        alloc_secs -> BigInt,
    }
}

table! {
    cluster_event_table (time_start, node_name) {
        time_start -> BigInt,
        time_end -> BigInt,
        node_name -> Text,
        cluster_nodes -> Text,
        reason -> Text,
        reason_uid -> Integer,
        state -> TinyInt,
        tres -> Text,
    }
}

table! {
    cluster_job_table (job_db_inx) {
        job_db_inx -> Unsigned<BigInt>,
        mod_time -> Unsigned<BigInt>,
        deleted -> TinyInt,
        account -> Nullable<Text>,
        admin_comment -> Nullable<Text>,
        array_task_str -> Nullable<Text>,
        array_max_tasks -> Unsigned<Integer>,
        array_task_pending -> Unsigned<Integer>,
        cpus_req -> Unsigned<Integer>,
        derived_ec-> Unsigned<Integer>,
        derived_es -> Nullable<Text>,
        exit_code -> Unsigned<Integer>,
        job_name -> Text,

        id_assoc -> Unsigned<Integer>,
        id_array_job -> Unsigned<Integer>,
        id_array_task -> Unsigned<Integer>,
        id_block -> Nullable<Text>,
        id_job -> Unsigned<Integer>,
        id_qos -> Unsigned<Integer>,
        id_resv -> Unsigned<Integer>,
        id_wckey -> Unsigned<Integer>,
        id_user -> Unsigned<Integer>,
        id_group -> Unsigned<Integer>,

        pack_job_id -> Unsigned<Integer>,
        pack_job_offset -> Unsigned<Integer>,
        kill_requid -> Integer,
        mcs_label -> Nullable<Text>,
        mem_req -> Unsigned<BigInt>,
        nodelist -> Nullable<Text>,
        nodes_alloc -> Unsigned<Integer>,
        node_inx -> Nullable<Text>,
        partition -> Text,
        priority -> Unsigned<Integer>,
        state -> Unsigned<Integer>,

        timelimit -> Unsigned<Integer>,
        time_submit -> Unsigned<BigInt>,
        time_eligible -> Unsigned<BigInt>,
        time_start -> Unsigned<BigInt>,
        time_end -> Unsigned<BigInt>,
        time_suspended -> Unsigned<BigInt>,

        gres_req -> Text,
        gres_alloc -> Text,
        gres_used -> Text,
        wckey -> Text,
        work_dir -> Text,
        track_steps -> TinyInt,
        tres_alloc -> Text,
        tres_req -> Text,
    }
}

table! {
    cluster_last_ran_table (hourly_rollup, daily_rollup, monthly_rollup) {
        hourly_rollup -> Unsigned<BigInt>,
        daily_rollup -> Unsigned<BigInt>,
        monthly_rollup -> Unsigned<BigInt>,
    }
}

table! {
    cluster_resv_table (id_resv, time_start) {
        id_resv -> Unsigned<Integer>,
        deleted -> TinyInt,
        assoclist -> Text,
        flags -> Unsigned<SmallInt>,
        nodelist -> Text,
        node_inx -> Text,
        resv_name -> Text,
        time_start -> Unsigned<BigInt>,
        time_end -> Unsigned<BigInt>,
        tres -> Text,
        unused_wall -> Double, //-> Unsigned<Double>,
    }
}

table! {
    cluster_step_table (job_db_inx, id_step) {
        job_db_inx -> Unsigned<BigInt>,
        deleted -> TinyInt,
        exit_code -> Integer,
        id_step -> Integer,
        kill_requid -> Integer,
        nodelist -> Text,
        nodes_alloc -> Integer,
        node_inx -> Nullable<Text>,
        state -> Unsigned<SmallInt>,
        step_name -> Text,

        task_cnt -> Unsigned<Integer>,
        task_dist -> SmallInt,

        time_start -> Unsigned<BigInt>,
        time_end -> Unsigned<BigInt>,
        time_suspend -> Unsigned<BigInt>,

        user_sec -> Unsigned<Integer>,
        user_usec -> Unsigned<Integer>,

        sys_sec -> Unsigned<Integer>,
        sys_usec -> Unsigned<Integer>,

        max_pages -> Unsigned<Integer>,
        max_pages_task -> Unsigned<Integer>,
        max_pages_node -> Unsigned<Integer>,
        ave_pages -> Double, //Unsigned<Double>,

        max_rss -> Unsigned<BigInt>,
        max_rss_task -> Unsigned<Integer>,
        max_rss_node -> Unsigned<Integer>,
        ave_rss -> Double, //Unsigned<Double>,

        max_vsize -> Unsigned<BigInt>,
        max_visze_task -> Unsigned<Integer>,
        max_vsize_node-> Unsigned<Integer>,
        ave_vsize-> Double, //Unsigned<Double>,

        min_cpu -> Unsigned<Integer>,
        min_cpu_task -> Unsigned<Integer>,
        min_cou_node -> Unsigned<Integer>,
        ave_cpu -> Double, //Unsigned<Double>,
        act_cpufreq -> Double, //Unsigned<Double>,

        consumed_energy -> Unsigned<BigInt>,

        req_cpufreq_min -> Unsigned<Integer>,
        req_cpufreq -> Unsigned<Integer>,
        req_cpufreq_gov -> Unsigned<Integer>,

        max_disk_read -> Double, //Unsigned<Double>,
        max_disk_read_task -> Unsigned<Integer>,
        max_disk_read_node -> Unsigned<Integer>,
        ave_disk_read -> Double, //Unsigned<Double>,
        max_disk_write -> Double, //Unsigned<Double>,
        max_disk_write_task -> Unsigned<Integer>,
        max_disk_write_node -> Unsigned<Integer>,
        ave_disk_write -> Double, //Unsigned<Double>,

        res_alloc -> Text,
    }
}

table! {
    cluster_suspend_table (job_db_inx, time_start) {
        job_db_inx -> Unsigned<BigInt>,
        id_assoc -> Integer,
        time_start -> Unsigned<BigInt>,
        time_end -> Unsigned<BigInt>,
    }
}

table! {
    cluster_table (name) {
        creation_time -> Unsigned<BigInt>,
        mod_time ->  Unsigned<BigInt>,
        deleted -> Nullable<TinyInt>,
        name -> Text,
        control_host -> Text,
        control_port -> Unsigned<Integer>,
        last_port -> Unsigned<Integer>,
        rpc_version -> Unsigned<SmallInt>,
        classification -> Nullable<Unsigned<SmallInt>>,
        dimensions -> Nullable<Unsigned<SmallInt>>,
        plugin_id_select -> Nullable<Unsigned<SmallInt>>,
        flags -> Nullable<Unsigned<Integer>>,
        federation -> Text,
        features -> Text,
        fed_id -> Unsigned<Integer>,
        fed_state -> Unsigned<SmallInt>,
    }
}

table! {
    cluster_usage_day_table (id_tres, time_start){
        creation_time -> Unsigned<BigInt>,
        mod_time -> Unsigned<BigInt>,
        deleted -> TinyInt,
        id_tres -> Integer,
        time_start -> Unsigned<BigInt>,
        count -> Unsigned<BigInt>,
        alloc_secs -> Unsigned<BigInt>,
        down_secs -> Unsigned<BigInt>,
        pdown_secs -> Unsigned<BigInt>,
        idle_secs -> Unsigned<BigInt>,
        resv_secs -> Unsigned<BigInt>,
        over_secs -> Unsigned<BigInt>,
    }
}

table! {
    cluster_usage_hour_table (id_tres, time_start){
        creation_time -> Unsigned<BigInt>,
        mod_time -> Unsigned<BigInt>,
        deleted -> TinyInt,
        id_tres -> Integer,
        time_start -> Unsigned<BigInt>,
        count -> Unsigned<BigInt>,
        alloc_secs -> Unsigned<BigInt>,
        down_secs -> Unsigned<BigInt>,
        pdown_secs -> Unsigned<BigInt>,
        idle_secs -> Unsigned<BigInt>,
        resv_secs -> Unsigned<BigInt>,
        over_secs -> Unsigned<BigInt>,
    }
}

table! {
    cluster_usage_month_table (id_tres, time_start){
        creation_time -> Unsigned<BigInt>,
        mod_time -> Unsigned<BigInt>,
        deleted -> TinyInt,
        id_tres -> Integer,
        time_start -> Unsigned<BigInt>,
        count -> Unsigned<BigInt>,
        alloc_secs -> Unsigned<BigInt>,
        down_secs -> Unsigned<BigInt>,
        pdown_secs -> Unsigned<BigInt>,
        idle_secs -> Unsigned<BigInt>,
        resv_secs -> Unsigned<BigInt>,
        over_secs -> Unsigned<BigInt>,
    }
}

table! {
    cluster_wckey_table (id_wckey) {
        creation_time -> Unsigned<BigInt>,
        mod_time -> Unsigned<BigInt>,
        deleted -> TinyInt,
        is_def -> TinyInt,
        id_wckey -> Unsigned<Integer>,
        wckey_name -> Text,
        user-> Text,
    }
}

table! {
    cluster_wckey_usage_day_table (id, id_tres, time_start) {
        creation_time -> Unsigned<BigInt>,
        mod_time -> Unsigned<BigInt>,
        deleted -> TinyInt,
        id -> Unsigned<Integer>,
        id_tres -> Integer,
        time_start -> Unsigned<BigInt>,
        alloc_secs -> Unsigned<BigInt>,
    }
}

table! {
    cluster_wckey_usage_hour_table (id, id_tres, time_start) {
        creation_time -> Unsigned<BigInt>,
        mod_time -> Unsigned<BigInt>,
        deleted -> TinyInt,
        id -> Unsigned<Integer>,
        id_tres -> Integer,
        time_start -> Unsigned<BigInt>,
        alloc_secs -> Unsigned<BigInt>,
    }
}

table! {
    cluster_wckey_usage_month_table (id, id_tres, time_start) {
        creation_time -> Unsigned<BigInt>,
        mod_time -> Unsigned<BigInt>,
        deleted -> TinyInt,
        id -> Unsigned<Integer>,
        id_tres -> Integer,
        time_start -> Unsigned<BigInt>,
        alloc_secs -> Unsigned<BigInt>,
    }
}

table! {
    user_table (name) {
        creation_time -> BigInt,
        mod_time -> BigInt,
        deleted -> TinyInt,
        name -> Text,
        admin_level -> TinyInt,
    }
}

