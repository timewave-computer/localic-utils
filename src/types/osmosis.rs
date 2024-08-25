use astroport::pair_concentrated::ConcentratedPoolParams;

/// All pool types supported by local-ic utils and Osmosis
#[derive(Clone, Copy)]
pub enum PoolType {
    Xyk,
    CosmWasm(CosmWasmPoolType),
}

/// Init parameters for different osmosis pool types
#[derive(Clone)]
pub enum PoolInitParams {
    Pcl(ConcentratedPoolParams),
}

/// All cosmwasm pool types supported by local-ic utils
#[derive(Clone, Copy)]
pub enum CosmWasmPoolType {
    Pcl,
}
