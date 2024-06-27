use astroport::factory::PairType;
use localic_utils::{types::contract::MinAmount, ConfigChainBuilder, TestContextBuilder};
use std::error::Error;

/// Demonstrates using localic-utils for neutron.
fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    // Create a testcontext
    let mut ctx = TestContextBuilder::default()
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

    // Deploy valence auctions
    ctx.build_tx_create_auctions_manager()
        .with_min_auction_amount(&[(
            &String::from("untrn"),
            MinAmount {
                send: "0".into(),
                start_auction: "0".into(),
            },
        )])
        .send()?;

    let bruhtoken = ctx.get_tokenfactory_denom(
        "neutron1kuf2kxwuv2p8k3gnpja7mzf05zvep0cyuy7mxg",
        "bruhtoken",
    );
    let amoguscoin = ctx.get_tokenfactory_denom(
        "neutron1kuf2kxwuv2p8k3gnpja7mzf05zvep0cyuy7mxg",
        "amoguscoin",
    );

    ctx.build_tx_create_auction()
        .with_offer_asset("untrn")
        .with_ask_asset(bruhtoken.as_str())
        .with_amount_denom_a(10000)
        .send()?;
    ctx.build_tx_create_auction()
        .with_offer_asset("untrn")
        .with_ask_asset(amoguscoin.as_str())
        .with_amount_denom_a(10000)
        .send()?;

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

    ctx.build_tx_create_token_registry()
        .with_owner("neutron1kuf2kxwuv2p8k3gnpja7mzf05zvep0cyuy7mxg")
        .send()?;
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
