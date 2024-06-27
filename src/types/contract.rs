use serde::Serialize;
use std::path::PathBuf;

/// A deployed CosmWasm contract with a code id, address, and artifact path.
#[derive(Debug)]
pub struct DeployedContractInfo {
    pub code_id: u64,
    pub address: String,
    pub artifact_path: PathBuf,
}

/*
    Valence contract bindings
*/

#[derive(Serialize)]
pub struct AuctionStrategy {
    pub start_price_perc: u64,
    pub end_price_perc: u64,
}

#[derive(Serialize)]
pub struct PriceFreshnessStrategy {
    pub limit: String,
    pub multipliers: Vec<(String, String)>,
}

#[derive(Serialize)]
pub struct ChainHaltConfig {
    pub cap: String,
    pub block_avg: String,
}

#[derive(Serialize)]
pub struct MinAmount {
    pub send: String,
    pub start_auction: String,
}
