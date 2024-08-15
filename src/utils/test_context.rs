use super::super::{
    error::Error,
    types::{config::ConfigChain, contract::DeployedContractInfo, ibc::Channel as QueryChannel},
    LOCAL_IC_API_URL, TRANSFER_PORT,
};

use localic_std::{
    modules::cosmwasm::CosmWasm,
    node::Chain,
    relayer::{Channel, Relayer},
    transactions::ChainRequestBuilder,
};
use std::{collections::HashMap, path::PathBuf};

/// A configurable builder that can be used to create a TestContext.
pub struct TestContextBuilder {
    chains: Vec<ConfigChain>,
    api_url: Option<String>,
    transfer_channel_ids: HashMap<(String, String), String>,
    ccv_channel_ids: HashMap<(String, String), String>,
    connection_ids: HashMap<(String, String), String>,
    ibc_denoms: HashMap<(String, String), String>,
    artifacts_dir: Option<String>,
    unwrap_raw_logs: bool,
    transfer_channels: Vec<(String, String)>,
    ccv_channels: Vec<(String, String)>,
}

impl Default for TestContextBuilder {
    fn default() -> Self {
        Self {
            chains: Default::default(),
            api_url: Some(LOCAL_IC_API_URL.to_owned()),
            transfer_channel_ids: Default::default(),
            ccv_channel_ids: Default::default(),
            connection_ids: Default::default(),
            ibc_denoms: Default::default(),
            artifacts_dir: Default::default(),
            unwrap_raw_logs: Default::default(),
            transfer_channels: Default::default(),
            ccv_channels: Default::default(),
        }
    }
}

impl TestContextBuilder {
    /// Resets the chains that this builder will create a context for to the specified value.
    pub fn with_chains(&mut self, chains: impl Into<Vec<ConfigChain>>) -> &mut Self {
        self.chains = chains.into();

        self
    }

    /// Adds the specified chain to the context built by the builder.
    pub fn with_chain(&mut self, chain: ConfigChain) -> &mut Self {
        self.chains.push(chain);

        self
    }

    /// Sets the local-ic endpoint that the built context will interact with.
    pub fn with_api_url(&mut self, api_url: impl Into<String>) -> &mut Self {
        self.api_url = Some(api_url.into());

        self
    }

    /// Sets the transfer channel ID map to the specified map.
    pub fn with_transfer_channel_ids(
        &mut self,
        ids: impl Into<HashMap<(String, String), String>>,
    ) -> &mut Self {
        self.transfer_channel_ids = ids.into();

        self
    }

    /// Inserts a given channel ID for a chain pair into the builder.
    pub fn with_transfer_channel_id(
        &mut self,
        chain_a: impl Into<String>,
        chain_b: impl Into<String>,
        channel_id: impl Into<String>,
    ) -> &mut Self {
        self.transfer_channel_ids
            .insert((chain_a.into(), chain_b.into()), channel_id.into());

        self
    }

    /// Inserts a transfer channel between the specified chains in both directions.
    pub fn with_transfer_channels(
        &mut self,
        chain_a: impl Into<String> + std::marker::Copy,
        chain_b: impl Into<String> + std::marker::Copy,
    ) -> &mut Self {
        self.transfer_channels
            .push((chain_a.into(), chain_b.into()));

        self.transfer_channels
            .push((chain_b.into(), chain_a.into()));

        self
    }

    // Inserts a ccv channel b etween the specified chains in both directions.
    pub fn with_ccv_channels(
        &mut self,
        chain_a: impl Into<String> + std::marker::Copy,
        chain_b: impl Into<String> + std::marker::Copy,
    ) -> &mut Self {
        self.ccv_channels.push((chain_a.into(), chain_b.into()));

        self.ccv_channels.push((chain_b.into(), chain_a.into()));

        self
    }

    /// Sets the transfer channel ID map to the specified map.
    pub fn with_ccv_channel_ids(
        &mut self,
        ids: impl Into<HashMap<(String, String), String>>,
    ) -> &mut Self {
        self.ccv_channel_ids = ids.into();

        self
    }

    /// Inserts a given channel ID for a chain pair into the builder.
    pub fn with_ccv_channel_id(
        &mut self,
        chain_a: impl Into<String>,
        chain_b: impl Into<String>,
        channel_id: impl Into<String>,
    ) -> &mut Self {
        self.ccv_channel_ids
            .insert((chain_a.into(), chain_b.into()), channel_id.into());

        self
    }

    /// Sets the connection ID map to the specified map.
    pub fn with_connection_ids(
        &mut self,
        ids: impl Into<HashMap<(String, String), String>>,
    ) -> &mut Self {
        self.connection_ids = ids.into();

        self
    }

    /// Inserts a given connection ID for a chain pair into the builder.
    pub fn with_connection_id(
        &mut self,
        chain_a: impl Into<String>,
        chain_b: impl Into<String>,
        channel_id: impl Into<String>,
    ) -> &mut Self {
        self.connection_ids
            .insert((chain_a.into(), chain_b.into()), channel_id.into());

        self
    }

    /// Sets the IBC denom map to the specified map.
    pub fn with_ibc_denoms(
        &mut self,
        denoms: impl Into<HashMap<(String, String), String>>,
    ) -> &mut Self {
        self.ibc_denoms = denoms.into();

        self
    }

    /// Inserts a given connection ID for a chain pair into the builder.
    pub fn with_ibc_denom(
        &mut self,
        chain_a: impl Into<String>,
        chain_b: impl Into<String>,
        denom: impl Into<String>,
    ) -> &mut Self {
        self.ibc_denoms
            .insert((chain_a.into(), chain_b.into()), denom.into());

        self
    }

    /// Sets the artifacts dir to the specified directory.
    pub fn with_artifacts_dir(&mut self, dir: impl Into<String>) -> &mut Self {
        self.artifacts_dir = Some(dir.into());

        self
    }

    /// Sets the artifacts dir to the specified directory.
    pub fn with_unwrap_raw_logs(&mut self, unwrap_logs: bool) -> &mut Self {
        self.unwrap_raw_logs = unwrap_logs;

        self
    }

    /// Builds a TestContext from the specified options.
    pub fn build(&self) -> Result<TestContext, Error> {
        let TestContextBuilder {
            chains,
            transfer_channel_ids,
            ccv_channel_ids,
            connection_ids,
            ibc_denoms,
            api_url,
            artifacts_dir,
            unwrap_raw_logs,
            transfer_channels,
            ccv_channels,
        } = self;

        // Upload contract artifacts

        /// Deploys all neutron contracts to the test context.
        fn config_chain_to_local_chain(
            c: ConfigChain,
            api_url: String,
        ) -> Result<LocalChain, Error> {
            let rb = ChainRequestBuilder::new(api_url.to_owned(), c.chain_id.clone(), c.debugging)?;

            let relayer = Relayer::new(&rb);
            let channels = relayer.get_channels(&rb.chain_id)?;

            Ok(LocalChain::new(
                rb,
                c.admin_addr,
                c.denom,
                channels,
                c.chain_name,
                c.chain_prefix,
            ))
        }

        let chains_res: Result<HashMap<String, LocalChain>, Error> = chains
            .clone()
            .into_iter()
            .map(|builder| {
                config_chain_to_local_chain(
                    builder,
                    api_url
                        .clone()
                        .ok_or(Error::MissingBuilderParam(String::from("api_url")))?,
                )
            })
            .try_fold(HashMap::new(), |acc, x| {
                let x = x?;
                let mut acc = acc;

                acc.insert(x.chain_name.clone(), x);

                Ok(acc)
            });
        let chains = chains_res?;

        let mut transfer_channel_ids = transfer_channel_ids.clone();
        let mut connection_ids = connection_ids.clone();

        for (chain_a, chain_b) in transfer_channels {
            let chain_a_chain = chains
                .get(chain_a)
                .ok_or(Error::MissingBuilderParam(String::from("chain")))?;
            let chain_b_chain = chains
                .get(chain_b)
                .ok_or(Error::MissingBuilderParam(String::from("chain")))?;

            let conns = find_pairwise_transfer_channel_ids(
                &chain_a_chain.rb,
                &chain_a_chain.rb.chain_id,
                &chain_b_chain.rb.chain_id,
            )?;

            transfer_channel_ids
                .insert((chain_a.clone(), chain_b.clone()), conns.channel_id.clone());
            connection_ids.insert((chain_a.clone(), chain_b.clone()), conns.connection_id);

            let conns = find_pairwise_transfer_channel_ids(
                &chain_b_chain.rb,
                &chain_b_chain.rb.chain_id,
                &chain_a_chain.rb.chain_id,
            )?;

            transfer_channel_ids
                .insert((chain_b.clone(), chain_a.clone()), conns.channel_id.clone());
            connection_ids.insert((chain_b.clone(), chain_a.clone()), conns.connection_id);
        }

        let mut ccv_channel_ids = ccv_channel_ids.clone();

        for (chain_a, chain_b) in ccv_channels {
            let chain_a_chain = chains
                .get(chain_a)
                .ok_or(Error::MissingBuilderParam(String::from("chain")))?;
            let chain_b_chain = chains
                .get(chain_b)
                .ok_or(Error::MissingBuilderParam(String::from("chain")))?;

            let conns =
                find_pairwise_ccv_channel_ids(&chain_a_chain.channels, &chain_b_chain.channels)?;

            ccv_channel_ids.insert(
                (chain_a.clone(), chain_b.clone()),
                conns.0.channel_id.clone(),
            );
            ccv_channel_ids.insert(
                (chain_b.clone(), chain_a.clone()),
                conns.1.channel_id.clone(),
            );
        }

        Ok(TestContext {
            chains,
            transfer_channel_ids,
            ccv_channel_ids: ccv_channel_ids.clone(),
            connection_ids: connection_ids.clone(),
            ibc_denoms: ibc_denoms.clone(),
            artifacts_dir: artifacts_dir
                .clone()
                .ok_or(Error::MissingBuilderParam(String::from("artifacts_dir")))?,
            auctions_manager: None,
            astroport_token_registry: None,
            astroport_factory: None,
            unwrap_logs: *unwrap_raw_logs,
        })
    }
}

pub struct TestContext {
    pub chains: HashMap<String, LocalChain>,
    // maps (src_chain_id, dest_chain_id) to transfer channel id
    pub transfer_channel_ids: HashMap<(String, String), String>,
    // maps (src_chain_id, dest_chain_id) to ccv channel id
    pub ccv_channel_ids: HashMap<(String, String), String>,
    // maps (src_chain_id, dest_chain_id) to connection id
    pub connection_ids: HashMap<(String, String), String>,
    // maps (src_chain_id, dest_chain_id) to src chain native
    // denom -> ibc denom on dest chain
    pub ibc_denoms: HashMap<(String, String), String>,
    /// The path to .wasm contract artifacts
    pub artifacts_dir: String,

    /// Valence deployment info
    pub auctions_manager: Option<DeployedContractInfo>,

    /// Astroport deployment info
    pub astroport_token_registry: Option<DeployedContractInfo>,
    pub astroport_factory: Option<DeployedContractInfo>,

    /// Whether or not logs should be expected and guarded for each tx
    pub unwrap_logs: bool,
}

pub struct LocalChain {
    /// ChainRequestBuilder
    pub rb: ChainRequestBuilder,
    /// contract codes stored on this chain (filename -> code_id)
    pub contract_codes: HashMap<String, u64>,
    /// outgoing channel ids
    pub channels: Vec<Channel>,
    /// outgoing connection ids available (dest_chain_id -> connection_id)
    pub connection_ids: HashMap<String, String>,
    pub admin_addr: String,
    pub native_denom: String,
    /// contract address for the deployed instance of a contract
    pub contract_addrs: HashMap<String, String>,
    /// The name of the chain
    pub chain_name: String,
    pub chain_prefix: String,
}

impl LocalChain {
    pub fn new(
        rb: ChainRequestBuilder,
        admin_addr: String,
        native_denom: String,
        channels: Vec<Channel>,
        chain_name: String,
        chain_prefix: String,
    ) -> Self {
        Self {
            rb,
            contract_codes: Default::default(),
            channels,
            connection_ids: Default::default(),
            admin_addr,
            native_denom,
            contract_addrs: Default::default(),
            chain_name,
            chain_prefix,
        }
    }

    pub fn get_cw(&mut self) -> CosmWasm {
        CosmWasm::new(&self.rb)
    }

    pub fn save_code(&mut self, abs_path: PathBuf, code: u64) {
        let id = abs_path.file_stem().unwrap().to_str().unwrap();
        self.contract_codes.insert(id.to_string(), code);
    }

    pub fn wait_for_blocks(&self, blocks: u64) {
        let chain = Chain::new(&self.rb);

        while chain.get_height() < current_height + blocks {
            std::thread::sleep(std::time::Duration::from_millis(500));
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

    pub fn get_ibc_denoms(&self) -> TestContextQuery {
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

    pub fn get_chain(&self, chain_name: &str) -> &LocalChain {
        self.chains.get(chain_name).unwrap()
    }

    pub fn get_mut_chain(&mut self, chain_name: &str) -> &mut LocalChain {
        self.chains.get_mut(chain_name).unwrap()
    }
}

pub enum QueryType {
    TransferChannel,
    Connection,
    CCVChannel,
    IBCDenom,
    AdminAddr,
    NativeDenom,
    ChainPrefix,
    RequestBuilder,
}

pub struct TestContextQuery<'a> {
    context: &'a TestContext,
    query_type: QueryType,
    src_chain: Option<String>,
    dest_chain: Option<String>,
    contract_name: Option<String>,
}

impl<'a> TestContextQuery<'a> {
    pub fn new(context: &'a TestContext, query_type: QueryType) -> Self {
        Self {
            context,
            query_type,
            src_chain: None,
            dest_chain: None,
            contract_name: None,
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

    pub fn get(self) -> String {
        let query_response = match self.query_type {
            QueryType::TransferChannel => self.get_transfer_channel(),
            QueryType::Connection => self.get_connection_id(),
            QueryType::CCVChannel => self.get_ccv_channel(),
            QueryType::IBCDenom => self.get_ibc_denom(),
            QueryType::AdminAddr => self.get_admin_addr(),
            QueryType::NativeDenom => self.get_native_denom(),
            QueryType::ChainPrefix => self.get_chain_prefix(),
            _ => None,
        };
        query_response.unwrap()
    }

    pub fn get_all(self) -> Vec<String> {
        match self.query_type {
            QueryType::TransferChannel => self.get_all_transfer_channels(),
            QueryType::Connection => self.get_all_connections(),
            _ => vec![],
        }
    }

    pub fn get_request_builder(mut self, chain: &str) -> &'a ChainRequestBuilder {
        self.src_chain = Some(chain.to_string());
        let rb = match self.query_type {
            QueryType::RequestBuilder => self.get_rb(),
            _ => None,
        };
        rb.unwrap()
    }

    fn get_transfer_channel(self) -> Option<String> {
        if let (Some(ref src), Some(ref dest)) = (self.src_chain, self.dest_chain) {
            self.context
                .transfer_channel_ids
                .get(&(src.clone(), dest.clone()))
                .cloned()
        } else {
            None
        }
    }

    fn get_all_transfer_channels(self) -> Vec<String> {
        if let Some(ref src) = self.src_chain {
            self.context
                .transfer_channel_ids
                .iter()
                .filter(|((s, _), _)| s == src)
                .map(|(_, v)| v.clone())
                .collect::<Vec<_>>()
        } else {
            vec![]
        }
    }

    fn get_connection_id(self) -> Option<String> {
        if let (Some(ref src), Some(ref dest)) = (self.src_chain, self.dest_chain) {
            self.context
                .connection_ids
                .get(&(src.clone(), dest.clone()))
                .cloned()
        } else {
            None
        }
    }

    fn get_all_connections(self) -> Vec<String> {
        if let Some(ref src) = self.src_chain {
            self.context
                .connection_ids
                .iter()
                .filter(|((s, _), _)| s == src)
                .map(|(_, v)| v.clone())
                .collect::<Vec<_>>()
        } else {
            vec![]
        }
    }

    fn get_ccv_channel(self) -> Option<String> {
        if let (Some(ref src), Some(ref dest)) = (self.src_chain, self.dest_chain) {
            self.context
                .ccv_channel_ids
                .get(&(src.clone(), dest.clone()))
                .cloned()
        } else {
            None
        }
    }

    fn get_ibc_denom(self) -> Option<String> {
        if let (Some(ref src), Some(ref dest)) = (self.src_chain, self.dest_chain) {
            self.context
                .ibc_denoms
                .get(&(src.clone(), dest.clone()))
                .cloned()
        } else {
            None
        }
    }

    fn get_admin_addr(self) -> Option<String> {
        if let Some(ref src) = self.src_chain {
            self.context
                .chains
                .get(src)
                .map(|chain| chain.admin_addr.clone())
        } else {
            None
        }
    }

    fn get_native_denom(self) -> Option<String> {
        if let Some(ref src) = self.src_chain {
            self.context
                .chains
                .get(src)
                .map(|chain| chain.native_denom.clone())
        } else {
            None
        }
    }

    fn get_chain_prefix(self) -> Option<String> {
        if let Some(ref src) = self.src_chain {
            self.context
                .chains
                .get(src)
                .map(|chain| chain.chain_prefix.clone())
        } else {
            None
        }
    }

    fn get_rb(self) -> Option<&'a ChainRequestBuilder> {
        if let Some(ref src) = self.src_chain {
            self.context.chains.get(src).map(|chain| &chain.rb)
        } else {
            None
        }
    }
}

pub fn find_pairwise_transfer_channel_ids(
    rb: &ChainRequestBuilder,
    src_chain_id: &str,
    dest_chain_id: &str,
) -> Result<PairwiseChannelResult, Error> {
    let relayer = Relayer::new(rb);
    let cmd = format!("rly q channels {src_chain_id} {dest_chain_id}");
    let result = relayer.execute(cmd.as_str(), true).unwrap();
    let json_string = result["text"].as_str().unwrap();
    let channels = json_string
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(serde_json::from_str)
        .collect::<Result<Vec<QueryChannel>, _>>()?;

    for channel in channels {
        if channel.port_id == TRANSFER_PORT {
            let party_channel = PairwiseChannelResult {
                index: 0,
                channel_id: channel.channel_id.to_string(),
                connection_id: channel.connection_hops[0].to_string(),
            };
            return Ok(party_channel);
        }
    }

    Err(Error::MissingContextVariable(format!(
        "channel_ids::{src_chain_id}-{dest_chain_id}"
    )))
}

pub fn find_pairwise_ccv_channel_ids(
    provider_channels: &[Channel],
    consumer_channels: &[Channel],
) -> Result<(PairwiseChannelResult, PairwiseChannelResult), Error> {
    for (a_i, a_chan) in provider_channels.iter().enumerate() {
        for (b_i, b_chan) in consumer_channels.iter().enumerate() {
            if a_chan.channel_id == b_chan.counterparty.channel_id
                && b_chan.channel_id == a_chan.counterparty.channel_id
                && a_chan.port_id == "provider"
                && b_chan.port_id == "consumer"
                && a_chan.ordering == "ORDER_ORDERED"
                && b_chan.ordering == "ORDER_ORDERED"
            {
                let provider_channel_result = PairwiseChannelResult {
                    index: a_i,
                    channel_id: a_chan.channel_id.to_string(),
                    connection_id: a_chan.connection_hops[0].to_string(),
                };
                let consumer_channel_result = PairwiseChannelResult {
                    index: b_i,
                    channel_id: b_chan.channel_id.to_string(),
                    connection_id: b_chan.connection_hops[0].to_string(),
                };
                return Ok((provider_channel_result, consumer_channel_result));
            }
        }
    }
    Err(Error::MissingContextVariable(
        "Failed to match ccv channels".to_string(),
    ))
}

pub struct PairwiseChannelResult {
    pub index: usize,
    pub channel_id: String,
    pub connection_id: String,
}
