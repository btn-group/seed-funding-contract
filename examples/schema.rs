use cosmwasm_schema::{export_schema, remove_schemas, schema_for};
use seed_funding_smart_contract::msg::{
    BalanceResponse, ConfigResponse, HandleMsg, InitMsg, QueryMsg,
};
use seed_funding_smart_contract::state::State;
use std::env::current_dir;
use std::fs::create_dir_all;

fn main() {
    let mut out_dir = current_dir().unwrap();
    out_dir.push("schema");
    create_dir_all(&out_dir).unwrap();
    remove_schemas(&out_dir).unwrap();

    export_schema(&schema_for!(InitMsg), &out_dir);
    export_schema(&schema_for!(HandleMsg), &out_dir);
    export_schema(&schema_for!(QueryMsg), &out_dir);
    export_schema(&schema_for!(State), &out_dir);
    export_schema(&schema_for!(BalanceResponse), &out_dir);
    export_schema(&schema_for!(ConfigResponse), &out_dir);
}
