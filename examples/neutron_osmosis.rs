use localic_utils::{ConfigChainBuilder, TestContextBuilder};
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
    ctx.build_tx_create_tokenfactory_token()
        .with_chain_name("osmosis")
        .with_subdenom("amoguscoin")
        .send()?;

    // Transfer from osmosis to neutron and neutron to osmosis

    Ok(())
}
