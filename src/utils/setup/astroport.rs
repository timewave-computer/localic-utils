use super::super::{
    super::{
        error::Error, CW1_WHITELIST_NAME, DEFAULT_KEY, FACTORY_NAME, FACTORY_ON_OSMOSIS_NAME,
        NEUTRON_CHAIN_ADMIN_ADDR, NEUTRON_CHAIN_NAME, OSMOSIS_CHAIN_NAME,
        OSMOSIS_PCL_POOL_TYPE_NAME, PAIR_NAME, PAIR_PCL_ON_OSMOSIS_NAME, STABLE_PAIR_NAME,
        TOKEN_NAME, TOKEN_REGISTRY_NAME, WHITELIST_NAME,
    },
    test_context::TestContext,
};
use astroport::{
    asset::{Asset, AssetInfo},
    factory::{self, PairConfig, PairType},
    native_coin_registry, pair,
};
use cosmwasm_std::Decimal;

/// A tx creating a token registry.
pub struct CreateTokenRegistryTxBuilder<'a> {
    key: Option<&'a str>,
    owner: Option<String>,
    chain: &'a str,
    test_ctx: &'a mut TestContext,
}

impl<'a> CreateTokenRegistryTxBuilder<'a> {
    pub fn with_key(&mut self, key: &'a str) -> &mut Self {
        self.key = Some(key);

        self
    }

    pub fn with_chain(&mut self, chain: &'a str) -> &mut Self {
        self.chain = chain;

        self
    }

    pub fn with_owner(&mut self, owner: impl Into<String>) -> &mut Self {
        self.owner = Some(owner.into());

        self
    }

    /// Sends the transaction.
    pub fn send(&mut self) -> Result<(), Error> {
        self.test_ctx.tx_create_token_registry(
            self.key
                .ok_or(Error::MissingBuilderParam(String::from("key")))?,
            self.chain,
            self.owner
                .clone()
                .ok_or(Error::MissingBuilderParam(String::from("owner")))?,
        )
    }
}

/// A tx creating a token registry.
pub struct CreatePoolTxBuilder<'a> {
    key: &'a str,
    chain: &'a str,
    pair_type: PairType,
    denom_a: Option<String>,
    denom_b: Option<String>,
    test_ctx: &'a mut TestContext,
}

impl<'a> CreatePoolTxBuilder<'a> {
    pub fn with_key(&mut self, key: &'a str) -> &mut Self {
        self.key = key;

        self
    }

    pub fn with_chain(&mut self, chain: &'a str) -> &mut Self {
        self.chain = chain;

        self
    }

    pub fn with_pairtype(&mut self, pairtype: PairType) -> &mut Self {
        self.pair_type = pairtype;

        self
    }

    pub fn with_denom_a(&mut self, denom_a: impl Into<String>) -> &mut Self {
        self.denom_a = Some(denom_a.into());

        self
    }

    pub fn with_denom_b(&mut self, denom_b: impl Into<String>) -> &mut Self {
        self.denom_b = Some(denom_b.into());

        self
    }

    /// Sends the transaction.
    pub fn send(&mut self) -> Result<(), Error> {
        self.test_ctx.tx_create_pool(
            self.key,
            self.chain,
            self.pair_type.clone(),
            self.denom_a
                .clone()
                .ok_or(Error::MissingBuilderParam(String::from("denom_a")))?,
            self.denom_b
                .clone()
                .ok_or(Error::MissingBuilderParam(String::from("denom_b")))?,
        )
    }
}

/// A tx creating an astroport factory.
pub struct CreateFactoryTxBuilder<'a> {
    key: &'a str,
    owner: String,
    chain: &'a str,
    test_ctx: &'a mut TestContext,
}

impl<'a> CreateFactoryTxBuilder<'a> {
    pub fn with_key(&mut self, key: &'a str) -> &mut Self {
        self.key = key;

        self
    }

    pub fn with_owner(&mut self, owner: impl Into<String>) -> &mut Self {
        self.owner = owner.into();

        self
    }

    pub fn with_chain(&mut self, chain: &'a str) -> &mut Self {
        self.chain = chain;

        self
    }

    /// Sends the transaction.
    pub fn send(&mut self) -> Result<(), Error> {
        self.test_ctx
            .tx_create_factory(self.key, self.chain, self.owner.clone())
    }
}

/// A tx funding an astroport pool.
pub struct FundPoolTxBuilder<'a> {
    key: &'a str,
    chain: &'a str,
    denom_a: Option<String>,
    denom_b: Option<String>,
    amt_denom_a: Option<u128>,
    amt_denom_b: Option<u128>,
    slippage_tolerance: Option<Decimal>,
    liq_token_receiver: Option<&'a str>,
    test_ctx: &'a mut TestContext,
}

impl<'a> FundPoolTxBuilder<'a> {
    pub fn with_key(&mut self, key: &'a str) -> &mut Self {
        self.key = key;

        self
    }

    pub fn with_chain(&mut self, chain: &'a str) -> &mut Self {
        self.chain = chain;

        self
    }

    pub fn with_denom_a(&mut self, denom_a: impl Into<String>) -> &mut Self {
        self.denom_a = Some(denom_a.into());

        self
    }

    pub fn with_denom_b(&mut self, denom_b: impl Into<String>) -> &mut Self {
        self.denom_b = Some(denom_b.into());

        self
    }

    pub fn with_amount_denom_a(&mut self, amt_denom_a: u128) -> &mut Self {
        self.amt_denom_a = Some(amt_denom_a);

        self
    }

    pub fn with_amount_denom_b(&mut self, amt_denom_b: u128) -> &mut Self {
        self.amt_denom_b = Some(amt_denom_b);

        self
    }

    pub fn with_slippage_tolerance(&mut self, slippage_tolerance: Decimal) -> &mut Self {
        self.slippage_tolerance = Some(slippage_tolerance);

        self
    }

    pub fn with_liq_token_receiver(&mut self, receiver_addr: &'a str) -> &mut Self {
        self.liq_token_receiver = Some(receiver_addr);

        self
    }

    /// Sends the transaction.
    pub fn send(&mut self) -> Result<(), Error> {
        self.test_ctx.tx_fund_pool(
            self.key,
            self.chain,
            self.denom_a
                .clone()
                .ok_or(Error::MissingBuilderParam(String::from("denom_a")))?,
            self.denom_b
                .clone()
                .ok_or(Error::MissingBuilderParam(String::from("denom_b")))?,
            self.amt_denom_a
                .ok_or(Error::MissingBuilderParam(String::from("amt_denom_a")))?,
            self.amt_denom_b
                .ok_or(Error::MissingBuilderParam(String::from("amt_denom_b")))?,
            self.slippage_tolerance
                .ok_or(Error::MissingBuilderParam(String::from(
                    "slippage_tolerance",
                )))?,
            self.liq_token_receiver
                .ok_or(Error::MissingBuilderParam(String::from(
                    "liq_token_receiver",
                )))?,
        )
    }
}

impl TestContext {
    pub fn build_tx_create_token_registry(&mut self) -> CreateTokenRegistryTxBuilder {
        CreateTokenRegistryTxBuilder {
            key: Some(DEFAULT_KEY),
            owner: Some(NEUTRON_CHAIN_ADMIN_ADDR.to_owned()),
            chain: NEUTRON_CHAIN_NAME,
            test_ctx: self,
        }
    }

    /// Instantiates the token registry.
    fn tx_create_token_registry(
        &mut self,
        key: &str,
        chain_name: &str,
        owner_addr: impl Into<String>,
    ) -> Result<(), Error> {
        let native_denom = self.get_native_denom().src(chain_name).get().clone();

        let mut contract_a = self
            .get_contract()
            .src(chain_name)
            .contract(TOKEN_REGISTRY_NAME)
            .get_cw();

        let contract = contract_a.instantiate(
            key,
            serde_json::to_string(&native_coin_registry::InstantiateMsg {
                owner: owner_addr.into(),
            })?
            .as_str(),
            TOKEN_REGISTRY_NAME,
            None,
            &format!("--gas 1000000 --fees 42069420{native_denom}"),
        )?;
        let addr = contract.address;

        let chain = self.get_mut_chain(chain_name);

        chain
            .contract_addrs
            .insert(TOKEN_REGISTRY_NAME.to_owned(), addr.clone());

        Ok(())
    }

    /// Instantiates the astroport factory.
    pub fn build_tx_create_factory(&mut self) -> CreateFactoryTxBuilder {
        CreateFactoryTxBuilder {
            key: DEFAULT_KEY,
            owner: NEUTRON_CHAIN_ADMIN_ADDR.to_owned(),
            chain: NEUTRON_CHAIN_NAME,
            test_ctx: self,
        }
    }

    /// Instantiates the astroport factory.
    fn tx_create_factory(
        &mut self,
        key: &str,
        chain_name: &str,
        factory_owner: impl Into<String>,
    ) -> Result<(), Error> {
        if chain_name == OSMOSIS_CHAIN_NAME {
            // Osmosis setup should be handled differently with astroport-on-osmosis contract:s
            return self.tx_create_factory_osmo(key, factory_owner);
        }

        // Assume neutron, or some neutron-capable chain, otherwise
        // bubble-up missing capabilities error to the user
        let chain = self.get_chain(chain_name);

        let pair_xyk_code_id =
            chain
                .contract_codes
                .get(PAIR_NAME)
                .ok_or(Error::MissingContextVariable(String::from(
                    "contract_codes::astroport_pair",
                )))?;
        let pair_stable_code_id =
            chain
                .contract_codes
                .get(STABLE_PAIR_NAME)
                .ok_or(Error::MissingContextVariable(String::from(
                    "contract_codes::astroport_pair_stable",
                )))?;
        let token_code_id =
            chain
                .contract_codes
                .get(TOKEN_NAME)
                .ok_or(Error::MissingContextVariable(String::from(
                    "contract_codes::cw20_base",
                )))?;
        let whitelist_code_id =
            chain
                .contract_codes
                .get(WHITELIST_NAME)
                .ok_or(Error::MissingContextVariable(String::from(
                    "contract_codes::astroport_whitelist",
                )))?;

        let native_registry_addr =
            chain
                .contract_addrs
                .get(TOKEN_REGISTRY_NAME)
                .ok_or(Error::MissingContextVariable(String::from(
                    "contract_ddrs::astroport_native_coin_registry",
                )))?;

        let mut contract_a = self
            .get_contract()
            .src(chain_name)
            .contract(FACTORY_NAME)
            .get_cw();

        let contract = contract_a.instantiate(
            key,
            serde_json::to_string(&factory::InstantiateMsg {
                pair_configs: vec![
                    PairConfig {
                        code_id: *pair_xyk_code_id,
                        pair_type: PairType::Xyk {},
                        total_fee_bps: 100,
                        maker_fee_bps: 10,
                        is_disabled: false,
                        is_generator_disabled: false,
                        permissioned: false,
                    },
                    PairConfig {
                        code_id: *pair_stable_code_id,
                        pair_type: PairType::Stable {},
                        total_fee_bps: 100,
                        maker_fee_bps: 10,
                        is_disabled: false,
                        is_generator_disabled: false,
                        permissioned: false,
                    },
                ],
                token_code_id: *token_code_id,
                owner: factory_owner.into(),
                whitelist_code_id: *whitelist_code_id,
                coin_registry_address: native_registry_addr.clone(),
                fee_address: None,
                generator_address: None,
                tracker_config: None,
            })?
            .as_str(),
            FACTORY_NAME,
            None,
            "",
        )?;

        let chain = self.get_mut_chain(chain_name);

        chain
            .contract_addrs
            .insert(FACTORY_NAME.to_owned(), contract.address);

        Ok(())
    }

    fn tx_create_factory_osmo(
        &mut self,
        key: &str,
        factory_owner: impl Into<String>,
    ) -> Result<(), Error> {
        let chain = self.get_chain(OSMOSIS_CHAIN_NAME);

        // Pcl contract code ID for a custom pool
        let pair_pcl_code_id = self
            .get_contract()
            .contract(PAIR_PCL_ON_OSMOSIS_NAME)
            .src(OSMOSIS_CHAIN_NAME)
            .get_cw()
            .code_id
            .unwrap();

        // Cw20 base code ID
        let token_code_id = self
            .get_contract()
            .contract(TOKEN_NAME)
            .src(OSMOSIS_CHAIN_NAME)
            .get_cw()
            .code_id
            .unwrap();

        // Don't use the astroport whitelist here, since it does not support osmosis
        // use the cw plus whitelist
        let whitelist_code_id = self
            .get_contract()
            .contract(CW1_WHITELIST_NAME)
            .src(OSMOSIS_CHAIN_NAME)
            .get_cw()
            .code_id
            .unwrap();

        // Instantiate the osmosis factory
        let mut contract_a = self
            .get_contract()
            .src(OSMOSIS_CHAIN_NAME)
            .contract(FACTORY_ON_OSMOSIS_NAME)
            .get_cw();

        // Get the deployed native coin registry
        let native_registry_addr =
            chain
                .contract_addrs
                .get(TOKEN_REGISTRY_NAME)
                .ok_or(Error::MissingContextVariable(String::from(
                    "contract_ddrs::astroport_native_coin_registry",
                )))?;

        println!("B: {:?}", pair_pcl_code_id);

        // Enable PCL (custom) pools only
        let contract = contract_a.instantiate(
            key,
            &serde_json::to_string(&serde_json::json!({
                "pair_configs": vec![PairConfig {
                    code_id: pair_pcl_code_id,
                    pair_type: PairType::Custom(String::from(OSMOSIS_PCL_POOL_TYPE_NAME)),
                    total_fee_bps: 100,
                    maker_fee_bps: 10,
                    is_disabled: false,
                    is_generator_disabled: false,
                    permissioned: false,
                }],
                "token_code_id": token_code_id,
                "owner": factory_owner.into(),
                "whitelist_code_id": whitelist_code_id,
                "coin_registry_address": native_registry_addr.clone(),
            }))
            .unwrap(),
            FACTORY_NAME,
            None,
            &format!("--gas 1000000 --fees 42069420uosmo"),
        )?;

        let chain = self.get_mut_chain(OSMOSIS_CHAIN_NAME);

        chain
            .contract_addrs
            .insert(FACTORY_ON_OSMOSIS_NAME.to_owned(), contract.address);

        Ok(())
    }

    /// Creates a pool with the specififed denoms.
    pub fn build_tx_create_pool(&mut self) -> CreatePoolTxBuilder {
        CreatePoolTxBuilder {
            key: DEFAULT_KEY,
            chain: NEUTRON_CHAIN_NAME,
            pair_type: PairType::Xyk {},
            denom_a: Default::default(),
            denom_b: Default::default(),
            test_ctx: self,
        }
    }

    /// Creates a pool with the specififed denoms.
    fn tx_create_pool(
        &self,
        key: &str,
        chain: &str,
        pair_type: PairType,
        denom_a: impl Into<String>,
        denom_b: impl Into<String>,
    ) -> Result<(), Error> {
        // Factory contract instance
        let contract_a = self.get_factory().src(chain).get_cw();
        let fee_denom = self.get_native_denom().src(chain).get();

        // Create the pair
        let tx = contract_a.execute(
            key,
            serde_json::to_string(&factory::ExecuteMsg::CreatePair {
                pair_type,
                asset_infos: vec![
                    AssetInfo::NativeToken {
                        denom: denom_a.into(),
                    },
                    AssetInfo::NativeToken {
                        denom: denom_b.into(),
                    },
                ],
                init_params: None,
            })?
            .as_str(),
            &format!("--fees 42069420{fee_denom} --gas 1000000"),
        )?;

        // Get the address of the createed contract via logs
        let tx_hash = tx.tx_hash.ok_or(Error::Misc(String::from(
            "transaction did not produce a tx hash",
        )))?;

        self.guard_tx_errors(chain, tx_hash.as_str())?;

        Ok(())
    }

    /// Provides liquidity for a specific astroport pool.
    pub fn build_tx_fund_pool(&mut self) -> FundPoolTxBuilder {
        FundPoolTxBuilder {
            key: DEFAULT_KEY,
            chain: NEUTRON_CHAIN_NAME,
            denom_a: Default::default(),
            denom_b: Default::default(),
            amt_denom_a: Default::default(),
            amt_denom_b: Default::default(),
            slippage_tolerance: Default::default(),
            liq_token_receiver: Default::default(),
            test_ctx: self,
        }
    }

    /// Provides liquidity for a specific astroport pool.
    #[allow(clippy::too_many_arguments)]
    fn tx_fund_pool(
        &mut self,
        key: &str,
        chain: &str,
        denom_a: String,
        denom_b: String,
        amt_denom_a: u128,
        amt_denom_b: u128,
        slippage_tolerance: Decimal,
        liq_token_receiver: impl Into<String>,
    ) -> Result<(), Error> {
        let fee_denom = self.get_native_denom().src(chain).get();

        // Get the instance from the address
        let pool = self
            .get_astro_pool()
            .src(chain)
            .denoms(denom_a.clone(), denom_b.clone())
            .get_cw();

        // Provide liquidity
        let tx = pool
            .execute(
                key,
                serde_json::to_string(&pair::ExecuteMsg::ProvideLiquidity {
                    assets: vec![
                        Asset {
                            info: AssetInfo::NativeToken {
                                denom: denom_a.clone(),
                            },
                            amount: amt_denom_a.into(),
                        },
                        Asset {
                            info: AssetInfo::NativeToken {
                                denom: denom_b.clone(),
                            },
                            amount: amt_denom_b.into(),
                        },
                    ],
                    slippage_tolerance: Some(slippage_tolerance),
                    auto_stake: None,
                    receiver: Some(liq_token_receiver.into()),
                    min_lp_to_receive: None,
                })?
                .as_str(),
                &format!("--amount {amt_denom_a}{denom_a},{amt_denom_b}{denom_b} --gas 1000000 --fees 42069420{fee_denom}"),
            )?
            .tx_hash
            .ok_or(Error::TxMissingLogs)?;

        self.guard_tx_errors(chain, tx.as_str())?;

        Ok(())
    }
}
