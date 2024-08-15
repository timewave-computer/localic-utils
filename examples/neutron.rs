use cosmwasm_std::Decimal;
use localic_utils::{types::contract::MinAmount, ConfigChainBuilder, TestContextBuilder};
use std::error::Error;

const ACC_0_ADDR: &str = "neutron1hj5fveer5cjtn4wd6wstzugjfdxzl0xpznmsky";

/// Demonstrates using localic-utils for neutron.
fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    // Create a testcontext
    let mut ctx = TestContextBuilder::default()
        .with_unwrap_raw_logs(true)
        .with_api_url("http://localhost:42069/")
        .with_artifacts_dir("contracts")
        .with_chain(ConfigChainBuilder::default_neutron().build()?)
        .build()?;

    // Upload contracts
    ctx.build_tx_upload_contracts().send()?;

    // Create a token in the tokenfactory
    ctx.build_tx_create_tokenfactory_token()
        .with_subdenom("bruhtoken")
        .send()?;
    ctx.build_tx_create_tokenfactory_token()
        .with_subdenom("amoguscoin")
        .send()?;

    let bruhtoken = ctx.get_tokenfactory_denom(ACC_0_ADDR, "bruhtoken");
    let amoguscoin = ctx.get_tokenfactory_denom(ACC_0_ADDR, "amoguscoin");

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

    ctx.get_auction(("untrn", ctx.get_tokenfactory_denom(ACC_0_ADDR, "bruhtoken")))?;
    ctx.get_auction((
        "untrn",
        ctx.get_tokenfactory_denom(ACC_0_ADDR, "amoguscoin"),
    ))?;

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

    let pool = ctx.get_astroport_pool(
        "untrn",
        ctx.get_tokenfactory_denom(ACC_0_ADDR, "amoguscoin"),
    )?;

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
        .get_contract("astroport_whitelist")
        .unwrap()
        .code_id
        .unwrap();

    // Instantiate a contract predictably
    ctx.build_tx_instantiate2()
        .with_code_id(factory_contract_code_id)
        .with_msg(serde_json::json!({
            "admins": [],
            "mutable": false,
        }))
        .with_salt(hex::encode("examplesalt").as_str())
        .with_label("test_contract")
        .send()
        .unwrap();

    Ok(())
}
