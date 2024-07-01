use super::{
    super::{
        error::Error, types::ibc::Trace, AUCTION_CONTRACT_NAME, FACTORY_NAME, NEUTRON_CHAIN_NAME,
        OSMOSIS_CHAIN_NAME, PAIR_NAME, PRICE_ORACLE_NAME, STABLE_PAIR_NAME,
        TX_HASH_QUERY_PAUSE_SEC, TX_HASH_QUERY_RETRIES,
    },
    test_context::TestContext,
};
use localic_std::modules::cosmwasm::CosmWasm;
use serde_json::Value;
use std::{path::PathBuf, thread, time::Duration};

impl TestContext {
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

        if &raw_log == &"" {
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

    /// Get a new CosmWasm instance for a contract identified by a name.
    pub fn get_contract(&self, name: &str) -> Result<CosmWasm, Error> {
        let chain = self.get_chain(NEUTRON_CHAIN_NAME);

        let code_id = chain
            .contract_codes
            .get(name)
            .ok_or(Error::Misc(format!("contract '{name}' is missing")))?;

        let artifacts_path = &self.artifacts_dir;

        Ok(CosmWasm::new_from_existing(
            &chain.rb,
            Some(PathBuf::from(format!("{artifacts_path}/{name}.wasm"))),
            Some(*code_id),
            None,
        ))
    }

    /// Get a new CosmWasm instance for the existing deployed auctions manager.
    pub fn get_auctions_manager(&self) -> Result<CosmWasm, Error> {
        let neutron = self.get_chain(NEUTRON_CHAIN_NAME);

        let contract_info = self
            .auctions_manager
            .as_ref()
            .ok_or(Error::MissingContextVariable(String::from(
                "auctions_manager",
            )))?;

        Ok(CosmWasm::new_from_existing(
            &neutron.rb,
            Some(contract_info.artifact_path.clone()),
            Some(contract_info.code_id.clone()),
            Some(contract_info.address.clone()),
        ))
    }

    /// Get a new CosmWasm instance for the existing deployed auctions manager.
    pub fn get_price_oracle(&self) -> Result<CosmWasm, Error> {
        let neutron = self.get_chain(NEUTRON_CHAIN_NAME);

        let mut contract = self.get_contract(PRICE_ORACLE_NAME)?;
        let contract_addr = neutron
            .contract_addrs
            .get(PRICE_ORACLE_NAME)
            .and_then(|addrs| addrs.get(0))
            .cloned()
            .ok_or(Error::MissingContextVariable(String::from(
                "contract_addrs::price_oracle",
            )))?;
        contract.contract_addr = Some(contract_addr);

        Ok(contract)
    }

    /// Gets a CosmWasm instance for an auction with a given pair.
    pub fn get_auction<TDenomA: AsRef<str>, TDenomB: AsRef<str>>(
        &self,
        denoms: (TDenomA, TDenomB),
    ) -> Result<CosmWasm, Error> {
        let mut auction_contract = self.get_contract(AUCTION_CONTRACT_NAME)?;

        // The auctions manager for this deployment
        let contract_a = self.get_auctions_manager()?;

        // Get the address of the auction specified
        let resp = contract_a.query_value(&serde_json::json!({
            "get_pair_addr": {
                "pair": (denoms.0.as_ref(), denoms.1.as_ref())
            }
        }));

        auction_contract.contract_addr = Some(
            resp.get("data")
                .and_then(|json| json.as_str())
                .ok_or(Error::Misc(format!("tx failed with resp: {:?}", resp)))?
                .to_owned(),
        );

        Ok(auction_contract)
    }

    pub fn get_tokenfactory_denom(&self, creator_addr: &str, subdenom: &str) -> String {
        format!("factory/{creator_addr}/{subdenom}")
    }

    /// Gets the deployed atroport factory for Neutron.
    pub fn get_astroport_factory(&self) -> Result<Vec<CosmWasm>, Error> {
        let neutron = self.get_chain(NEUTRON_CHAIN_NAME);

        let code_id =
            neutron
                .contract_codes
                .get(FACTORY_NAME)
                .ok_or(Error::MissingContextVariable(format!(
                    "contract_codes::{FACTORY_NAME}",
                )))?;
        let contract_addrs =
            neutron
                .contract_addrs
                .get(FACTORY_NAME)
                .ok_or(Error::MissingContextVariable(format!(
                    "contract_addrs::{FACTORY_NAME}",
                )))?;

        let artifacts_path = self.artifacts_dir.as_str();

        Ok(contract_addrs
            .into_iter()
            .map(|addr| {
                CosmWasm::new_from_existing(
                    &neutron.rb,
                    Some(PathBuf::from(format!(
                        "{artifacts_path}/{FACTORY_NAME}.wasm"
                    ))),
                    Some(*code_id),
                    Some(addr.clone()),
                )
            })
            .collect::<Vec<_>>())
    }

    /// Gets a previously deployed astroport pair.
    pub fn get_astroport_pool(
        &self,
        denom_a: impl AsRef<str>,
        denom_b: impl AsRef<str>,
    ) -> Result<CosmWasm, Error> {
        let factories = self.get_astroport_factory()?;
        let factory = factories
            .get(0)
            .ok_or(Error::MissingContextVariable(String::from(FACTORY_NAME)))?;

        let pair_info = factory.query_value(&serde_json::json!(
            {
                "pair": {
                    "asset_infos": [
                        {
                            "native_token": {
                                "denom": denom_a.as_ref(),
                            }
                        },
                        {
                            "native_token": {
                                "denom": denom_b.as_ref(),
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

        let neutron = self.get_chain(NEUTRON_CHAIN_NAME);

        if kind.get("xyk").is_some() {
            let contract = self.get_contract(PAIR_NAME)?;

            return Ok(CosmWasm::new_from_existing(
                &neutron.rb,
                contract.file_path,
                contract.code_id,
                Some(addr.to_owned()),
            ));
        }

        let contract = self.get_contract(STABLE_PAIR_NAME)?;

        Ok(CosmWasm::new_from_existing(
            &neutron.rb,
            contract.file_path,
            contract.code_id,
            Some(addr.to_owned()),
        ))
    }

    /// Gets the id of the pool with the specifieed denoms.
    pub fn get_osmo_pool(
        &self,
        denom_a: impl AsRef<str>,
        denom_b: impl AsRef<str>,
    ) -> Result<u64, Error> {
        let osmosis = self.get_chain(OSMOSIS_CHAIN_NAME);
        let denom_a_str = denom_a.as_ref();

        let res = osmosis.rb.query(
            &format!("q poolmanager list-pools-by-denom {denom_a_str} --output=json"),
            true,
        );

        let res_text = res.get("text").and_then(|v| v.as_str()).unwrap();
        let res_value: Value = serde_json::from_str(res_text)?;

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

        Ok(pool.parse().unwrap())
    }

    /// Gets the IBC denom for a base denom given a src and dest chain.
    pub fn get_ibc_denom(
        &self,
        base_denom: impl AsRef<str>,
        src_chain: impl Into<String>,
        dest_chain: impl Into<String>,
    ) -> Option<String> {
        let src_chain_string = src_chain.into();
        let dest_chain_string = dest_chain.into();
        let base_denom_str = base_denom.as_ref();

        let dest_chain = self.get_chain(&dest_chain_string);

        let channel = self
            .transfer_channel_ids
            .get(&(src_chain_string, dest_chain_string))?;
        let trace = format!("transfer/{}/{}", channel, base_denom_str);

        let resp = dest_chain
            .rb
            .q(&format!("q ibc-transfer denom-hash {trace}"), true);

        Some(format!(
            "ibc/{}",
            serde_json::from_str::<Value>(&resp.get("text")?.as_str()?)
                .ok()?
                .get("hash")?
                .as_str()?
        ))
    }

    /// Gets the IBC channel and port for a given denom.
    pub fn get_ibc_trace(
        &self,
        base_denom: impl Into<String> + AsRef<str>,
        src_chain: impl Into<String> + AsRef<str>,
        dest_chain: impl Into<String> + AsRef<str>,
    ) -> Option<Trace> {
        let dest_denom =
            self.get_ibc_denom(base_denom.as_ref(), src_chain.as_ref(), dest_chain.as_ref())?;

        let channel = self
            .transfer_channel_ids
            .get(&(src_chain.into(), dest_chain.into()))?;

        Some(Trace {
            port_id: "transfer".to_owned(),
            channel_id: channel.to_owned(),
            base_denom: base_denom.into(),
            dest_denom,
        })
    }
}
