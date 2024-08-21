use super::super::{
    super::{
        error::Error,
        types::contract::{AuctionStrategy, ChainHaltConfig, MinAmount, PriceFreshnessStrategy},
        AUCTIONS_MANAGER_CONTRACT_NAME, AUCTION_CONTRACT_NAME, DEFAULT_AUCTION_LABEL, DEFAULT_KEY,
        NEUTRON_CHAIN_ADMIN_ADDR, NEUTRON_CHAIN_NAME, PRICE_ORACLE_NAME,
    },
    test_context::TestContext,
};
use cosmwasm_std::Decimal;
use localic_std::modules::cosmwasm::CosmWasm;
use serde_json::Value;

/// A tx creating an auctions manager.
pub struct CreateAuctionsManagerTxBuilder<'a> {
    key: &'a str,
    min_auction_amount: &'a [(&'a str, MinAmount)],
    server_addr: &'a str,
    test_ctx: &'a mut TestContext,
}

impl<'a> CreateAuctionsManagerTxBuilder<'a> {
    pub fn with_key(&mut self, key: &'a str) -> &mut Self {
        self.key = key;

        self
    }
    pub fn with_min_auction_amount(
        &mut self,
        min_auction_amount: &'a [(&'a str, MinAmount)],
    ) -> &mut Self {
        self.min_auction_amount = min_auction_amount;

        self
    }

    pub fn with_server_addr(&mut self, addr: &'a str) -> &mut Self {
        self.server_addr = addr;

        self
    }

    /// Sends the transaction.
    pub fn send(&mut self) -> Result<(), Error> {
        self.test_ctx.tx_create_auctions_manager(
            self.key,
            self.min_auction_amount,
            self.server_addr,
        )
    }
}

pub struct CreateAuctionTxBuilder<'a> {
    key: &'a str,
    offer_asset: Option<&'a str>,
    ask_asset: Option<&'a str>,
    auction_strategy: AuctionStrategy,
    chain_halt_config: ChainHaltConfig,
    price_freshness_strategy: PriceFreshnessStrategy,
    label: &'a str,
    amount_offer_asset: Option<u128>,
    test_ctx: &'a mut TestContext,
}

impl<'a> CreateAuctionTxBuilder<'a> {
    pub fn with_key(&mut self, key: &'a str) -> &mut Self {
        self.key = key;

        self
    }

    pub fn with_offer_asset(&mut self, asset: &'a str) -> &mut Self {
        self.offer_asset = Some(asset);

        self
    }

    pub fn with_ask_asset(&mut self, asset: &'a str) -> &mut Self {
        self.ask_asset = Some(asset);

        self
    }

    pub fn with_auction_strategy(&mut self, auction_strategy: AuctionStrategy) -> &mut Self {
        self.auction_strategy = auction_strategy;

        self
    }

    pub fn with_chain_halt_config(&mut self, chain_halt_config: ChainHaltConfig) -> &mut Self {
        self.chain_halt_config = chain_halt_config;

        self
    }

    pub fn with_price_freshness_strategy(
        &mut self,
        price_freshness_strategy: PriceFreshnessStrategy,
    ) -> &mut Self {
        self.price_freshness_strategy = price_freshness_strategy;

        self
    }

    pub fn with_label(&mut self, label: &'a str) -> &mut Self {
        self.label = label;

        self
    }

    pub fn with_amount_offer_asset(&mut self, amount_offer_asset: u128) -> &mut Self {
        self.amount_offer_asset = Some(amount_offer_asset);

        self
    }

    /// Sends the transaction.
    pub fn send(&mut self) -> Result<(), Error> {
        self.test_ctx.tx_create_auction(
            self.key,
            (
                self.offer_asset
                    .ok_or(Error::MissingBuilderParam(String::from("pair")))?,
                self.ask_asset
                    .ok_or(Error::MissingBuilderParam(String::from("pair")))?,
            ),
            self.auction_strategy.clone(),
            self.chain_halt_config.clone(),
            self.price_freshness_strategy.clone(),
            self.label,
            self.amount_offer_asset
                .ok_or(Error::MissingBuilderParam(String::from(
                    "amount_offer_asset",
                )))?,
        )
    }
}

pub struct FundAuctionTxBuilder<'a> {
    key: &'a str,
    offer_asset: Option<&'a str>,
    ask_asset: Option<&'a str>,
    amt_offer_asset: Option<u128>,
    test_ctx: &'a mut TestContext,
}

impl<'a> FundAuctionTxBuilder<'a> {
    pub fn with_key(&mut self, key: &'a str) -> &mut Self {
        self.key = key;

        self
    }

    pub fn with_offer_asset(&mut self, asset: &'a str) -> &mut Self {
        self.offer_asset = Some(asset);

        self
    }

    pub fn with_ask_asset(&mut self, asset: &'a str) -> &mut Self {
        self.ask_asset = Some(asset);

        self
    }

    pub fn with_amount_offer_asset(&mut self, amount_offer_asset: u128) -> &mut Self {
        self.amt_offer_asset = Some(amount_offer_asset);

        self
    }

    /// Sends the transaction.
    pub fn send(&mut self) -> Result<(), Error> {
        self.test_ctx.tx_fund_auction(
            self.key,
            (
                self.offer_asset
                    .ok_or(Error::MissingBuilderParam(String::from("pair")))?,
                self.ask_asset
                    .ok_or(Error::MissingBuilderParam(String::from("pair")))?,
            ),
            self.amt_offer_asset
                .ok_or(Error::MissingBuilderParam(String::from(
                    "amount_offer_asset",
                )))?,
        )
    }
}

pub struct StartAuctionTxBuilder<'a> {
    key: &'a str,
    offer_asset: Option<&'a str>,
    ask_asset: Option<&'a str>,
    end_block_delta: Option<u128>,
    test_ctx: &'a mut TestContext,
}

impl<'a> StartAuctionTxBuilder<'a> {
    pub fn with_key(&mut self, key: &'a str) -> &mut Self {
        self.key = key;

        self
    }

    pub fn with_offer_asset(&mut self, asset: &'a str) -> &mut Self {
        self.offer_asset = Some(asset);

        self
    }

    pub fn with_ask_asset(&mut self, asset: &'a str) -> &mut Self {
        self.ask_asset = Some(asset);

        self
    }

    pub fn with_end_block_delta(&mut self, delta_blocks: u128) -> &mut Self {
        self.end_block_delta = Some(delta_blocks);

        self
    }

    /// Sends the transaction.
    pub fn send(&mut self) -> Result<(), Error> {
        self.test_ctx.tx_start_auction(
            self.key,
            self.end_block_delta
                .ok_or(Error::MissingBuilderParam(String::from("end_block_delta")))?,
            (
                self.offer_asset
                    .ok_or(Error::MissingBuilderParam(String::from("pair")))?,
                self.ask_asset
                    .ok_or(Error::MissingBuilderParam(String::from("pair")))?,
            ),
        )
    }
}

pub struct MigrateAuctionTxBuilder<'a> {
    key: &'a str,
    offer_asset: Option<&'a str>,
    ask_asset: Option<&'a str>,
    test_ctx: &'a mut TestContext,
}

impl<'a> MigrateAuctionTxBuilder<'a> {
    pub fn with_key(&mut self, key: &'a str) -> &mut Self {
        self.key = key;

        self
    }

    pub fn with_offer_asset(&mut self, asset: &'a str) -> &mut Self {
        self.offer_asset = Some(asset);

        self
    }

    pub fn with_ask_asset(&mut self, asset: &'a str) -> &mut Self {
        self.ask_asset = Some(asset);

        self
    }

    /// Sends the transaction.
    pub fn send(&mut self) -> Result<(), Error> {
        self.test_ctx.tx_migrate_auction(
            self.key,
            (
                self.offer_asset
                    .ok_or(Error::MissingBuilderParam(String::from("pair")))?,
                self.ask_asset
                    .ok_or(Error::MissingBuilderParam(String::from("pair")))?,
            ),
        )
    }
}

pub struct CreatePriceOracleTxBuilder<'a> {
    key: &'a str,
    seconds_allow_manual_change: u64,
    seconds_auction_prices_fresh: u64,
    test_ctx: &'a mut TestContext,
}

impl<'a> CreatePriceOracleTxBuilder<'a> {
    pub fn with_key(&mut self, key: &'a str) -> &mut Self {
        self.key = key;

        self
    }

    pub fn with_seconds_allow_manual_change(&mut self, sec: u64) -> &mut Self {
        self.seconds_allow_manual_change = sec;

        self
    }

    pub fn with_seconds_auction_prices_fresh(&mut self, sec: u64) -> &mut Self {
        self.seconds_auction_prices_fresh = sec;

        self
    }

    /// Sends the transaction.
    pub fn send(&mut self) -> Result<(), Error> {
        self.test_ctx.tx_create_price_oracle(
            self.key,
            self.seconds_allow_manual_change,
            self.seconds_auction_prices_fresh,
        )
    }
}

pub struct UpdateAuctionOracleTxBuilder<'a> {
    key: &'a str,
    test_ctx: &'a mut TestContext,
}

impl<'a> UpdateAuctionOracleTxBuilder<'a> {
    pub fn with_key(&mut self, key: &'a str) -> &mut Self {
        self.key = key;

        self
    }

    /// Sends the transaction.
    pub fn send(&mut self) -> Result<(), Error> {
        self.test_ctx.tx_update_auction_oracle(self.key)
    }
}

pub struct ManualOraclePriceUpdateTxBuilder<'a> {
    key: &'a str,
    offer_asset: Option<&'a str>,
    ask_asset: Option<&'a str>,
    price: Option<Decimal>,
    test_ctx: &'a mut TestContext,
}

impl<'a> ManualOraclePriceUpdateTxBuilder<'a> {
    pub fn with_key(&mut self, key: &'a str) -> &mut Self {
        self.key = key;

        self
    }

    pub fn with_offer_asset(&mut self, asset: &'a str) -> &mut Self {
        self.offer_asset = Some(asset);

        self
    }

    pub fn with_ask_asset(&mut self, asset: &'a str) -> &mut Self {
        self.ask_asset = Some(asset);

        self
    }

    pub fn with_price(&mut self, price: Decimal) -> &mut Self {
        self.price = Some(price);

        self
    }

    /// Sends the transaction.
    pub fn send(&mut self) -> Result<(), Error> {
        self.test_ctx.tx_manual_oracle_price_update(
            self.key,
            self.offer_asset
                .ok_or(Error::MissingBuilderParam(String::from("offer_asset")))?,
            self.ask_asset
                .ok_or(Error::MissingBuilderParam(String::from("ask_asset")))?,
            self.price
                .ok_or(Error::MissingBuilderParam(String::from("price")))?,
        )
    }
}

impl TestContext {
    pub fn build_tx_create_auctions_manager(&mut self) -> CreateAuctionsManagerTxBuilder {
        CreateAuctionsManagerTxBuilder {
            key: DEFAULT_KEY,
            min_auction_amount: &[],
            server_addr: NEUTRON_CHAIN_ADMIN_ADDR,
            test_ctx: self,
        }
    }

    /// Creates an auction manager on Neutron, updating the autions manager
    /// code id and address in the TestContext.
    fn tx_create_auctions_manager<'a>(
        &mut self,
        sender_key: &str,
        min_auction_amount: impl AsRef<[(&'a str, MinAmount)]>,
        server_addr: impl AsRef<str>,
    ) -> Result<(), Error> {
        let mut contract_a: CosmWasm = self
            .get_contract()
            .contract(AUCTIONS_MANAGER_CONTRACT_NAME)
            .get_cw();
        let neutron = self.get_chain(NEUTRON_CHAIN_NAME);

        let auction_code_id =
            neutron
                .contract_codes
                .get(AUCTION_CONTRACT_NAME)
                .ok_or(Error::Misc(format!(
                    "contract '{AUCTION_CONTRACT_NAME}' is missing"
                )))?;

        let contract = contract_a.instantiate(
            sender_key,
            serde_json::json!({
                "auction_code_id": auction_code_id,
                "min_auction_amount": min_auction_amount.as_ref(),
                "server_addr": server_addr.as_ref(),
            })
            .to_string()
            .as_str(),
            AUCTIONS_MANAGER_CONTRACT_NAME,
            None,
            "",
        )?;

        let chain = self.get_mut_chain(NEUTRON_CHAIN_NAME);

        chain
            .contract_addrs
            .insert(AUCTIONS_MANAGER_CONTRACT_NAME.to_owned(), contract.address);

        Ok(())
    }

    pub fn build_tx_create_price_oracle(&mut self) -> CreatePriceOracleTxBuilder {
        CreatePriceOracleTxBuilder {
            key: DEFAULT_KEY,
            seconds_allow_manual_change: 0,
            seconds_auction_prices_fresh: 100000000000,
            test_ctx: self,
        }
    }

    /// Creates an auction manager on Neutron, updating the autions manager
    /// code id and address in the TestContext.
    fn tx_create_price_oracle(
        &mut self,
        sender_key: &str,
        seconds_allow_manual_change: u64,
        seconds_auction_prices_fresh: u64,
    ) -> Result<(), Error> {
        let auctions_manager: CosmWasm = self.get_auctions_manager().get_cw();
        let auctions_manager_addr =
            auctions_manager
                .contract_addr
                .ok_or(Error::MissingContextVariable(String::from(
                    "contract_addresses::auctions_manager",
                )))?;

        let mut contract_a = self.get_contract().contract(PRICE_ORACLE_NAME).get_cw();
        let contract = contract_a.instantiate(
            sender_key,
            serde_json::json!({
                "auctions_manager_addr": auctions_manager_addr,
                "seconds_allow_manual_change": seconds_allow_manual_change,
                "seconds_auction_prices_fresh": seconds_auction_prices_fresh,
            })
            .to_string()
            .as_str(),
            PRICE_ORACLE_NAME,
            None,
            "",
        )?;

        let chain = self.get_mut_chain(NEUTRON_CHAIN_NAME);

        chain
            .contract_addrs
            .insert(PRICE_ORACLE_NAME.to_owned(), contract.address);

        Ok(())
    }

    /// Creates an auction on Neutron. Requires that an auction manager has already been deployed.
    pub fn build_tx_create_auction(&mut self) -> CreateAuctionTxBuilder {
        CreateAuctionTxBuilder {
            key: DEFAULT_KEY,
            offer_asset: Default::default(),
            ask_asset: Default::default(),
            auction_strategy: AuctionStrategy {
                start_price_perc: 5000,
                end_price_perc: 5000,
            },
            chain_halt_config: ChainHaltConfig {
                cap: "14400".into(),
                block_avg: "3".into(),
            },
            price_freshness_strategy: PriceFreshnessStrategy {
                limit: "3".into(),
                multipliers: vec![("2".into(), "2".into()), ("1".into(), "1.5".into())],
            },
            label: DEFAULT_AUCTION_LABEL,
            amount_offer_asset: Default::default(),
            test_ctx: self,
        }
    }

    /// Creates an auction on Neutron. Requires that an auction manager has already been deployed.
    #[allow(clippy::too_many_arguments)]
    fn tx_create_auction<TDenomA: AsRef<str>, TDenomB: AsRef<str>>(
        &mut self,
        sender_key: &str,
        pair: (TDenomA, TDenomB),
        auction_strategy: AuctionStrategy,
        chain_halt_config: ChainHaltConfig,
        price_freshness_strategy: PriceFreshnessStrategy,
        label: impl AsRef<str>,
        amount_denom_a: u128,
    ) -> Result<(), Error> {
        // The auctions manager for this deployment
        let contract_a = self.get_auctions_manager().get_cw();
        let denom_a = pair.0.as_ref();

        let receipt = contract_a.execute(
            sender_key,
            serde_json::json!(
            {
                "admin": {
                    "new_auction": {
                        "msg": {
                            "pair": (pair.0.as_ref(), pair.1.as_ref()),
                            "auction_strategy": auction_strategy,
                            "chain_halt_config": chain_halt_config,
                            "price_freshness_strategy": price_freshness_strategy
                        },
                        "label": label.as_ref(),
                    },
            }})
            .to_string()
            .as_str(),
            format!("--amount {amount_denom_a}{denom_a} --gas 2000000").as_str(),
        )?;

        log::debug!(
            "submitted tx creating auction ({}, {}) {:?}",
            pair.0.as_ref(),
            pair.1.as_ref(),
            receipt
        );

        self.guard_tx_errors(
            NEUTRON_CHAIN_NAME,
            receipt.tx_hash.ok_or(Error::TxMissingLogs)?.as_str(),
        )?;

        Ok(())
    }

    /// Creates an auction on Neutron. Requires that an auction manager has already been deployed.
    pub fn build_tx_migrate_auction(&mut self) -> MigrateAuctionTxBuilder {
        MigrateAuctionTxBuilder {
            key: DEFAULT_KEY,
            offer_asset: Default::default(),
            ask_asset: Default::default(),
            test_ctx: self,
        }
    }

    /// Creates an auction on Neutron. Requires that an auction manager has already been deployed.
    fn tx_migrate_auction<TDenomA: AsRef<str>, TDenomB: AsRef<str>>(
        &mut self,
        sender_key: &str,
        pair: (TDenomA, TDenomB),
    ) -> Result<(), Error> {
        // The auctions manager for this deployment
        let contract_a = self.get_auctions_manager().get_cw();
        let code_id = self
            .get_contract()
            .contract(AUCTION_CONTRACT_NAME)
            .get_cw()
            .code_id
            .ok_or(Error::MissingContextVariable(String::from(
                "code_ids::auction",
            )))?;

        let receipt = contract_a.execute(
            sender_key,
            serde_json::json!(
            {
                "admin": {
                    "migrate_auction": {
                        "pair": (pair.0.as_ref(), pair.1.as_ref()),
                        "code_id": code_id,
                        "msg": {
                            "no_state_change": {}
                        },
                    },
            }})
            .to_string()
            .as_str(),
            "--gas 2000000",
        )?;

        log::debug!(
            "submitted tx migrating auction ({}, {}) {:?}",
            pair.0.as_ref(),
            pair.1.as_ref(),
            receipt
        );

        self.guard_tx_errors(
            NEUTRON_CHAIN_NAME,
            receipt.tx_hash.ok_or(Error::TxMissingLogs)?.as_str(),
        )?;

        Ok(())
    }

    /// Creates a builder setting the oracle address on the auctions manager on neutron.
    pub fn build_tx_update_auction_oracle(&mut self) -> UpdateAuctionOracleTxBuilder {
        UpdateAuctionOracleTxBuilder {
            key: DEFAULT_KEY,
            test_ctx: self,
        }
    }

    fn tx_update_auction_oracle(&mut self, sender_key: &str) -> Result<(), Error> {
        // The auctions manager for this deployment
        let contract_a = self.get_auctions_manager().get_cw();
        let neutron = self.get_chain(NEUTRON_CHAIN_NAME);
        let oracle =
            neutron
                .contract_addrs
                .get(PRICE_ORACLE_NAME)
                .ok_or(Error::MissingContextVariable(String::from(
                    "contract_addrs::price_oracle",
                )))?;

        let receipt = contract_a.execute(
            sender_key,
            serde_json::json!(
            {
                "admin": {
                    "update_oracle": {
                        "oracle_addr": oracle,
                    },
            }})
            .to_string()
            .as_str(),
            "--gas 2000000",
        )?;

        self.guard_tx_errors(
            NEUTRON_CHAIN_NAME,
            receipt.tx_hash.ok_or(Error::TxMissingLogs)?.as_str(),
        )?;

        Ok(())
    }

    /// Creates a builder setting the oracle address on the auctions manager on neutron.
    pub fn build_tx_manual_oracle_price_update(&mut self) -> ManualOraclePriceUpdateTxBuilder {
        ManualOraclePriceUpdateTxBuilder {
            key: DEFAULT_KEY,
            offer_asset: Default::default(),
            ask_asset: Default::default(),
            price: Default::default(),
            test_ctx: self,
        }
    }

    fn tx_manual_oracle_price_update(
        &mut self,
        sender_key: &str,
        offer_asset: &str,
        ask_asset: &str,
        price: Decimal,
    ) -> Result<(), Error> {
        // The auctions manager for this deployment
        let oracle = self.get_price_oracle().get_cw();

        let receipt = oracle.execute(
            sender_key,
            serde_json::json!(
            {
                "manual_price_update": {
                    "pair": (offer_asset, ask_asset),
                    "price": price,
                }
            })
            .to_string()
            .as_str(),
            "--gas 2000000",
        )?;

        self.guard_tx_errors(
            NEUTRON_CHAIN_NAME,
            receipt.tx_hash.ok_or(Error::TxMissingLogs)?.as_str(),
        )?;

        Ok(())
    }

    /// Sends the specified amount of funds to an auction.
    pub fn build_tx_fund_auction(&mut self) -> FundAuctionTxBuilder {
        FundAuctionTxBuilder {
            key: DEFAULT_KEY,
            offer_asset: Default::default(),
            ask_asset: Default::default(),
            amt_offer_asset: Default::default(),
            test_ctx: self,
        }
    }

    /// Sends the specified amount of funds to an auction.
    fn tx_fund_auction<TDenomA: AsRef<str>, TDenomB: AsRef<str>>(
        &mut self,
        sender_key: &str,
        pair: (TDenomA, TDenomB),
        amt_offer_asset: u128,
    ) -> Result<(), Error> {
        let manager = self.get_auctions_manager().get_cw();

        let denom_a = pair.0.as_ref();

        let receipt = manager.execute(
            sender_key,
            serde_json::json!({
                "auction_funds": {
                    "pair": (pair.0.as_ref(), pair.1.as_ref()),
                },
            })
            .to_string()
            .as_str(),
            format!("--amount {amt_offer_asset}{denom_a} --gas 1000000").as_str(),
        )?;

        self.guard_tx_errors(
            NEUTRON_CHAIN_NAME,
            receipt.tx_hash.ok_or(Error::TxMissingLogs)?.as_str(),
        )?;

        Ok(())
    }

    /// Builds a transaction to start the auction.
    pub fn build_tx_start_auction(&mut self) -> StartAuctionTxBuilder {
        StartAuctionTxBuilder {
            key: DEFAULT_KEY,
            offer_asset: Default::default(),
            ask_asset: Default::default(),
            end_block_delta: Default::default(),
            test_ctx: self,
        }
    }

    /// Starts the specified auction.
    fn tx_start_auction<TDenomA: AsRef<str>, TDenomB: AsRef<str>>(
        &mut self,
        sender_key: &str,
        end_blocks: u128,
        pair: (TDenomA, TDenomB),
    ) -> Result<(), Error> {
        let manager = self.get_auctions_manager().get_cw();
        let neutron = self.get_chain(NEUTRON_CHAIN_NAME);

        let start_block_resp = neutron
            .rb
            .bin("q block --node=%RPC% --chain-id=%CHAIN_ID%", true);
        let maybe_start_block_data: Value = start_block_resp
            .get("text")
            .and_then(|maybe_text| maybe_text.as_str())
            .and_then(|text| serde_json::from_str(text).ok())
            .ok_or(Error::ContainerCmd(String::from("query block")))?;

        let maybe_start_block = maybe_start_block_data
            .get("block")
            .and_then(|block| block.get("header"))
            .and_then(|header| header.get("height"))
            .ok_or(Error::ContainerCmd(String::from("query block")))?;
        let start_block = maybe_start_block
            .as_str()
            .and_then(|s| s.parse::<u128>().ok())
            .ok_or(Error::ContainerCmd(String::from("query block")))?;

        let receipt = manager.execute(
            sender_key,
            serde_json::json!({
                "server": {
                    "open_auction": {
                        "pair": (pair.0.as_ref(), pair.1.as_ref()),
                        "params": {
                        "end_block": start_block + end_blocks,
                        "start_block": start_block,
                    }
                }},
            })
            .to_string()
            .as_str(),
            "--gas 1000000",
        )?;

        self.guard_tx_errors(
            NEUTRON_CHAIN_NAME,
            receipt.tx_hash.ok_or(Error::TxMissingLogs)?.as_str(),
        )?;

        Ok(())
    }
}
