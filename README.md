# localic-utils

Utilities for writing tests with local-interchain.

## Usage

All utilities provided by localic-utils are member functions of a top-level `TestContext`, which stores information related to contract code ID's, channel ID's, and others. To create a `TestContext`, use the `TestContextBuilder` and specify chains to work with:

### Making a `TestContext`

```rust
use localic_utils::{ConfigChainBuilder, TestContextBuilder};

fn main() {
    let ctx = TestContextBuilder::default()
        .with_unwrap_raw_logs(true)
        .with_api_url("http://localhost:42069/")
        .with_artifacts_dir("contracts")
        .with_chain(ConfigChainBuilder::default_neutron()
        .build()
        .expect("Failed to build neutron"))
        .build()
		.expect("Failed to build TestContext");
}
```

#### Required builder calls

* `.with_artifacts_dir(dir: impl Into<String>)`
  * Should be a directory containing contracts for any features of localic-utils you would like to use, including:
    * Astroport factory, pair, and native coin registry contracts
	* Cw20 base contract
	* Valence auctions manager, auction, and price oracle contracts
  * **See a full list of contracts used and their expected names in [lib.rs](https://github.com/timewave-computer/localic-utils/blob/main/src/lib.rs).**	

The `TestContextBuilder` cannot be successfully built without specifying an artifacts dir. The builder will return an `Error::MissingBuilderParam` if this call is not made.

#### Notable optional builder calls

For a full list of builder calls available under the `TestContextBuilder`, see [test_context.rs](https://github.com/timewave-computer/localic-utils/blob/main/src/utils/test_context.rs).

* `.with_api_url(api_url: impl Into<String>)`
* `.with_chain(chain: ConfigChain)`
  * The `TestContext` is not configured to use any chains by default. Calling this builder method adds a `ConfigChain`, which grants the `TestContext` access to that chain's related helper functions. These helper functions will error without access to their requisite chains.
* `.with_transfer_channel(chain_a: impl Into<String>, chain_b: impl Into<String>)`
  * Registers a transfer channel ID upon building the `TestContext` between chain A and chain B. Assumes that chain A and chain B are chains registered with `.with_chain`
* `.with_unwrap_raw_logs(unwrap_logs: bool)`
  * Enables or disables log unwrapping - an assertion upon every `tx_*` helper function's execution that ensures no errors are present in logs returned by the transaction

#### Finalizing the builder

The builder can be converted into a `TestContext` by calling `.build`. This function will return an `Err` result variant if one of the required calls is missing.

### Making a `ConfigChain`

```rust
use localic_utils::{ConfigChainBuilder};

fn main() {
    let chain = ConfigChainBuilder::default_neutron()
        .build()
        .expect("Failed to build neutron");
}
```

#### Required builder calls

If one of the default chain builders (`ConfigChainBuilder::default_neutron`, `ConfigChainBuilder::default_osmosis`, or `ConfigChainBuilder::default_stride`) is used, then no builder calls need to be performed. However, if `ConfigChainBuilder::default` is used, then the following must all be called:

* `.with_denom(denom: impl Into<String>)`
* `.with_debugging(debugging: bool)`
* `.with_chain_id(chain_id: impl Into<String>)`
* `.with_chain_name(chain_name: impl Into<String>)`
* `.with_chain_prefix(prefix: impl Into<String>)`
* `.with_admin_addr(addr: impl Into<String>)`

#### Notable optional builder calls

Even if one of the default chain builders are used, the following builder calls may be of use:

* `.with_admin_addr(addr: impl Into<String>)`
  * By default, every chain's admin addr is derived from the mnemonic "decorate bright ozone fork gallery riot bus exhaust worth way bone indoor calm squirrel merry zero scheme cotton until shop any excess stage laundry." However, this admin addr may be overridden with `.with_admin_addr`.
* `.with_chain_id(chain_id: impl Into<String>)`
  * For some use cases, it may be useful to use `neutron-1`, or another chain ID, instead of `localneutron-1`. This builder call serves that purpose.

#### Finalizing the builder

The builder can be converted into a `ConfigChain` by calling `.build`. This function will return an `Err` result variant if one of the required calls is missing.

### Utility Functions

Note that most `tx_*` helper functions expose a `.with_key(key: &str)` builder function which specifies which key is signing the transaction. Furthermore, all `tx_*` helper builders can be sent as transactions with `.send`

#### General utility functions

* `.build_tx_upload_contracts` - Uploads all contracts in the specified artifacts dir to Neutron.
  * No required builder calls
  * No notable optional builder calls

#### Tokens

* `.build_tx_create_tokenfactory_token` - Creates a tokenfactory token from `acc0` on Neutron by default.
  * Required builder calls:
    * `.with_subdenom(subdenom: &str)`
  * Notable optional builder calls:
    * `.with_chain_name(chain_name: impl Into<String>)` - Should be on of `"osmosis" | "neutron" | "stride"` or one of the registered chain names from `.with_chain`
* `.get_tokenfactory_denom(key: &str, subdenom: &str)` - Gets the tokenfactory denom of a tokenfactory token given its subdenom and key
* `.build_tx_mint_tokenfactory_token` - Mints a tokenfactory token from `acc0` on Neutron by default.
  * Required builder calls
    * `.with_denom(denom: &str)` - Note that the denom provided should be a full tokenfactory denom, not a subdenom.
	* `.with_amount(amount: u128)`
  * Required builder calls for osmosis
    * `.with_recipient_addr(addr: &str)` - Specifies a recipient of the minted tokens on Osmosis. This builder call does nothing on Neutron.
  * Notable optional builder calls:
    * `.with_chain_name(chain_name: impl Into<String>)` - Specifies which chain to mint the tokens on. See previous notes about chain names.

#### Auctions

* `.build_tx_create_auctions_manager` - Creates an auctions manager, registering it to the `TestContext` and overwriting the previous registered auctions manager (this is useful for cached tests)
  * Required builder calls:
    * No required builder calls
  * Notable optional builder calls:
    * `.with_server_addr(addr: &str)` - Should be the admin address, or whichever address will be interacting with the auctions manager
	* `.with_min_auction_amount(min_auction_amount: &[(&str, struct MinAmount { send: String, start_auction: String })])`
	  * Where the first element in a provided tuple is the offered denom
	  * Where `send` specifies the minimum amount that can be sent to the auction
	  * Where `start_auction` specifies the minimum balance that an auction can be started with
* `.build_tx_create_price_oracle` - Creates a price oracle
  * Required builder calls:
    * No required builder calls
  * Notable optional builder calls:
    * `.with_seconds_allow_manual_change(sec: u64)` - Specifies how long after a change in the oracle's price that the admin can manually update the price (in seconds)
	* `.with_seconds_auction_prices_fresh(sec: u64)` - Specifies how long an auction will be used as the price source (deemed "fresh") in seconds
* `.build_tx_manual_oracle_price_update` - Manually updates the price of the registered price oracle, given an offer asset and ask asset
  * Required builder calls:
    * `.with_offer_asset(offer_asset: &str)`
	* `.with_ask_asset(ask_asset: &str)`
	* `.with_price(price: cosmwasm_std::Decimal)`
* `.build_tx_update_auction_oracle` - Overwrites the auction oracle address in the auctions manager, setting it to the registered auction oracle
  * Required builder calls:
    * No required builder calls
  * Notable optional builder calls:
    * No notable optional builder calls
* `.build_tx_create_auction` - Creates an auction on the registered auction manager given an offer and ask asset
  * Required builder calls:
    * `.with_offer_asset(offer_asset: &str)`
    * `.with_ask_asset(ask_asset: &str)`
	* `.with_amount_offer_asset(amount: u128)`
  * Notable optional builder calls:
    * `.with_auction_strategy(strategy: struct AuctionStrategy { start_price_perc: u64, end_price_perc: u64 })`
	  * Where `start_price_perc` and `end_price_perc` are specified in basis points (defaults are 5000 and 5000, respectively)
	* `.with_chain_halt_config(chain_halt_config: struct ChainHaltConfig { cap: String, block_avg: String })`
	  * Where `cap` is a time in milliseconds
	  * Where `block_avg` is a decimal value in seconds (see valence-services for more details)
* `.get_auction(offer_ask_assets: (impl AsRef<str>, impl AsRef<str>))` - Gets a `CosmWasm` instance for an auction identified by its ask and offer assets
* `.build_tx_fund_auction` - Sends funds to an auction, without starting it
  * Required builder calls:
    * `.with_offer_asset(offer_asset: &str)`
    * `.with_ask_asset(ask_asset: &str)`
	* `.with_amount_offer_asset(amount: u128)`
  * Notable optional builder calls:
    * No notable optional builder calls
* `.build_tx_start_auction` - Starts an auction identified by its offer and ask assets and an end block specified by a delta form the current block
  * Required builder calls:
    * `.with_offer_asset(offer_asset: &str)`
    * `.with_ask_asset(ask_asset: &str)`
	* `.with_end_block_delta(delta: u128)`
  * Notable builder calls:
    * No notable optional builder calls

#### Astroport

* `.build_tx_create_token_registry` - Creates the token registry with some owner
  * Required builder calls:
    * No required builder calls
  * Notable optional builder calls:
    * `.with_owner(addr: impl Into<String>)`
* `.build_tx_create_factory` - Creates the astroport pair factory, overwriting the previously saved factory address
  * Required builder calls:
    * No required builder calls
  * Notable optional builder calls:
    * `.with_owner(addr: impl Into<String>)`
* `.build_tx_create_pool` - Creates an astroport pool of a given type, and denoms
  * Required builder calls:
    * `.with_denom_a(denom: impl Into<String>)` - Sets the first token denom in the pool
	* `.with_denom_b(denom: impl Into<String>)` - Sets the second token denom in the pool
  * Notable optional builder calls:
    * `.with_pairtype(pairtype: astroport::factory::PairType)` - Sets the pair type to one of `PairType::Xyk | PairType::Stable | PairType::Custom(String)`
* `.get_astroport_pool(denom_a: impl AsRef<str> denom_b: impl AsRef<str>)` - Gets a `CosmWasm` instance for a pool identified by its constituent denoms
* `.build_tx_fund_pool` - Enters into a liquidity position in a pool identified by its constituent denoms
  * Required builder calls:
    * `.with_denom_a(denom: impl Into<String>)` - Sets the first token denom in the pool
	* `.with_denom_b(denom: impl Into<String>)` - Sets the second token denom in the pool
	* `.with_amount_denom_a(amount: u64)`
	* `.with_amount_denom_b(amount: u64)`
  * Notable optional builder calls:
    * `.with_slippage_tolerance(slippage_tolerance: Decimal)` - See astroport docs for more details
	* `.with_liq_token_receiver(receiver_addr: &str)`

#### Osmosis

* `.build_tx_create_osmo_pool` - Creates an osmosis pool with some weights and initial deposits
  * Required builder calls:
    * No builder calls are required. However, failing to specify any weights or initial deposits will result in failure to create the pool.
  * Notable optional builder calls:
    * `.with_weight(denom: &str, weight: u64)` - Specifies a weight for a token in the pool
	* `.with_initial_deposit(denom: &str, initial_deposit: u64)` - Specifies an initial deposit amount for a token in the pool
* `.get_osmo_pool(denom_a: impl AsRef<str>, denom_b: impl AsRef<str>)` - Gets the pool ID of a pool with some tokens
* `.build_tx_fund_osmo_pool` - Enters into a liquidity position in a pool with some ID (obtained with `.get_osmo_pool`)
  * Required builder calls:
    * `.with_pool_id(pool_id: u64)`
    * `.with_amount_in(denom: &str, amount: u64)` - Specifies the amount to fund the pool with of a given denom. The key sending this transaction should have the requisite funds to complete this transaction, otherwise it will fail.
	* `.with_share_amount_out(amout: u64)` - The number of gamm tokens to expect to be returned to the sender of the transaction (see notes on `.with_key`)
  * Notable optional builder calls:
    * No notable optional builder calls

### Complete Example

Examples of using almost every helper function provided by this repository are available in the [examples](https://github.com/timewave-computer/localic-utils/tree/main/examples) directory.
