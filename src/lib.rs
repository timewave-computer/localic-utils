pub mod error;
pub mod types;
pub mod utils;

/// A builder for the testing environment harness.
pub use utils::test_context::TestContextBuilder;

/// A builder for localic chain configs.
pub use types::config::ConfigChainBuilder;

/// The IBC port name for ibc transfers.
pub const TRANSFER_PORT: &str = "transfer";

/// File extension for WASM files
pub const WASM_EXTENSION: &str = "wasm";

/// Neutron chain info
pub const NEUTRON_CHAIN_NAME: &str = "neutron";
pub const NEUTRON_CHAIN_PREFIX: &str = "neutron";
pub const NEUTRON_CHAIN_DENOM: &str = "untrn";
pub const NEUTRON_CHAIN_ADMIN_ADDR: &str = "neutron1hj5fveer5cjtn4wd6wstzugjfdxzl0xpznmsky";

/// Osmosis chain info
pub const OSMOSIS_CHAIN_ID: &str = "localosmosis-1";
pub const OSMOSIS_CHAIN_DENOM: &str = "uosmo";
pub const OSMOSIS_CHAIN_PREFIX: &str = "osmo";
pub const OSMOSIS_CHAIN_ADMIN_ADDR: &str = "osmo1kuf2kxwuv2p8k3gnpja7mzf05zvep0cysqyf2a";
pub const OSMOSIS_CHAIN_NAME: &str = "osmosis";
pub const OSMOSIS_POOLFILE_PATH: &str = "/tmp/pool_file.json";

/// Stride chain info
pub const STRIDE_CHAIN_ID: &str = "localstride-1";
pub const STRIDE_CHAIN_DENOM: &str = "ustrd";
pub const STRIDE_CHAIN_PREFIX: &str = "stride";
pub const STRIDE_CHAIN_ADMIN_ADDR: &str = "stride1u20df3trc2c2zdhm8qvh2hdjx9ewh00sv6eyy8";
pub const STRIDE_CHAIN_NAME: &str = "stride";

/// File names
pub const AUCTION_CONTRACT_NAME: &str = "auction";
pub const AUCTIONS_MANAGER_CONTRACT_NAME: &str = "auctions_manager";
pub const TOKEN_REGISTRY_NAME: &str = "astroport_native_coin_registry";
pub const FACTORY_NAME: &str = "astroport_factory";
pub const PAIR_NAME: &str = "astroport_pair";
pub const STABLE_PAIR_NAME: &str = "astroport_pair_stable";
pub const TOKEN_NAME: &str = "cw20_base";
pub const WHITELIST_NAME: &str = "astroport_whitelist";
pub const PRICE_ORACLE_NAME: &str = "price_oracle";

/// Local ic info
pub const LOCAL_IC_API_URL: &str = "http://localhost:42069/";

/// Builder defautls
pub const DEFAULT_KEY: &str = "acc0";
pub const DEFAULT_TRANSFER_PORT: &str = "transfer";
pub const DEFAULT_AUCTION_LABEL: &str = "auction";
pub const DEFAULT_NEUTRON_CHAIN_ID: &str = "localneutron-1";

pub const TX_HASH_QUERY_RETRIES: u16 = 5;
pub const TX_HASH_QUERY_PAUSE_SEC: u64 = 2;
