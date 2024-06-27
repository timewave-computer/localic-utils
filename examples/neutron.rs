use astroport::factory::PairType;
use localic_utils::{
    types::contract::{AuctionStrategy, ChainHaltConfig, MinAmount, PriceFreshnessStrategy},
    ConfigChainBuilder, TestContextBuilder,
};
use std::error::Error;

/// Demonstrates using localic-utils for neutron.
fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    let neutron = ConfigChainBuilder::default()
        .with_admin_addr("neutron1kuf2kxwuv2p8k3gnpja7mzf05zvep0cyuy7mxg")
        .with_chain_id("localneutron-1")
        .with_denom("untrn")
        .with_debugging(true)
        .build()?;

    // Create a testcontext
    let mut ctx = TestContextBuilder::default()
        .with_api_url("http://localhost:42069/")
        .with_artifacts_dir("contracts")
        .with_chain(neutron)
        .build()?;

    // Upload contracts
    ctx.tx_upload_contracts("acc0")?;

    // Create a token in the tokenfactory
    ctx.tx_create_tokenfactory_token("localneutron-1", "acc0", "bruhtoken")?;
    ctx.tx_create_tokenfactory_token("localneutron-1", "acc0", "amoguscoin")?;

    // Deploy valence auctions
    ctx.tx_create_auctions_manager(
        "acc0",
        [(
            String::from("untrn"),
            MinAmount {
                send: "0".into(),
                start_auction: "0".into(),
            },
        )],
        "neutron1kuf2kxwuv2p8k3gnpja7mzf05zvep0cyuy7mxg",
    )?;

    ctx.tx_create_auction(
        "acc0",
        (
            "untrn",
            ctx.get_tokenfactory_denom(
                "neutron1kuf2kxwuv2p8k3gnpja7mzf05zvep0cyuy7mxg",
                "bruhtoken",
            ),
        ),
        AuctionStrategy {
            start_price_perc: 5000,
            end_price_perc: 5000,
        },
        ChainHaltConfig {
            cap: "14400".into(),
            block_avg: "3".into(),
        },
        PriceFreshnessStrategy {
            limit: "3".into(),
            multipliers: vec![("2".into(), "2".into()), ("1".into(), "1.5".into())],
        },
        "bruh_auction",
        10000,
    )?;
    ctx.tx_create_auction(
        "acc0",
        (
            "untrn",
            ctx.get_tokenfactory_denom(
                "neutron1kuf2kxwuv2p8k3gnpja7mzf05zvep0cyuy7mxg",
                "amoguscoin",
            ),
        ),
        AuctionStrategy {
            start_price_perc: 5000,
            end_price_perc: 5000,
        },
        ChainHaltConfig {
            cap: "14400".into(),
            block_avg: "3".into(),
        },
        PriceFreshnessStrategy {
            limit: "3".into(),
            multipliers: vec![("2".into(), "2".into()), ("1".into(), "1.5".into())],
        },
        "amogus_auction",
        10000,
    )?;

    ctx.get_auction((
        "untrn",
        ctx.get_tokenfactory_denom(
            "neutron1kuf2kxwuv2p8k3gnpja7mzf05zvep0cyuy7mxg",
            "bruhtoken",
        ),
    ))?;
    ctx.get_auction((
        "untrn",
        ctx.get_tokenfactory_denom(
            "neutron1kuf2kxwuv2p8k3gnpja7mzf05zvep0cyuy7mxg",
            "amoguscoin",
        ),
    ))?;

    ctx.tx_create_token_registry("acc0", "neutron1kuf2kxwuv2p8k3gnpja7mzf05zvep0cyuy7mxg")?;
    ctx.tx_create_factory("acc0", "neutron1kuf2kxwuv2p8k3gnpja7mzf05zvep0cyuy7mxg")?;
    ctx.tx_create_pool(
        "acc0",
        PairType::Xyk {},
        "untrn",
        ctx.get_tokenfactory_denom(
            "neutron1kuf2kxwuv2p8k3gnpja7mzf05zvep0cyuy7mxg",
            "amoguscoin",
        )
        .as_str(),
    )?;
    ctx.tx_create_pool(
        "acc0",
        PairType::Xyk {},
        "untrn",
        ctx.get_tokenfactory_denom(
            "neutron1kuf2kxwuv2p8k3gnpja7mzf05zvep0cyuy7mxg",
            "bruhtoken",
        )
        .as_str(),
    )?;

    let pool = ctx.get_astroport_pool(
        "untrn",
        ctx.get_tokenfactory_denom(
            "neutron1kuf2kxwuv2p8k3gnpja7mzf05zvep0cyuy7mxg",
            "amoguscoin",
        ),
    )?;

    assert!(pool
        .query_value(&serde_json::json!({
            "pair": {}
        }))
        .get("data")
        .and_then(|data| data.get("asset_infos"))
        .is_some());

    ctx.tx_fund_auction(
        "acc0",
        (
            "untrn",
            ctx.get_tokenfactory_denom(
                "neutron1kuf2kxwuv2p8k3gnpja7mzf05zvep0cyuy7mxg",
                "amoguscoin",
            ),
        ),
        10000,
    )?;

    ctx.tx_start_auction(
        "acc0",
        (
            "untrn",
            ctx.get_tokenfactory_denom(
                "neutron1kuf2kxwuv2p8k3gnpja7mzf05zvep0cyuy7mxg",
                "amoguscoin",
            ),
        ),
    )?;

    ctx.tx_fund_pool(
        "acc0",
        "untrn",
        ctx.get_tokenfactory_denom(
            "neutron1kuf2kxwuv2p8k3gnpja7mzf05zvep0cyuy7mxg",
            "amoguscoin",
        ),
        1000,
        1000,
        "neutron1kuf2kxwuv2p8k3gnpja7mzf05zvep0cyuy7mxg",
    )?;

    Ok(())
}
