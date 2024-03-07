use dharitri_wasm_debug::*;

fn world() -> BlockchainMock {
    let mut blockchain = BlockchainMock::new();
    blockchain.register_contract_builder("file:client/output/client.wasm", client::ContractBuilder);
    blockchain.register_contract_builder("file:oracle/output/oracle.wasm", oracle::ContractBuilder);
    blockchain.register_contract_builder(
        "file:aggregator/output/aggregator.wasm",
        aggregator::ContractBuilder,
    );
    blockchain.register_contract_builder(
        "file:price-aggregator/output/price-aggregator.wasm",
        price_aggregator::ContractBuilder,
    );
    blockchain
}

#[test]
fn init() {
    dharitri_wasm_debug::denali_rs("denali/init.scen.json", world());
}

#[test]
fn client_request() {
    dharitri_wasm_debug::denali_rs("denali/client-request.scen.json", world());
}

#[test]
fn aggregator() {
    dharitri_wasm_debug::denali_rs("denali/aggregator.scen.json", world());
}

#[test]
fn init_price_aggregator() {
    dharitri_wasm_debug::denali_rs("denali/init-price-aggregator.scen.json", world());
}

#[test]
fn price_aggregator() {
    dharitri_wasm_debug::denali_rs("denali/price-aggregator.scen.json", world());
}

#[test]
fn price_aggregator_balance() {
    dharitri_wasm_debug::denali_rs("denali/price-aggregator-balance.scen.json", world());
}
