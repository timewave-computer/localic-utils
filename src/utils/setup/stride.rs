use localic_std::{errors::LocalError, transactions::ChainRequestBuilder};
use log::info;
use serde_json::Value;

use crate::{
    error::Error, utils::test_context::TestContext, ADMIN_KEY, DEFAULT_KEY, STRIDE_CHAIN_NAME,
};

/// A tx liquid staking.
pub struct LiquidStakeTxBuilder<'a> {
    key: &'a str,
    denom: Option<&'a str>,
    amount: Option<u128>,
    test_ctx: &'a mut TestContext,
}

impl<'a> LiquidStakeTxBuilder<'a> {
    pub fn with_key(&mut self, key: &'a str) -> &mut Self {
        self.key = key;

        self
    }

    pub fn with_denom(&mut self, denom: &'a str) -> &mut Self {
        self.denom = Some(denom);

        self
    }

    pub fn with_amount(&mut self, amount: u128) -> &mut Self {
        self.amount = Some(amount);

        self
    }

    /// Sends the transaction.
    pub fn send(&mut self) -> Result<(), Error> {
        self.test_ctx.tx_liquid_stake(
            self.key,
            self.denom
                .ok_or(Error::MissingBuilderParam(String::from("denom")))?,
            self.amount
                .ok_or(Error::MissingBuilderParam(String::from("amount")))?,
        )
    }
}

impl TestContext {
    pub fn build_tx_liquid_stake(&mut self) -> LiquidStakeTxBuilder {
        LiquidStakeTxBuilder {
            key: DEFAULT_KEY,
            denom: None,
            amount: None,
            test_ctx: self,
        }
    }

    pub fn set_up_stride_host_zone(&mut self, dest_chain: &str) {
        let native_denom = self.get_native_denom().src(dest_chain).get().clone();

        let host_denom_on_stride = self.get_ibc_denom(&native_denom, dest_chain, STRIDE_CHAIN_NAME);

        let stride = self.get_chain(STRIDE_CHAIN_NAME);
        let stride_rb = &stride.rb;

        let stride_to_host_channel_id = self
            .get_transfer_channels()
            .src(STRIDE_CHAIN_NAME)
            .dest(dest_chain)
            .get();

        let dest_chain_id = &self.get_chain(dest_chain).rb.chain_id;

        if self.query_host_zone(stride_rb, dest_chain_id) {
            info!("Host zone registered.");
        } else {
            info!("Host zone not registered.");
            self.register_stride_host_zone(
                stride_rb,
                &self
                    .get_connections()
                    .src(STRIDE_CHAIN_NAME)
                    .dest(dest_chain)
                    .get(),
                &self.get_native_denom().src(dest_chain).get(),
                &self.get_chain_prefix().src(dest_chain).get(),
                &host_denom_on_stride,
                &stride_to_host_channel_id,
                ADMIN_KEY,
            )
            .unwrap();
        }
    }

    fn query_host_zone(&self, rb: &ChainRequestBuilder, chain_id: &str) -> bool {
        let query_cmd = format!("stakeibc show-host-zone {chain_id} --output=json");
        let host_zone_query_response = rb.q(&query_cmd, false);

        host_zone_query_response["host_zone"].is_object()
    }

    #[allow(clippy::too_many_arguments)]
    fn register_stride_host_zone(
        &self,
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

    fn tx_liquid_stake(
        &mut self,
        sender_key: &str,
        liquid_stake_denom: &str,
        liquid_stake_amount: u128,
    ) -> Result<(), Error> {
        let cmd = format!(
            "tx stakeibc liquid-stake {} {} --from={} --gas auto --gas-adjustment 1.3 --output=json",
            liquid_stake_amount, liquid_stake_denom, sender_key,
        );
        self.get_chain(STRIDE_CHAIN_NAME).rb.tx(&cmd, true)?;

        Ok(())
    }
}
