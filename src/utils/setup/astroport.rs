use super::super::{
    super::{
        error::Error, DEFAULT_KEY, FACTORY_NAME, NEUTRON_CHAIN_ADMIN_ADDR, NEUTRON_CHAIN_NAME,
        PAIR_NAME, STABLE_PAIR_NAME, TOKEN_NAME, TOKEN_REGISTRY_NAME, WHITELIST_NAME,
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
    test_ctx: &'a mut TestContext,
}

impl<'a> CreateTokenRegistryTxBuilder<'a> {
    pub fn with_key(&mut self, key: &'a str) -> &mut Self {
        self.key = Some(key);

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
            self.owner
                .clone()
                .ok_or(Error::MissingBuilderParam(String::from("owner")))?,
        )
    }
}

/// A tx creating a token registry.
pub struct CreatePoolTxBuilder<'a> {
    key: &'a str,
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

    /// Sends the transaction.
    pub fn send(&mut self) -> Result<(), Error> {
        self.test_ctx
            .tx_create_factory(self.key, self.owner.clone())
    }
}

/// A tx funding an astroport pool.
pub struct FundPoolTxBuilder<'a> {
    key: &'a str,
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
            test_ctx: self,
        }
    }

    /// Instantiates the token registry.
    fn tx_create_token_registry(
        &mut self,
        key: &str,
        owner_addr: impl Into<String>,
    ) -> Result<(), Error> {
        let mut contract_a = self
            .get_contract()
            .src(NEUTRON_CHAIN_NAME)
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
            "--gas 1000000",
        )?;
        let addr = contract.address;

        let neutron = self.get_mut_chain(NEUTRON_CHAIN_NAME);

        neutron
            .contract_addrs
            .insert(TOKEN_REGISTRY_NAME.to_owned(), addr.clone());

        Ok(())
    }

    /// Instantiates the astroport factory.
    pub fn build_tx_create_factory(&mut self) -> CreateFactoryTxBuilder {
        CreateFactoryTxBuilder {
            key: DEFAULT_KEY,
            owner: NEUTRON_CHAIN_ADMIN_ADDR.to_owned(),
            test_ctx: self,
        }
    }

    /// Instantiates the astroport factory.
    fn tx_create_factory(
        &mut self,
        key: &str,
        factory_owner: impl Into<String>,
    ) -> Result<(), Error> {
        let neutron = self.get_chain(NEUTRON_CHAIN_NAME);

        let pair_xyk_code_id =
            neutron
                .contract_codes
                .get(PAIR_NAME)
                .ok_or(Error::MissingContextVariable(String::from(
                    "contract_codes::astroport_pair",
                )))?;
        let pair_stable_code_id =
            neutron
                .contract_codes
                .get(STABLE_PAIR_NAME)
                .ok_or(Error::MissingContextVariable(String::from(
                    "contract_codes::astroport_pair_stable",
                )))?;
        let token_code_id =
            neutron
                .contract_codes
                .get(TOKEN_NAME)
                .ok_or(Error::MissingContextVariable(String::from(
                    "contract_codes::cw20_base",
                )))?;
        let whitelist_code_id =
            neutron
                .contract_codes
                .get(WHITELIST_NAME)
                .ok_or(Error::MissingContextVariable(String::from(
                    "contract_codes::astroport_whitelist",
                )))?;

        let native_registry_addr = neutron.contract_addrs.get(TOKEN_REGISTRY_NAME).ok_or(
            Error::MissingContextVariable(String::from(
                "contract_ddrs::astroport_native_coin_registry",
            )),
        )?;

        let mut contract_a = self
            .get_contract()
            .src(NEUTRON_CHAIN_NAME)
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

        let neutron = self.get_mut_chain(NEUTRON_CHAIN_NAME);

        neutron
            .contract_addrs
            .insert(FACTORY_NAME.to_owned(), contract.address);

        Ok(())
    }

    /// Creates a pool with the specififed denoms.
    pub fn build_tx_create_pool(&mut self) -> CreatePoolTxBuilder {
        CreatePoolTxBuilder {
            key: DEFAULT_KEY,
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
        pair_type: PairType,
        denom_a: impl Into<String>,
        denom_b: impl Into<String>,
    ) -> Result<(), Error> {
        // Factory contract instance
        let contract_a = self.get_factory().src(NEUTRON_CHAIN_NAME).get_cw();

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
            "--gas 1000000",
        )?;

        // Get the address of the createed contract via logs
        let tx_hash = tx.tx_hash.ok_or(Error::Misc(String::from(
            "transaction did not produce a tx hash",
        )))?;

        self.guard_tx_errors(NEUTRON_CHAIN_NAME, tx_hash.as_str())?;

        Ok(())
    }

    /// Provides liquidity for a specific astroport pool.
    pub fn build_tx_fund_pool(&mut self) -> FundPoolTxBuilder {
        FundPoolTxBuilder {
            key: DEFAULT_KEY,
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
        denom_a: String,
        denom_b: String,
        amt_denom_a: u128,
        amt_denom_b: u128,
        slippage_tolerance: Decimal,
        liq_token_receiver: impl Into<String>,
    ) -> Result<(), Error> {
        // Get the instance from the address
        let pool = self
            .get_astro_pool()
            .src(NEUTRON_CHAIN_NAME)
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
                &format!("--amount {amt_denom_a}{denom_a},{amt_denom_b}{denom_b} --gas 1000000"),
            )?
            .tx_hash
            .ok_or(Error::TxMissingLogs)?;

        self.guard_tx_errors(NEUTRON_CHAIN_NAME, tx.as_str())?;

        Ok(())
    }
}
