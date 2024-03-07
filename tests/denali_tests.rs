use dharitri_wasm::*;
use dharitri_wasm_debug::*;

fn contract_map() -> ContractMap<TxContext> {
    let mut contract_map = ContractMap::new();
    contract_map.register_contract(
        "file:../client/output/client.wasm",
        Box::new(|context| Box::new(client::contract_obj(context))),
    );
    contract_map.register_contract(
        "file:../oracle/output/oracle.wasm",
        Box::new(|context| Box::new(oracle::contract_obj(context))),
    );
    contract_map.register_contract(
        "file:../aggregator/output/aggregator.wasm",
        Box::new(|context| Box::new(aggregator::contract_obj(context))),
    );
    contract_map.register_contract(
        "file:../price-aggregator/output/price-aggregator.wasm",
        Box::new(|context| Box::new(price_aggregator::contract_obj(context))),
    );
    contract_map
}

#[test]
fn init() {
    dharitri_wasm_debug::denali_rs("denali/init.scen.json", &contract_map());
}

#[test]
fn client_request() {
    dharitri_wasm_debug::denali_rs("denali/client-request.scen.json", &contract_map());
}

#[test]
fn aggregator() {
    dharitri_wasm_debug::denali_rs("denali/aggregator.scen.json", &contract_map());
}

#[test]
fn init_price_aggregator() {
    dharitri_wasm_debug::denali_rs("denali/init-price-aggregator.scen.json", &contract_map());
}

#[test]
fn price_aggregator() {
    dharitri_wasm_debug::denali_rs("denali/price-aggregator.scen.json", &contract_map());
}

#[test]
fn price_aggregator_balance() {
    dharitri_wasm_debug::denali_rs("denali/price-aggregator-balance.scen.json", &contract_map());
}
