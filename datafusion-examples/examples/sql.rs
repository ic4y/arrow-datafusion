// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.
#![feature(core_intrinsics)]
use datafusion::arrow::record_batch::RecordBatch;
use datafusion::arrow::util::pretty;
use datafusion::arrow::util::pretty::print_batches;
use std::sync::Arc;
use std::intrinsics::prefetch_read_data;

use datafusion::error::Result;
use datafusion::optimizer::common_subexpr_eliminate::CommonSubexprEliminate;
use datafusion::optimizer::constant_folding::ConstantFolding;
use datafusion::optimizer::eliminate_limit::EliminateLimit;
use datafusion::optimizer::filter_push_down::FilterPushDown;
use datafusion::optimizer::limit_push_down::LimitPushDown;
use datafusion::optimizer::projection_push_down::ProjectionPushDown;
use datafusion::optimizer::simplify_expressions::SimplifyExpressions;
// use datafusion::optimizer::single_distinct_to_groupby::SingleDistinctToGroupBy;
use datafusion::physical_optimizer::aggregate_statistics::AggregateStatistics;
use datafusion::physical_optimizer::coalesce_batches::CoalesceBatches;
use datafusion::physical_optimizer::hash_build_probe_order::HashBuildProbeOrder;
use datafusion::physical_optimizer::merge_exec::AddCoalescePartitionsExec;
use datafusion::prelude::*;
use chrono::prelude::*;

/// This example demonstrates executing a simple query against an Arrow data source (CSV) and
/// fetching results
#[tokio::main]
async fn main() -> Result<()> {
    //let parquet_path = "/Users/liliu/Desktop/lineorder_flat_2";
    let parquet_path = "/Users/liliu/Downloads/parquet_10/lineorder_flat";
    //let parquet_path = "/Users/liliu/Desktop/export_profile";

    let execution_config = ExecutionConfig::new().with_optimizer_rules(vec![
        Arc::new(ConstantFolding::new()),
        Arc::new(CommonSubexprEliminate::new()),
        Arc::new(EliminateLimit::new()),
        Arc::new(ProjectionPushDown::new()),
        Arc::new(FilterPushDown::new()),
        Arc::new(SimplifyExpressions::new()),
        Arc::new(LimitPushDown::new()),
        //Arc::new(SingleDistinctToGroupBy::new()),
    ]);
    let execution_config = ExecutionConfig::new();

    let mut ctx = ExecutionContext::with_config(execution_config);

    let result = ctx.register_parquet("lineorder_1", parquet_path).await?;
    //let sql = r#"select sum(distinct lo_orderkey) as a from lineorder_flat;"#;
    //let sql = r#"select count(distinct a.ee) from (select lo_orderpriority, count(distinct lo_orderkey) as ee from lineorder_flat group by lo_orderpriority)a"#;
    //let sql = "select lo_orderpriority, count(distinct lo_orderkey), max(distinct lo_orderkey) from lineorder_flat group by lo_orderpriority order by lo_orderpriority";
    //let sql = "select count(1),count(distinct distinct_id) from event";
    //let sql = "select sum(revenue) from (SELECT sum(LO_EXTENDEDPRICE) AS revenue  FROM lineorder_1 group by S_ADDRESS) a";
    //let sql = "select sum(revenue) from (SELECT sum(lo_extendedprice) AS revenue  FROM lineorder_1 group by lo_orderpriority) a";
    //let sql = "select * from lineorder_1";
    //let sql = "SELECT count(s_address) FROM lineorder_1 group by lo_orderpriority";
    let sql = "SELECT count(LO_EXTENDEDPRICE) FROM lineorder_1 group by S_ADDRESS";
    //let sql = "SELECT count(S_ADDRESS) FROM lineorder_1 group by LO_ORDERPRIORITY";

    let dt = Local::now();
    let df = ctx.sql(sql).await?;
    let logic_plan = df.to_logical_plan();
    println!("---------------------------------------");
    println!("sql : {}", sql);
    println!("---------------------------------------");
    println!("Display: {}", logic_plan.display_indent_schema());

    let results: Vec<RecordBatch> = df.collect().await?;

    //print_batches(&results)?;

    println!(
        "usage millis: {}",
        Local::now().timestamp_millis() - dt.timestamp_millis()
    );

    Ok(())
}