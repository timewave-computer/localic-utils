use localic_std::{errors::LocalError, transactions::ChainRequestBuilder};
use log::info;
use serde_json::Value;

use crate::{utils::test_context::TestContext, ADMIN_KEY, STRIDE_CHAIN_NAME};

pub fn set_up_host_zone(test_ctx: &mut TestContext, dest_chain: &str) {
    let native_denom = test_ctx.get_native_denom().src(dest_chain).get().clone();

    let host_denom_on_stride = test_ctx
        .get_ibc_denom(native_denom, STRIDE_CHAIN_NAME, dest_chain)
        .unwrap();

    let stride = test_ctx.get_chain(STRIDE_CHAIN_NAME);
    let stride_rb = &stride.rb;

    let stride_to_host_channel_id = test_ctx
        .get_transfer_channels()
        .src(STRIDE_CHAIN_NAME)
        .dest(dest_chain)
        .get();

    let dest_chain_id = &test_ctx.get_chain(dest_chain).rb.chain_id;

    if query_host_zone(stride_rb, dest_chain_id) {
        info!("Host zone registered.");
    } else {
        info!("Host zone not registered.");
        register_stride_host_zone(
            stride_rb,
            &test_ctx
                .get_connections()
                .src(STRIDE_CHAIN_NAME)
                .dest(dest_chain)
                .get(),
            &test_ctx.get_native_denom().src(dest_chain).get(),
            &test_ctx.get_chain_prefix().src(dest_chain).get(),
            &host_denom_on_stride,
            &stride_to_host_channel_id,
            ADMIN_KEY,
        )
        .unwrap();
    }
}

pub fn query_host_zone(rb: &ChainRequestBuilder, chain_id: &str) -> bool {
    let query_cmd = format!("stakeibc show-host-zone {chain_id} --output=json");
    let host_zone_query_response = rb.q(&query_cmd, false);

    host_zone_query_response["host_zone"].is_object()
}

pub fn register_stride_host_zone(
    rb: &ChainRequestBuilder,
    connection_id: &str,
    host_denom: &str,
    bech_32_prefix: &str,
    ibc_denom: &str,
    channel_id: &str,
    from_key: &str,
) -> Result<Value, LocalError> {
    let cmd = format!(
        "tx stakeibc register-host-zone {} {} {} {} {} 1 --from={} --gas auto --gas-adjustment 1.3 --output=json",
        connection_id,
        host_denom,
        bech_32_prefix,
        ibc_denom,
        channel_id,
        from_key,
    );
    rb.tx(&cmd, true)
}
