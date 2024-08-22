/// All pool types supported by local-ic utils and Osmosis
#[derive(Clone, Copy)]
pub enum PoolType {
    Xyk,
    CosmWasm(CosmWasmPoolType),
}

/// All cosmwasm pool types supported by local-ic utils
#[derive(Clone, Copy)]
pub enum CosmWasmPoolType {
    Pcl,
}
