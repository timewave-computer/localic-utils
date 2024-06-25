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

/// Neutron chain ID
pub const NEUTRON_CHAIN_ID: &str = "localneutron-1";

/// File names
pub const AUCTION_CONTRACT_NAME: &str = "auction";
pub const AUCTIONS_MANAGER_CONTRACT_NAME: &str = "auctions_manager";
