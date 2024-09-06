use crate::{
    types::ibc::{get_prefixed_denom, parse_denom_trace},
    AUCTIONS_MANAGER_CONTRACT_NAME, TRANSFER_PORT,
};

use super::{
    super::{
        error::Error, AUCTION_CONTRACT_NAME, FACTORY_NAME, NEUTRON_CHAIN_NAME, OSMOSIS_CHAIN_NAME,
        PAIR_NAME, PRICE_ORACLE_NAME, STABLE_PAIR_NAME, TX_HASH_QUERY_PAUSE_SEC,
        TX_HASH_QUERY_RETRIES,
    },
    test_context::{LocalChain, TestContext},
};
use localic_std::{modules::cosmwasm::CosmWasm, transactions::ChainRequestBuilder};
use serde_json::Value;
use std::{path::PathBuf, thread, time::Duration};

pub enum QueryType {
    TransferChannel,
    Connection,
    CCVChannel,
    IBCDenom,
    AdminAddr,
    NativeDenom,
    ChainPrefix,
    RequestBuilder,
    BuiltContractAddress,
    CodeInfo,
    Contract,
    AuctionsManager,
    PriceOracle,
    Auction,
    TokenfactoryDenom,
    Factory,
    AstroPool,
    OsmoPool,
}

pub struct TestContextQuery<'a> {
    context: &'a TestContext,
    query_type: QueryType,
    src_chain: Option<String>,
    dest_chain: Option<String>,
    contract_name: Option<String>,

    // denoms in a pool
    offer_asset: Option<String>,
    ask_asset: Option<String>,
    denoms: Option<(String, String)>,

    base_denom: Option<String>,
    subdenom: Option<String>,

    // build-contract-address query args
    creator_address: Option<String>,
    salt_hex_encoded: Option<String>,
}

impl<'a> TestContextQuery<'a> {
    pub fn new(context: &'a TestContext, query_type: QueryType) -> Self {
        Self {
            context,
            query_type,
            src_chain: Some(NEUTRON_CHAIN_NAME.to_owned()),
            dest_chain: None,
            contract_name: None,
            offer_asset: None,
            ask_asset: None,
            base_denom: None,
            subdenom: None,
            denoms: None,
            creator_address: None,
            salt_hex_encoded: None,
        }
    }

    pub fn src(mut self, src_chain: &str) -> Self {
        self.src_chain = Some(src_chain.to_string());
        self
    }

    pub fn dest(mut self, dest_chain: &str) -> Self {
        self.dest_chain = Some(dest_chain.to_string());
        self
    }

    pub fn contract(mut self, contract_name: &str) -> Self {
        self.contract_name = Some(contract_name.to_string());
        self
    }

    pub fn offer_asset(mut self, offer_asset: &str) -> Self {
        self.offer_asset = Some(offer_asset.to_owned());
        self
    }

    pub fn ask_asset(mut self, ask_asset: &str) -> Self {
        self.ask_asset = Some(ask_asset.to_owned());
        self
    }

    pub fn denoms(mut self, denom_a: String, denom_b: String) -> Self {
        self.denoms = Some((denom_a, denom_b));
        self
    }

    pub fn base_denom(mut self, base_denom: String) -> Self {
        self.base_denom = Some(base_denom);
        self
    }

    pub fn subdenom(mut self, subdenom: String) -> Self {
        self.subdenom = Some(subdenom);
        self
    }

    pub fn creator(mut self, creator_addr: &str) -> Self {
        self.creator_address = Some(creator_addr.to_owned());
        self
    }

    pub fn salt_hex_encoded(mut self, salt_hex_encoded: &str) -> Self {
        self.salt_hex_encoded = Some(salt_hex_encoded.to_owned());
        self
    }

    pub fn get(self) -> String {
        // None cases explicitly enumerated here to ensure compilation-time
        // checking of query inclusion in some get_x method
        match self.query_type {
            QueryType::TransferChannel => self.get_transfer_channel().map(ToOwned::to_owned),
            QueryType::Connection => self.get_connection_id().map(ToOwned::to_owned),
            QueryType::CCVChannel => self.get_ccv_channel().map(ToOwned::to_owned),
            QueryType::IBCDenom => self.get_ibc_denom(),
            QueryType::AdminAddr => self.get_admin_addr().map(ToOwned::to_owned),
            QueryType::NativeDenom => self.get_native_denom().map(ToOwned::to_owned),
            QueryType::ChainPrefix => self.get_chain_prefix().map(ToOwned::to_owned),
            QueryType::BuiltContractAddress => self.get_built_contract_address(),
            QueryType::TokenfactoryDenom => self.get_tokenfactory_denom(),
            QueryType::CodeInfo
            | QueryType::OsmoPool
            | QueryType::AuctionsManager
            | QueryType::AstroPool
            | QueryType::Auction
            | QueryType::Contract
            | QueryType::Factory
            | QueryType::RequestBuilder
            | QueryType::PriceOracle => None,
        }
        .unwrap()
    }

    pub fn get_cw(self) -> CosmWasm<'a> {
        match self.query_type {
            QueryType::Contract => self.get_contract(),
            QueryType::Auction => self.get_auction(),
            QueryType::Factory | QueryType::PriceOracle | QueryType::AuctionsManager => {
                self.get_deployed_contract()
            }
            QueryType::AstroPool => self.get_astro_pool(),
            QueryType::TransferChannel
            | QueryType::Connection
            | QueryType::CCVChannel
            | QueryType::IBCDenom
            | QueryType::AdminAddr
            | QueryType::NativeDenom
            | QueryType::ChainPrefix
            | QueryType::BuiltContractAddress
            | QueryType::TokenfactoryDenom
            | QueryType::OsmoPool
            | QueryType::RequestBuilder
            | QueryType::CodeInfo => None,
        }
        .unwrap()
    }

    pub fn get_value(self) -> Value {
        match self.query_type {
            QueryType::CodeInfo => self.get_code_info(),
            QueryType::TransferChannel
            | QueryType::Connection
            | QueryType::CCVChannel
            | QueryType::IBCDenom
            | QueryType::AdminAddr
            | QueryType::NativeDenom
            | QueryType::ChainPrefix
            | QueryType::BuiltContractAddress
            | QueryType::TokenfactoryDenom
            | QueryType::OsmoPool
            | QueryType::AuctionsManager
            | QueryType::AstroPool
            | QueryType::Auction
            | QueryType::Contract
            | QueryType::Factory
            | QueryType::RequestBuilder
            | QueryType::PriceOracle => None,
        }
        .unwrap()
    }

    pub fn get_u64(self) -> u64 {
        match self.query_type {
            QueryType::OsmoPool => self.get_osmo_pool(),
            QueryType::TransferChannel
            | QueryType::Connection
            | QueryType::CCVChannel
            | QueryType::IBCDenom
            | QueryType::AdminAddr
            | QueryType::NativeDenom
            | QueryType::ChainPrefix
            | QueryType::BuiltContractAddress
            | QueryType::TokenfactoryDenom
            | QueryType::AuctionsManager
            | QueryType::AstroPool
            | QueryType::Auction
            | QueryType::Contract
            | QueryType::Factory
            | QueryType::RequestBuilder
            | QueryType::PriceOracle
            | QueryType::CodeInfo => None,
        }
        .unwrap()
    }

    pub fn get_all(self) -> Vec<String> {
        match self.query_type {
            QueryType::TransferChannel => self.get_all_transfer_channels(),
            QueryType::Connection => self.get_all_connections(),
            _ => vec![],
        }
        .into_iter()
        .map(ToOwned::to_owned)
        .collect::<Vec<_>>()
    }

    pub fn get_request_builder(mut self, chain: &str) -> &'a ChainRequestBuilder {
        self.src_chain = Some(chain.to_string());
        let rb = match self.query_type {
            QueryType::RequestBuilder => self.get_rb(),
            _ => None,
        };
        rb.unwrap()
    }

    fn get_transfer_channel(&self) -> Option<&str> {
        self.context
            .transfer_channel_ids
            .get(&(self.src_chain.clone()?, self.dest_chain.clone()?))
            .map(|s| s.as_str())
    }

    fn get_all_transfer_channels(&self) -> Vec<&str> {
        self.src_chain
            .as_ref()
            .map(|src| {
                self.context
                    .transfer_channel_ids
                    .iter()
                    .filter(|((s, _), _)| s == src)
                    .map(|(_, v)| v.as_str())
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default()
    }

    fn get_connection_id(&self) -> Option<&str> {
        self.context
            .connection_ids
            .get(&(self.src_chain.clone()?, self.dest_chain.clone()?))
            .map(|s| s.as_str())
    }

    fn get_all_connections(&self) -> Vec<&str> {
        self.src_chain
            .as_ref()
            .map(|src| {
                self.context
                    .connection_ids
                    .iter()
                    .filter(|((s, _), _)| s == src)
                    .map(|(_, s)| s.as_str())
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default()
    }

    fn get_ccv_channel(&self) -> Option<&str> {
        self.context
            .ccv_channel_ids
            .get(&(self.src_chain.clone()?, self.dest_chain.clone()?))
            .map(|s| s.as_str())
    }

    fn get_ibc_denom(&self) -> Option<String> {
        let dest_chain = self.dest_chain.as_deref()?;
        let src_chain = self.src_chain.as_deref()?;

        let channel_id = self
            .context
            .get_transfer_channels()
            .src(dest_chain)
            .dest(src_chain)
            .get();

        let prefixed_denom = get_prefixed_denom(
            TRANSFER_PORT.to_string(),
            channel_id.to_string(),
            self.base_denom.clone()?,
        );

        let src_denom_trace = parse_denom_trace(prefixed_denom);
        let ibc_denom = src_denom_trace.ibc_denom();

        Some(ibc_denom)
    }

    fn get_admin_addr(&self) -> Option<&str> {
        let src = self.src_chain.as_deref()?;

        Some(self.context.chains.get(src)?.admin_addr.as_ref())
    }

    fn get_native_denom(&self) -> Option<&str> {
        let src = self.src_chain.as_deref()?;

        Some(self.context.chains.get(src)?.native_denom.as_ref())
    }

    fn get_chain_prefix(&self) -> Option<&str> {
        let src = self.src_chain.as_deref()?;

        Some(self.context.chains.get(src)?.chain_prefix.as_ref())
    }

    fn get_code_info(&self) -> Option<Value> {
        let contract = self
            .context
            .get_contract()
            .src(self.src_chain.as_deref()?)
            .contract(self.contract_name.as_ref()?)
            .get_cw();
        let code_id = contract.code_id?;
        let chain = self.context.chains.get(self.src_chain.as_deref()?)?;

        // This will produce a { ... text: "{ 'data_hash': xyz }" }. Get the code info enclosed
        let resp = chain.rb.query(&format!("q wasm code-info {code_id}"), true);

        let str_info_object = resp["text"].as_str()?;
        serde_json::from_str(str_info_object).ok()
    }

    fn get_tokenfactory_denom(&self) -> Option<String> {
        let creator_addr = self.creator_address.as_deref()?;
        let subdenom = self.subdenom.as_deref()?;

        Some(format!("factory/{creator_addr}/{subdenom}"))
    }

    fn get_built_contract_address(&self) -> Option<String> {
        let code_info = self.get_code_info()?;
        let code_id_hash = code_info["data_hash"].as_str()?;

        let creator_address = self.creator_address.as_ref()?;
        let salt = self.salt_hex_encoded.as_deref()?;

        let chain = self.context.chains.get(self.src_chain.as_deref()?)?;

        let mut resp = chain.rb.bin(
            &format!("q wasm build-address {code_id_hash} {creator_address} {salt}"),
            true,
        );

        // text field contains built address
        match resp["text"].take() {
            Value::String(s) => Some(s.replace("\n", "")),
            _ => None,
        }
    }

    fn get_contract(&self) -> Option<CosmWasm<'a>> {
        let chain: &LocalChain = self.context.get_chain(self.src_chain.as_deref()?);
        let name = self.contract_name.as_deref()?;

        let code_id = chain.contract_codes.get(name)?;
        let artifacts_path = &self.context.artifacts_dir;

        Some(CosmWasm::new_from_existing(
            &chain.rb,
            Some(PathBuf::from(format!("{artifacts_path}/{name}.wasm"))),
            Some(*code_id),
            None,
        ))
    }

    fn get_deployed_contract(&self) -> Option<CosmWasm<'a>> {
        let chain = self.context.get_chain(self.src_chain.as_deref()?);
        let name = self.contract_name.as_deref()?;

        let code_id = chain.contract_codes.get(name)?;
        let contract_addr = chain.contract_addrs.get(name)?.clone();
        let artifacts_path = &self.context.artifacts_dir;

        Some(CosmWasm::new_from_existing(
            &chain.rb,
            Some(PathBuf::from(format!("{artifacts_path}/{name}.wasm"))),
            Some(*code_id),
            Some(contract_addr),
        ))
    }

    fn get_auction(&self) -> Option<CosmWasm<'a>> {
        let auction_manager = self
            .context
            .get_auctions_manager()
            .src(self.src_chain.as_deref()?)
            .get_cw();
        let denoms = (self.offer_asset.as_deref()?, self.ask_asset.as_deref()?);

        let resp = auction_manager.query(
            &serde_json::to_string(&serde_json::json!({
                "get_pair_addr": {
                    "pair": denoms,
                }}
            ))
            .unwrap(),
        );

        let mut cw = self
            .context
            .get_contract()
            .contract(PAIR_NAME)
            .src(self.src_chain.as_deref()?)
            .get_cw();
        cw.contract_addr = Some(resp["data"].as_str()?.to_owned());

        Some(cw)
    }

    fn get_astro_pool(&self) -> Option<CosmWasm<'a>> {
        let (denom_a, denom_b) = self.denoms.as_ref()?;
        let factory = self
            .context
            .get_factory()
            .src(self.src_chain.as_deref()?)
            .get_cw();

        let pair_info = factory.query_value(&serde_json::json!(
            {
                "pair": {
                    "asset_infos": [
                        {
                            "native_token": {
                                "denom": denom_a,
                            }
                        },
                        {
                            "native_token": {
                                "denom": denom_b,
                            }
                        }
                    ]
                }
            }
        ));

        let addr = pair_info
            .get("data")
            .and_then(|data| data.get("contract_addr"))
            .and_then(|addr| addr.as_str())
            .unwrap();
        let kind = pair_info
            .get("data")
            .and_then(|data| data.get("pair_type"))
            .unwrap();

        let chain = self.context.get_chain(self.src_chain.as_deref()?);

        if kind.get("xyk").is_some() {
            let contract = self
                .context
                .get_contract()
                .contract(PAIR_NAME)
                .src(self.src_chain.as_deref()?)
                .get_cw();

            return Some(CosmWasm::new_from_existing(
                &chain.rb,
                contract.file_path,
                contract.code_id,
                Some(addr.to_owned()),
            ));
        }

        let contract = self
            .context
            .get_contract()
            .contract(STABLE_PAIR_NAME)
            .src(self.src_chain.as_deref()?)
            .get_cw();

        Some(CosmWasm::new_from_existing(
            &chain.rb,
            contract.file_path,
            contract.code_id,
            Some(addr.to_owned()),
        ))
    }

    fn get_osmo_pool(&self) -> Option<u64> {
        // Do not use get_chain here, since we only want to support osmosis pools on osmosis
        let (denom_a, denom_b) = self.denoms.as_ref()?;
        let osmosis = self.context.get_chain(OSMOSIS_CHAIN_NAME);

        let res = osmosis.rb.query(
            &format!("q poolmanager list-pools-by-denom {denom_a} --output=json"),
            true,
        );

        let res_text = res.get("text").and_then(|v| v.as_str()).unwrap();
        let res_value: Value = serde_json::from_str(res_text).ok()?;

        let pools_value = res_value.get("pools").unwrap();
        let pool = pools_value
            .as_array()
            .and_then(|pools| {
                pools.iter().find(|pool_value| {
                    pool_value
                        .get("pool_assets")
                        .and_then(|assets_val| {
                            assets_val.as_array().and_then(|arr| {
                                arr.iter().find(|asset| {
                                    asset["token"]["denom"].as_str() == Some(denom_b.as_ref())
                                })
                            })
                        })
                        .is_some()
                })
            })
            .and_then(|pool| pool.get("id"))
            .and_then(|id_str| id_str.as_str())
            .unwrap();

        Some(pool.parse().unwrap())
    }

    fn get_rb(&self) -> Option<&'a ChainRequestBuilder> {
        if let Some(ref src) = self.src_chain {
            self.context.chains.get(src).map(|chain| &chain.rb)
        } else {
            None
        }
    }
}

impl TestContext {
    pub fn get_transfer_channels(&self) -> TestContextQuery {
        TestContextQuery::new(self, QueryType::TransferChannel)
    }

    pub fn get_connections(&self) -> TestContextQuery {
        TestContextQuery::new(self, QueryType::Connection)
    }

    pub fn get_ccv_channels(&self) -> TestContextQuery {
        TestContextQuery::new(self, QueryType::CCVChannel)
    }

    pub fn get_ibc_denom(&self) -> TestContextQuery {
        TestContextQuery::new(self, QueryType::IBCDenom)
    }

    pub fn get_admin_addr(&self) -> TestContextQuery {
        TestContextQuery::new(self, QueryType::AdminAddr)
    }

    pub fn get_native_denom(&self) -> TestContextQuery {
        TestContextQuery::new(self, QueryType::NativeDenom)
    }

    pub fn get_chain_prefix(&self) -> TestContextQuery {
        TestContextQuery::new(self, QueryType::ChainPrefix)
    }

    pub fn get_request_builder(&self) -> TestContextQuery {
        TestContextQuery::new(self, QueryType::RequestBuilder)
    }

    pub fn get_code_info(&self) -> TestContextQuery {
        TestContextQuery::new(self, QueryType::CodeInfo)
    }

    pub fn get_built_contract_address(&self) -> TestContextQuery {
        TestContextQuery::new(self, QueryType::BuiltContractAddress)
    }

    pub fn get_contract(&self) -> TestContextQuery {
        TestContextQuery::new(self, QueryType::Contract)
    }

    pub fn get_tokenfactory_denom(&self) -> TestContextQuery {
        TestContextQuery::new(self, QueryType::TokenfactoryDenom)
    }

    pub fn get_price_oracle(&self) -> TestContextQuery {
        TestContextQuery::new(self, QueryType::PriceOracle).contract(PRICE_ORACLE_NAME)
    }

    pub fn get_auction(&self) -> TestContextQuery {
        TestContextQuery::new(self, QueryType::Auction).contract(AUCTION_CONTRACT_NAME)
    }

    pub fn get_factory(&self) -> TestContextQuery {
        TestContextQuery::new(self, QueryType::Factory).contract(FACTORY_NAME)
    }

    pub fn get_astro_pool(&self) -> TestContextQuery {
        TestContextQuery::new(self, QueryType::AstroPool)
    }

    pub fn get_osmo_pool(&self) -> TestContextQuery {
        TestContextQuery::new(self, QueryType::OsmoPool)
    }

    pub fn get_auctions_manager(&self) -> TestContextQuery {
        TestContextQuery::new(self, QueryType::AuctionsManager)
            .contract(AUCTIONS_MANAGER_CONTRACT_NAME)
    }

    pub fn get_chain(&self, chain_name: &str) -> &LocalChain {
        self.chains.get(chain_name).unwrap()
    }

    pub fn get_mut_chain(&mut self, chain_name: &str) -> &mut LocalChain {
        self.chains.get_mut(chain_name).unwrap()
    }

    /// Gets the event log of a transaction as a JSON object,
    /// or returns an error if it does not exist.
    pub fn guard_tx_errors(&self, chain_name: &str, hash: &str) -> Result<(), Error> {
        if !self.unwrap_logs {
            return Ok(());
        }

        let chain = self.get_chain(chain_name);
        let mut logs = None;

        for _ in 0..TX_HASH_QUERY_RETRIES {
            thread::sleep(Duration::from_secs(TX_HASH_QUERY_PAUSE_SEC));

            let mut tx_res = chain.rb.query_tx_hash(hash);

            if tx_res.get("raw_log").is_none() {
                continue;
            }

            logs = Some(tx_res["raw_log"].take());

            break;
        }

        let raw_log = logs.as_ref().and_then(|raw_log| raw_log.as_str()).unwrap();

        if raw_log.is_empty() {
            return Ok(());
        }

        let logs = serde_json::from_str::<Value>(raw_log).map_err(|_| Error::TxFailed {
            hash: hash.to_owned(),
            error: raw_log.to_owned(),
        })?;

        if let Some(err) = logs.as_str() {
            return Err(Error::TxFailed {
                hash: hash.to_owned(),
                error: err.to_owned(),
            });
        }

        Ok(())
    }
}
