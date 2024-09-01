use cosmwasm_std::Decimal;
use localic_utils::{
    types::contract::MinAmount, ConfigChainBuilder, TestContextBuilder, DEFAULT_KEY,
    NEUTRON_CHAIN_NAME,
};
use std::error::Error;

const ARTIFACTS_DIR: &str = "contracts";
const ACC_0_ADDR: &str = "neutron1hj5fveer5cjtn4wd6wstzugjfdxzl0xpznmsky";
const LOCAL_CODE_ID_CACHE_PATH: &str = "code_id_cache.json";

const TEST_TOKEN_1_NAME: &str = "bruhtoken3";
const TEST_TOKEN_2_NAME: &str = "amoguscoin3";

/// Demonstrates using localic-utils for neutron.
fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    // Create a testcontext
    let mut ctx = TestContextBuilder::default()
        .with_unwrap_raw_logs(true)
        .with_api_url("http://localhost:42069/")
        .with_artifacts_dir(ARTIFACTS_DIR)
        .with_chain(ConfigChainBuilder::default_neutron().build()?)
        .build()?;

    // Upload contracts
    ctx.build_tx_upload_contracts().send_with_local_cache(
        "contracts",
        NEUTRON_CHAIN_NAME,
        LOCAL_CODE_ID_CACHE_PATH,
    )?;

    // Create a token in the tokenfactory
    ctx.build_tx_create_tokenfactory_token()
        .with_subdenom(TEST_TOKEN_1_NAME)
        .send()?;
    ctx.build_tx_create_tokenfactory_token()
        .with_subdenom(TEST_TOKEN_2_NAME)
        .send()?;

    let bruhtoken = ctx
        .get_tokenfactory_denom()
        .creator(ACC_0_ADDR)
        .subdenom(TEST_TOKEN_1_NAME.to_owned())
        .get();
    let amoguscoin = ctx
        .get_tokenfactory_denom()
        .creator(ACC_0_ADDR)
        .subdenom(TEST_TOKEN_2_NAME.to_owned())
        .get();

    // Deploy valence auctions
    ctx.build_tx_create_auctions_manager()
        .with_min_auction_amount(&[(
            &String::from("untrn"),
            MinAmount {
                send: "0".into(),
                start_auction: "0".into(),
            },
        )])
        .with_server_addr(ACC_0_ADDR)
        .send()?;
    ctx.build_tx_create_price_oracle().send()?;
    ctx.build_tx_manual_oracle_price_update()
        .with_offer_asset("untrn")
        .with_ask_asset(amoguscoin.as_str())
        .with_price(Decimal::percent(10))
        .send()?;
    ctx.build_tx_update_auction_oracle().send()?;

    ctx.build_tx_mint_tokenfactory_token()
        .with_denom(bruhtoken.as_str())
        .with_amount(10000000000)
        .send()?;
    ctx.build_tx_mint_tokenfactory_token()
        .with_denom(amoguscoin.as_str())
        .with_amount(10000000000)
        .send()?;

    ctx.build_tx_create_auction()
        .with_offer_asset("untrn")
        .with_ask_asset(bruhtoken.as_str())
        .with_amount_offer_asset(10000)
        .send()?;
    ctx.build_tx_create_auction()
        .with_offer_asset("untrn")
        .with_ask_asset(amoguscoin.as_str())
        .with_amount_offer_asset(10000)
        .send()?;

    let _ = ctx
        .get_auction()
        .offer_asset("untrn")
        .ask_asset(
            &ctx.get_tokenfactory_denom()
                .creator(ACC_0_ADDR)
                .subdenom(TEST_TOKEN_1_NAME.to_owned())
                .get(),
        )
        .get_cw();
    let _ = ctx
        .get_auction()
        .offer_asset("untrn")
        .ask_asset(
            &ctx.get_tokenfactory_denom()
                .creator(ACC_0_ADDR)
                .subdenom(TEST_TOKEN_2_NAME.to_owned())
                .get(),
        )
        .get_cw();

    ctx.build_tx_create_token_registry()
        .with_owner(ACC_0_ADDR)
        .send()?;
    ctx.build_tx_create_factory()
        .with_owner(ACC_0_ADDR)
        .send()?;
    ctx.build_tx_create_pool()
        .with_denom_a("untrn")
        .with_denom_b(amoguscoin.clone())
        .send()?;
    ctx.build_tx_create_pool()
        .with_denom_a("untrn")
        .with_denom_b(bruhtoken)
        .send()?;

    let pool = ctx
        .get_astro_pool()
        .denoms(
            "untrn".to_owned(),
            ctx.get_tokenfactory_denom()
                .creator(ACC_0_ADDR)
                .subdenom(TEST_TOKEN_2_NAME.to_owned())
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
        .with_offer_asset("untrn")
        .with_ask_asset(amoguscoin.as_str())
        .with_amount_offer_asset(10000)
        .send()?;

    ctx.build_tx_start_auction()
        .with_offer_asset("untrn")
        .with_ask_asset(amoguscoin.as_str())
        .with_end_block_delta(1000000)
        .send()?;

    ctx.build_tx_fund_pool()
        .with_denom_a("untrn")
        .with_denom_b(amoguscoin)
        .with_amount_denom_a(10000)
        .with_amount_denom_b(10000)
        .with_slippage_tolerance(Decimal::percent(50))
        .with_liq_token_receiver(ACC_0_ADDR)
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
        .contract("astroport_whitelist")
        .creator(ACC_0_ADDR)
        .salt_hex_encoded(hex::encode("examplesalt").as_str())
        .get();

    let mut cw = ctx.get_contract().contract("astroport_whitelist").get_cw();
    cw.contract_addr = Some(addr);

    cw.execute(
        DEFAULT_KEY,
        &serde_json::json!({ "execute": { "msgs": [] } }).to_string(),
        "",
    )
    .unwrap();

    Ok(())
}
