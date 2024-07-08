use localic_utils::{ConfigChainBuilder, TestContextBuilder};
use std::error::Error;

const ACC_0_ADDR: &str = "osmo1hj5fveer5cjtn4wd6wstzugjfdxzl0xpwhpz63";
const NEUTRON_ACC_0_ADDR: &str = "neutron1hj5fveer5cjtn4wd6wstzugjfdxzl0xpznmsky";

/// Demonstrates using localic-utils for neutron + osmosis.
fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    // Create a testcontext
    let mut ctx = TestContextBuilder::default()
        .with_unwrap_raw_logs(true)
        .with_api_url("http://localhost:42069/")
        .with_artifacts_dir("contracts")
        .with_chain(ConfigChainBuilder::default_neutron().build()?)
        .with_chain(ConfigChainBuilder::default_osmosis().build()?)
        .with_transfer_channels("osmosis", "neutron")
        .build()?;

    ctx.build_tx_create_tokenfactory_token()
        .with_chain_name("neutron")
        .with_subdenom("bruhtoken")
        .send()?;
    let bruhtoken = ctx.get_tokenfactory_denom(NEUTRON_ACC_0_ADDR, "bruhtoken");
    ctx.build_tx_mint_tokenfactory_token()
        .with_chain_name("neutron")
        .with_amount(10000000000000000000)
        .with_denom(&bruhtoken)
        .send()?;

    // Transfer from osmosis to neutron and neutron to osmosis
    ctx.build_tx_transfer()
        .with_chain_name("neutron")
        .with_recipient(ACC_0_ADDR)
        .with_denom("untrn")
        .with_amount(1000000)
        .send()?;
    ctx.build_tx_transfer()
        .with_chain_name("neutron")
        .with_recipient(ACC_0_ADDR)
        .with_denom(&bruhtoken)
        .with_amount(1000000)
        .send()?;

    let ibc_bruhtoken = ctx.get_ibc_denom(&bruhtoken, "neutron", "osmosis");
    let ibc_neutron = ctx.get_ibc_denom("untrn", "neutron", "osmosis");

    // Create an osmosis pool
    ctx.build_tx_create_osmo_pool()
        .with_weight(&ibc_neutron, 1)
        .with_weight(&ibc_bruhtoken, 1)
        .with_initial_deposit(&ibc_neutron, 1)
        .with_initial_deposit(&ibc_bruhtoken, 1)
        .send()?;

    // Get its id
    let pool_id = ctx.get_osmo_pool(&ibc_neutron, &ibc_bruhtoken)?;

    // Fund the pool
    ctx.build_tx_fund_osmo_pool()
        .with_pool_id(pool_id)
        .with_max_amount_in(&ibc_neutron, 10000)
        .with_max_amount_in(&ibc_bruhtoken, 10000)
        .with_share_amount_out(1000000000000)
        .send()?;

    Ok(())
}
