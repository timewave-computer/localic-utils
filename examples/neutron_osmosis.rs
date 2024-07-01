use localic_utils::{error::Error as LocalIcUtilsError, ConfigChainBuilder, TestContextBuilder};
use std::error::Error;

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
        .with_transfer_channel("osmosis", "neutron")
        .with_transfer_channel("neutron", "osmosis")
        .build()?;

    ctx.build_tx_create_tokenfactory_token()
        .with_chain_name("neutron")
        .with_subdenom("bruhtoken")
        .send()?;
    let bruhtoken = ctx.get_tokenfactory_denom(
        "neutron1kuf2kxwuv2p8k3gnpja7mzf05zvep0cyuy7mxg",
        "bruhtoken",
    );
    ctx.build_tx_mint_tokenfactory_token()
        .with_chain_name("neutron")
        .with_amount(10000000000000000000)
        .with_denom(&bruhtoken)
        .send()?;

    let ibc_bruhtoken = ctx.get_ibc_denom(&bruhtoken, "neutron", "osmosis").ok_or(
        LocalIcUtilsError::MissingContextVariable(format!("ibc_denom::{}", &bruhtoken)),
    )?;
    let ibc_neutron = ctx.get_ibc_denom("untrn", "neutron", "osmosis").ok_or(
        LocalIcUtilsError::MissingContextVariable(format!("ibc_denom::{}", "untrn")),
    )?;

    // Transfer from osmosis to neutron and neutron to osmosis
    ctx.build_tx_transfer()
        .with_chain_name("neutron")
        .with_recipient("osmo1kuf2kxwuv2p8k3gnpja7mzf05zvep0cysqyf2a")
        .with_denom("untrn")
        .with_amount(1000000)
        .send()?;
    ctx.build_tx_transfer()
        .with_chain_name("neutron")
        .with_recipient("osmo1kuf2kxwuv2p8k3gnpja7mzf05zvep0cysqyf2a")
        .with_denom(&bruhtoken)
        .with_amount(1000000)
        .send()?;

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
