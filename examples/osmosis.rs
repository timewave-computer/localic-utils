use astroport::pair_concentrated::ConcentratedPoolParams;
use cosmwasm_std::Decimal;
use localic_utils::{
    types::{
        contract::MinAmount,
        osmosis::{CosmWasmPoolType, PoolInitParams, PoolType},
    },
    ConfigChainBuilder, TestContextBuilder, DEFAULT_KEY, OSMOSIS_CHAIN_NAME,
};
use std::error::Error;

const ACC_0_ADDR: &str = "osmo1hj5fveer5cjtn4wd6wstzugjfdxzl0xpwhpz63";
const LOCAL_CODE_ID_CACHE_PATH: &str = "code_id_cache_osmo.json";

const TEST_TOKEN_1_NAME: &str = "bruhtoken1000";

/// Demonstrates using localic-utils for neutron.
fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    // Create a testcontext
    let mut ctx = TestContextBuilder::default()
        .with_unwrap_raw_logs(true)
        .with_api_url("http://localhost:42069/")
        .with_artifacts_dir("contracts")
        .with_chain(ConfigChainBuilder::default_neutron().build()?)
        .with_chain(ConfigChainBuilder::default_osmosis().build()?)
        .build()?;

    // Upload astroport contracts
    ctx.build_tx_upload_contracts().send_with_local_cache(
        "contracts_osmosis",
        OSMOSIS_CHAIN_NAME,
        LOCAL_CODE_ID_CACHE_PATH,
    )?;

    // Whitelist the PCL contract
    ctx.build_tx_whitelist_cosmwasm_pool()
        .with_contract_path("contracts_osmosis/astroport_pcl_osmo.wasm")
        .send()?;

    // Create some tokens on osmosis
    ctx.build_tx_create_tokenfactory_token()
        .with_chain(OSMOSIS_CHAIN_NAME)
        .with_subdenom(TEST_TOKEN_1_NAME)
        .send()?;
    let bruhtoken = ctx
        .get_tokenfactory_denom()
        .creator(ACC_0_ADDR)
        .subdenom(TEST_TOKEN_1_NAME.into())
        .get();
    ctx.build_tx_mint_tokenfactory_token()
        .with_chain(OSMOSIS_CHAIN_NAME)
        .with_amount(10000000000000000000)
        .with_denom(&bruhtoken)
        .with_recipient_addr(ACC_0_ADDR)
        .send()?;

    // Create an osmosis pool
    ctx.build_tx_create_token_registry()
        .with_owner(ACC_0_ADDR)
        .with_chain(OSMOSIS_CHAIN_NAME)
        .send()?;
    ctx.build_tx_create_factory()
        .with_owner(ACC_0_ADDR)
        .with_chain(OSMOSIS_CHAIN_NAME)
        .send()?;

    ctx.build_tx_create_osmo_pool()
        .with_pool_type(PoolType::CosmWasm(CosmWasmPoolType::Pcl))
        .with_weight("uosmo", 1)
        .with_weight(&bruhtoken, 1)
        .with_initial_deposit("uosmo", 1)
        .with_initial_deposit(&bruhtoken, 1)
        .with_pool_init_params(PoolInitParams::Pcl(ConcentratedPoolParams {
            amp: Decimal::one(),
            gamma: Decimal::one(),
            mid_fee: Decimal::one(),
            out_fee: Decimal::one(),
            fee_gamma: Decimal::one(),
            repeg_profit_threshold: Decimal::one(),
            min_price_scale_delta: Decimal::one(),
            price_scale: Decimal::one(),
            ma_half_time: 0,
            track_asset_balances: None,
            fee_share: None,
        }))
        .send()?;

    // Get its id
    let pool_id = ctx
        .get_osmo_pool()
        .denoms("uosmo".into(), bruhtoken.clone())
        .get_u64();

    // Fund the pool
    ctx.build_tx_fund_osmo_pool()
        .with_pool_id(pool_id)
        .with_max_amount_in("uosmo", 10000)
        .with_max_amount_in(&bruhtoken, 10000)
        .with_share_amount_out(1000000000000)
        .send()?;

    // Deploy some astroport and valence contracts to osmosis
    // Deploy valence auctions
    ctx.build_tx_create_auctions_manager()
        .with_min_auction_amount(&[(
            &String::from("uosmo"),
            MinAmount {
                send: "0".into(),
                start_auction: "0".into(),
            },
        )])
        .with_server_addr(ACC_0_ADDR)
        .with_chain(OSMOSIS_CHAIN_NAME)
        .send()?;
    ctx.build_tx_create_price_oracle()
        .with_chain(OSMOSIS_CHAIN_NAME)
        .send()?;
    ctx.build_tx_manual_oracle_price_update()
        .with_offer_asset("uosmo")
        .with_ask_asset(bruhtoken.as_str())
        .with_price(Decimal::percent(10))
        .with_chain(OSMOSIS_CHAIN_NAME)
        .send()?;
    ctx.build_tx_update_auction_oracle()
        .with_chain(OSMOSIS_CHAIN_NAME)
        .send()?;

    ctx.build_tx_create_auction()
        .with_offer_asset("uosmo")
        .with_ask_asset(bruhtoken.as_str())
        .with_amount_offer_asset(10000)
        .with_chain(OSMOSIS_CHAIN_NAME)
        .send()?;
    ctx.build_tx_create_auction()
        .with_offer_asset("uosmo")
        .with_ask_asset(bruhtoken.as_str())
        .with_amount_offer_asset(10000)
        .with_chain(OSMOSIS_CHAIN_NAME)
        .send()?;

    let _ = ctx
        .get_auction()
        .src(OSMOSIS_CHAIN_NAME)
        .offer_asset("uosmo")
        .ask_asset(
            &ctx.get_tokenfactory_denom()
                .creator(ACC_0_ADDR)
                .subdenom(TEST_TOKEN_1_NAME.to_owned())
                .get(),
        )
        .get_cw();
    let _ = ctx
        .get_auction()
        .src(OSMOSIS_CHAIN_NAME)
        .offer_asset("uosmo")
        .ask_asset(
            &ctx.get_tokenfactory_denom()
                .creator(ACC_0_ADDR)
                .subdenom(TEST_TOKEN_1_NAME.to_owned())
                .get(),
        )
        .get_cw();

    ctx.build_tx_create_pool()
        .with_denom_a("uosmo")
        .with_denom_b(bruhtoken.clone())
        .with_chain(OSMOSIS_CHAIN_NAME)
        .send()?;

    let pool = ctx
        .get_astro_pool()
        .denoms(
            "uosmo".to_owned(),
            ctx.get_tokenfactory_denom()
                .creator(ACC_0_ADDR)
                .subdenom(TEST_TOKEN_1_NAME.to_owned())
                .get(),
        )
        .get_cw();

    assert!(pool
        .query_value(&serde_json::json!({
            "pair": {}
        }))
        .get("data")
        .and_then(|data| data.get("asset_infos"))
        .is_some());

    ctx.build_tx_fund_auction()
        .with_offer_asset("uosmo")
        .with_ask_asset(bruhtoken.as_str())
        .with_amount_offer_asset(10000)
        .with_chain(OSMOSIS_CHAIN_NAME)
        .send()?;

    ctx.build_tx_start_auction()
        .with_offer_asset("uosmo")
        .with_ask_asset(bruhtoken.as_str())
        .with_end_block_delta(1000000)
        .with_chain(OSMOSIS_CHAIN_NAME)
        .send()?;

    ctx.build_tx_fund_pool()
        .with_denom_a("uosmo")
        .with_denom_b(bruhtoken)
        .with_amount_denom_a(10000)
        .with_amount_denom_b(10000)
        .with_slippage_tolerance(Decimal::percent(50))
        .with_liq_token_receiver(ACC_0_ADDR)
        .with_chain(OSMOSIS_CHAIN_NAME)
        .send()?;

    let factory_contract_code_id = ctx
        .get_contract()
        .contract("astroport_whitelist")
        .get_cw()
        .code_id
        .unwrap();

    // Instantiate a contract with a predictable address
    ctx.build_tx_instantiate2()
        .with_code_id(factory_contract_code_id)
        .with_msg(serde_json::json!({
            "admins": [],
            "mutable": false,
        }))
        .with_salt_hex_encoded(hex::encode("examplesalt").as_str())
        .with_label("test_contract")
        .with_flags("--gas 10000000")
        .send()
        .unwrap();

    let addr = ctx
        .get_built_contract_address()
        .contract("cw1_whitelist")
        .creator(ACC_0_ADDR)
        .salt_hex_encoded(hex::encode("examplesalt").as_str())
        .get();

    let mut cw = ctx.get_contract().contract("cw1_whitelist").get_cw();
    cw.contract_addr = Some(addr);

    cw.execute(
        DEFAULT_KEY,
        &serde_json::json!({ "execute": { "msgs": [] } }).to_string(),
        "",
    )
    .unwrap();

    Ok(())
}
