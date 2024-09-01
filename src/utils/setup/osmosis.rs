use super::super::{
    super::{
        error::Error,
        types::osmosis::{CosmWasmPoolType, PoolInitParams, PoolType},
        DEFAULT_KEY, OSMOSIS_CHAIN_ADMIN_ADDR, OSMOSIS_CHAIN_NAME, OSMOSIS_POOLFILE_PATH,
        OSMOSIS_WHITELIST_PROP_PATH, PAIR_PCL_ON_OSMOSIS_NAME, VALIDATOR_KEY,
    },
    test_context::TestContext,
};
use astroport::{
    asset::AssetInfo,
    factory::{self, PairType},
    pair_concentrated::ConcentratedPoolParams,
};
use base64::{prelude::BASE64_STANDARD, Engine};
use cosmwasm_std::{Binary, Decimal};
use serde_json::Value;
use std::{
    fs::{self, OpenOptions},
    io::Write,
    path::Path,
};

pub struct WhitelistCosmWasmPoolTxBuilder<'a> {
    key: &'a str,
    from: &'a str,
    contract_path: Option<&'a str>,
    test_ctx: &'a mut TestContext,
}

impl<'a> WhitelistCosmWasmPoolTxBuilder<'a> {
    pub fn with_key(&mut self, key: &'a str) -> &mut Self {
        self.key = key;

        self
    }

    pub fn with_proposer(&mut self, proposer: &'a str) -> &mut Self {
        self.from = proposer;

        self
    }

    pub fn with_contract_path(&mut self, contract_path: &'a str) -> &mut Self {
        self.contract_path = Some(contract_path);

        self
    }

    /// Sends the transaction, returning the pool ID if it was created successfully.
    pub fn send(&mut self) -> Result<(), Error> {
        self.test_ctx.tx_whitelist_cosmwasm_pool(
            self.key,
            self.from,
            self.contract_path
                .expect("missing whitelisted pool contract path"),
        )
    }
}

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
    pub fn build_tx_whitelist_cosmwasm_pool(&mut self) -> WhitelistCosmWasmPoolTxBuilder {
        WhitelistCosmWasmPoolTxBuilder {
            key: VALIDATOR_KEY,
            from: OSMOSIS_CHAIN_ADMIN_ADDR,
            contract_path: Default::default(),
            test_ctx: self,
        }
    }

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

        println!(
            "{:?} {:?}",
            serde_json::to_string(&init_params).unwrap(),
            serde_json::to_string(&factory::ExecuteMsg::CreatePair {
                pair_type: PairType::Custom(String::from("concentrated")),
                asset_infos: asset_infos.clone(),
                init_params: Some(Binary(serde_json::to_vec(&init_params).unwrap())),
            })
        );

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
                    "--fees 42069420uosmo{} --gas 42069420 --amount 1000000000uosmo",
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

    fn tx_whitelist_cosmwasm_pool(
        &mut self,
        key: &str,
        from: &str,
        contract_path: &str,
    ) -> Result<(), Error> {
        // Whitelist the pool cosmwasm contract by
        // writing the prop json to the osmosis node
        // and submitting a UploadCosmWasmPoolCodeAndWhiteListProposal prop
        // and voting to pass the prop
        let osmosis = self.get_chain(OSMOSIS_CHAIN_NAME);

        let authority_resp = osmosis
            .rb
            .q("q auth module-account gov --output=json", true);
        let authority_info: Value =
            serde_json::from_str(authority_resp["text"].as_str().unwrap()).unwrap();
        let authority = authority_info["account"]["base_account"]["address"]
            .as_str()
            .unwrap();

        // Read the bytecode of the contract
        let bytecode = fs::read(contract_path).unwrap();

        let prop = serde_json::to_vec(&serde_json::json!(
            {
               "messages": [
                   {
                       "@type": "/cosmos.gov.v1.MsgExecLegacyContent",
                       "content": {
                           "@type": "/osmosis.cosmwasmpool.v1beta1.UploadCosmWasmPoolCodeAndWhiteListProposal",
                           "title": "Whitelisting Contract",
                           "description": "",
                           "wasmByteCode": BASE64_STANDARD.encode(bytecode),
                       },
                       "authority": authority,
                   }
               ],
               "initial_deposit": [ { "denom": "uosmo", "amount": "100000" }],
               "title": "Whitelisting Contract",
               "summary": "Whitelisting Contract",
               "proposer": from,
            }
        )).unwrap();

        // Upload the proposal as JSON file
        fs::write(OSMOSIS_WHITELIST_PROP_PATH, prop.as_slice()).unwrap();

        osmosis
            .rb
            .upload_file(Path::new(OSMOSIS_WHITELIST_PROP_PATH), true)?
            .send()?
            .text()?;

        let chain_id = &osmosis.rb.chain_id;

        let remote_proposal_path = format!("/var/cosmos-chain/{chain_id}/prop.json");

        // Bond voting tokens
        let validator_address_resp = osmosis.rb.bin(
            &format!("keys show {key} --bech val --output=json --keyring-backend test --home /var/cosmos-chain/{chain_id}/"),
            true,
        );
        let validator_address_obj: Value =
            serde_json::from_str(&validator_address_resp["text"].as_str().unwrap()).unwrap();
        let validator_address = validator_address_obj["address"].as_str().unwrap();

        let receipt = osmosis
            .rb
            .tx(
                &format!("tx staking delegate {validator_address} 10000000000uosmo --from {key} --fees 42069420uosmo"),
                true,
            )
            .unwrap();

        self.guard_tx_errors(
            OSMOSIS_CHAIN_NAME,
            receipt
                .get("txhash")
                .and_then(|receipt| receipt.as_str())
                .ok_or(Error::TxMissingLogs)?,
        )?;

        // Submit the proposal
        let _ = osmosis.rb.tx(
            &format!(
                "tx gov submit-proposal {remote_proposal_path} --from {key} --fees 42069420uosmo --gas 771023100"
            ),
            true,
        )?;

        // Find the proposal's ID
        let props = osmosis
            .rb
            .q(&format!("q gov proposals --output=json"), true);

        let proposals_object: Value =
            serde_json::from_str(&props["text"].as_str().unwrap()).unwrap();

        let mut proposal_ids = proposals_object["proposals"]
            .as_array()
            .unwrap()
            .into_iter()
            .map(|prop| prop.as_object().unwrap()["id"].as_str().unwrap())
            .map(|prop_id_str| prop_id_str.parse::<i64>().unwrap())
            .collect::<Vec<_>>();
        proposal_ids.sort();

        // Deposit to activate the proposal
        let _ = osmosis.rb.tx(
            &format!(
                "tx gov deposit {} 10000000uosmo --from {key} --fees 42069420uosmo --gas 179841000",
                proposal_ids[proposal_ids.len() - 1]
            ),
            true,
        )?;

        // Vote on the proposal
        let receipt = osmosis.rb.tx(
            &format!(
                "tx gov vote {} yes --from {key} --fees 42069420uosmo",
                proposal_ids[proposal_ids.len() - 1]
            ),
            true,
        )?;

        self.guard_tx_errors(
            OSMOSIS_CHAIN_NAME,
            receipt
                .get("txhash")
                .and_then(|receipt| receipt.as_str())
                .ok_or(Error::TxMissingLogs)?,
        )?;

        // Get the whitelisted code
        let codes_resp: Value = serde_json::from_str(
            osmosis.rb.q("wasm list-code --output=json", true)["text"]
                .as_str()
                .unwrap(),
        )
        .unwrap();
        let codes = codes_resp["code_infos"]
            .as_array()
            .unwrap()
            .into_iter()
            .map(|code_id_object| code_id_object["code_id"].as_str().unwrap())
            .collect::<Vec<_>>();
        let code = codes[codes.len() - 1].parse::<u64>().unwrap();

        let osmosis = self.get_mut_chain(OSMOSIS_CHAIN_NAME);

        osmosis
            .contract_codes
            .insert(PAIR_PCL_ON_OSMOSIS_NAME.to_owned(), code);

        Ok(())
    }
}
