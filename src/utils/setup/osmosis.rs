use super::super::{
    super::{
        error::Error,
        types::osmosis::{CosmWasmPoolType, PoolInitParams, PoolType},
        DEFAULT_KEY, OSMOSIS_CHAIN_NAME, OSMOSIS_POOLFILE_PATH,
    },
    test_context::TestContext,
};
use astroport::{
    asset::AssetInfo,
    factory::{self, PairType},
    pair_concentrated::ConcentratedPoolParams,
};
use cosmwasm_std::{Binary, Decimal};
use std::{fs::OpenOptions, io::Write, path::Path};

pub struct CreateOsmoPoolTxBuilder<'a> {
    key: &'a str,
    flags: Option<&'a str>,
    pool_type: PoolType,
    pool_init_params: Option<PoolInitParams>,
    weights: Vec<(u64, &'a str)>,
    initial_deposit: Vec<(u64, &'a str)>,
    swap_fee: Decimal,
    exit_fee: Decimal,
    future_governor: &'a str,
    test_ctx: &'a mut TestContext,
}

impl<'a> CreateOsmoPoolTxBuilder<'a> {
    pub fn with_key(&mut self, key: &'a str) -> &mut Self {
        self.key = key;

        self
    }

    pub fn with_flags(&mut self, flags: &'a str) -> &mut Self {
        self.flags = Some(flags);

        self
    }

    pub fn with_pool_type(&mut self, pool_type: PoolType) -> &mut Self {
        self.pool_type = pool_type;

        self
    }

    pub fn with_pool_init_params(&mut self, init_params: PoolInitParams) -> &mut Self {
        self.pool_init_params = Some(init_params);

        self
    }

    pub fn with_weight(&mut self, denom: &'a str, weight: u64) -> &mut Self {
        self.weights.push((weight, denom));

        self
    }

    pub fn with_initial_deposit(&mut self, denom: &'a str, deposit: u64) -> &mut Self {
        self.initial_deposit.push((deposit, denom));

        self
    }

    pub fn with_swap_fee(&mut self, swap_fee: Decimal) -> &mut Self {
        self.swap_fee = swap_fee;

        self
    }

    pub fn with_exit_fee(&mut self, exit_fee: Decimal) -> &mut Self {
        self.exit_fee = exit_fee;

        self
    }

    pub fn with_future_governor(&mut self, future_governor: &'a str) -> &mut Self {
        self.future_governor = future_governor;

        self
    }

    /// Sends the transaction, returning the pool ID if it was created successfully.
    pub fn send(&mut self) -> Result<(), Error> {
        self.test_ctx.tx_create_osmo_pool(
            self.key,
            self.flags,
            self.pool_type,
            self.pool_init_params.clone(),
            self.weights.iter().cloned(),
            self.initial_deposit.iter().cloned(),
            self.swap_fee,
            self.exit_fee,
            self.future_governor,
        )
    }
}

pub struct FundOsmoPoolTxBuilder<'a> {
    key: &'a str,
    pool_id: Option<u64>,
    max_amounts_in: Vec<(u64, &'a str)>,
    share_amount_out: Option<u64>,
    test_ctx: &'a mut TestContext,
}

impl<'a> FundOsmoPoolTxBuilder<'a> {
    pub fn with_key(&mut self, key: &'a str) -> &mut Self {
        self.key = key;

        self
    }

    pub fn with_pool_id(&mut self, pool_id: u64) -> &mut Self {
        self.pool_id = Some(pool_id);

        self
    }

    pub fn with_max_amount_in(&mut self, denom: &'a str, amount: u64) -> &mut Self {
        self.max_amounts_in.push((amount, denom));

        self
    }

    pub fn with_share_amount_out(&mut self, share_amount_out: u64) -> &mut Self {
        self.share_amount_out = Some(share_amount_out);

        self
    }

    /// Sends the transaction, returning the pool ID if it was created successfully.
    pub fn send(&mut self) -> Result<(), Error> {
        self.test_ctx.tx_fund_osmo_pool(
            self.key,
            self.pool_id
                .ok_or(Error::MissingBuilderParam(String::from("pool_id")))?,
            self.max_amounts_in.iter().cloned(),
            self.share_amount_out
                .ok_or(Error::MissingBuilderParam(String::from("share_amount_out")))?,
        )
    }
}

impl TestContext {
    pub fn build_tx_create_osmo_pool(&mut self) -> CreateOsmoPoolTxBuilder {
        CreateOsmoPoolTxBuilder {
            key: DEFAULT_KEY,
            flags: Default::default(),
            pool_type: PoolType::Xyk,
            pool_init_params: Default::default(),
            weights: Default::default(),
            initial_deposit: Default::default(),
            swap_fee: Decimal::percent(0),
            exit_fee: Decimal::percent(0),
            future_governor: "128h",
            test_ctx: self,
        }
    }

    /// Creates an osmosis pool with the given denoms.
    fn tx_create_osmo_pool<'a>(
        &mut self,
        key: &str,
        flags: Option<&str>,
        pool_type: PoolType,
        pool_params: Option<PoolInitParams>,
        weights: impl Iterator<Item = (u64, &'a str)>,
        initial_deposit: impl Iterator<Item = (u64, &'a str)>,
        swap_fee: Decimal,
        exit_fee: Decimal,
        future_governor: &'a str,
    ) -> Result<(), Error> {
        match pool_type {
            PoolType::Xyk => self.tx_create_osmo_pool_xyk(
                key,
                weights,
                initial_deposit,
                swap_fee,
                exit_fee,
                future_governor,
            ),
            PoolType::CosmWasm(CosmWasmPoolType::Pcl) => match pool_params.unwrap() {
                PoolInitParams::Pcl(params) => {
                    self.tx_create_osmo_pool_pcl(key, flags, weights, params)
                }
            },
        }
    }

    fn tx_create_osmo_pool_xyk<'a>(
        &mut self,
        key: &str,
        weights: impl Iterator<Item = (u64, &'a str)>,
        initial_deposit: impl Iterator<Item = (u64, &'a str)>,
        swap_fee: Decimal,
        exit_fee: Decimal,
        future_governor: &'a str,
    ) -> Result<(), Error> {
        let osmosis = self.get_chain(OSMOSIS_CHAIN_NAME);

        // Osmosisd requires a JSON file to specify the
        // configuration of the pool being created
        let poolfile_str = serde_json::json!({
            "weights": weights.map(|(weight, denom)| format!("{weight}{denom}")).collect::<Vec<_>>().join(","),
            "initial-deposit": initial_deposit.map(|(deposit, denom)| format!("{deposit}{denom}")).collect::<Vec<_>>().join(","),
            "swap-fee": swap_fee,
            "exit-fee": exit_fee,
            "future-governor": future_governor,
        })
        .to_string();

        // Copy poolfile to localosmo
        let mut f = OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .open(OSMOSIS_POOLFILE_PATH)?;
        f.write_all(poolfile_str.as_bytes())?;

        osmosis
            .rb
            .upload_file(Path::new(OSMOSIS_POOLFILE_PATH), true)?
            .send()?
            .text()?;

        let chain_id = &osmosis.rb.chain_id;
        let remote_poolfile_path = format!("/var/cosmos-chain/{chain_id}/pool_file.json");

        // Create pool
        let receipt = osmosis.rb.tx(
            format!("tx poolmanager create-pool --pool-file {remote_poolfile_path} --from {key} --fees 2500uosmo --gas 1000000")
            .as_str(),
            true,
        )?;

        self.guard_tx_errors(
            OSMOSIS_CHAIN_NAME,
            receipt
                .get("txhash")
                .and_then(|receipt| receipt.as_str())
                .ok_or(Error::TxMissingLogs)?,
        )?;

        Ok(())
    }

    fn tx_create_osmo_pool_pcl<'a>(
        &mut self,
        key: &str,
        flags: Option<&str>,
        weights: impl Iterator<Item = (u64, &'a str)>,
        init_params: ConcentratedPoolParams,
    ) -> Result<(), Error> {
        // Creating a PCL pool takes a few steps:
        // - Instantiating the contract for the PCL pool
        // - Registering the pool in x/cosmwasmpool
        // - Registering THAT pool in x/poolmanager

        // Start by creating the PCL contract instance
        // Select only denoms for weights (weights are not supplied at instantiation in astroport vs osmosis)
        // Support only native tokens, not cw 20's
        let asset_infos = weights
            .map(|(_, denom)| denom.to_owned())
            .map(|denom| AssetInfo::NativeToken { denom })
            .collect::<Vec<_>>();

        let factory = self.get_factory().src(OSMOSIS_CHAIN_NAME).get_cw();

        let receipt = factory
            .execute(
                key,
                &serde_json::to_string(&factory::ExecuteMsg::CreatePair {
                    pair_type: PairType::Custom(String::from("concentrated")),
                    asset_infos,
                    init_params: Some(Binary(serde_json::to_vec(&init_params).unwrap())),
                })
                .unwrap(),
                &format!(
                    "--fees 42069420uosmo{} --amount 1000000000uosmo",
                    flags.map(|flags| format!(" {flags}")).unwrap_or_default()
                ),
            )
            .unwrap();

        self.guard_tx_errors(
            OSMOSIS_CHAIN_NAME,
            receipt.tx_hash.ok_or(Error::TxMissingLogs)?.as_str(),
        )?;

        Ok(())
    }

    pub fn build_tx_fund_osmo_pool(&mut self) -> FundOsmoPoolTxBuilder {
        FundOsmoPoolTxBuilder {
            key: DEFAULT_KEY,
            pool_id: Default::default(),
            max_amounts_in: Default::default(),
            share_amount_out: Default::default(),
            test_ctx: self,
        }
    }

    /// Creates an osmosis pool with the given denoms.
    fn tx_fund_osmo_pool<'a>(
        &mut self,
        key: &str,
        pool_id: u64,
        max_amounts_in: impl Iterator<Item = (u64, &'a str)>,
        share_amount_out: u64,
    ) -> Result<(), Error> {
        let osmosis = self.get_chain(OSMOSIS_CHAIN_NAME);

        // Enter LP
        let receipt = osmosis.rb.tx(
            format!("tx gamm join-pool --pool-id {pool_id} --max-amounts-in {} --share-amount-out {share_amount_out} --from {key} --fees 2500uosmo --gas 1000000", max_amounts_in.map(|(weight, denom)| format!("{weight}{denom}")).collect::<Vec<_>>().join(","))
            .as_str(),
            true,
        )?;

        self.guard_tx_errors(
            OSMOSIS_CHAIN_NAME,
            receipt
                .get("txhash")
                .and_then(|receipt| receipt.as_str())
                .ok_or(Error::TxMissingLogs)?,
        )?;

        Ok(())
    }
}
