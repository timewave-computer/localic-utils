use super::super::{
    super::{
        error::Error,
        types::contract::{DeployedContractInfo, PairType},
        FACTORY_NAME, NEUTRON_CHAIN_ID, PAIR_NAME, STABLE_PAIR_NAME, TOKEN_NAME,
        TOKEN_REGISTRY_NAME, WHITELIST_NAME,
    },
    test_context::TestContext,
};

impl TestContext {
    /// Instantiates the token registry.
    pub fn tx_create_token_registry(&mut self, key: &str, owner_addr: &str) -> Result<(), Error> {
        let mut contract_a = self.get_contract(TOKEN_REGISTRY_NAME)?;
        let code_id = contract_a
            .code_id
            .ok_or(Error::MissingContextVariable(String::from(
                "astroport_token_registry::code_id",
            )))?;

        let contract = contract_a.instantiate(
            key,
            serde_json::json!({
                "owner": owner_addr,
            })
            .to_string()
            .as_str(),
            TOKEN_REGISTRY_NAME,
            None,
            "",
        )?;
        let addr = contract.address;
        let artifact_path =
            contract_a
                .file_path
                .ok_or(Error::MissingContextVariable(String::from(
                    "astroport_token_registry::artifact_path",
                )))?;

        let neutron = self.get_mut_chain(NEUTRON_CHAIN_ID);

        neutron
            .contract_addrs
            .entry(TOKEN_REGISTRY_NAME.to_owned())
            .or_default()
            .push(addr.clone());

        self.astroport_token_registry = Some(DeployedContractInfo {
            code_id,
            address: addr,
            artifact_path,
        });

        Ok(())
    }

    /// Instantiates the astroport factory.
    /// Note: by default, all pair types are enabled
    pub fn tx_create_factory(&mut self, key: &str, factory_owner: &str) -> Result<(), Error> {
        let neutron = self.get_chain(NEUTRON_CHAIN_ID);

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

        let native_registry_addr = neutron
            .contract_addrs
            .get(TOKEN_REGISTRY_NAME)
            .and_then(|maybe_addr| maybe_addr.get(0))
            .ok_or(Error::MissingContextVariable(String::from(
                "contract_ddrs::astroport_native_coin_registry",
            )))?;

        let mut contract_a = self.get_contract(FACTORY_NAME)?;

        let contract = contract_a.instantiate(
            key,
            serde_json::json!({
                "pair_configs": [
                    {
                        "code_id": pair_xyk_code_id,
                        "pair_type": {
                             "xyk": {}
                        },
                        "total_fee_bps": 100,
                        "maker_fee_bps": 10,
                        "is_disabled": false,
                        "is_generator_disabled": false
                    },
                    {
                        "code_id": pair_stable_code_id,
                        "pair_type": {
                             "stable": {}
                        },
                        "total_fee_bps": 100,
                        "maker_fee_bps": 10,
                        "is_disabled": false,
                        "is_generator_disabled": false
                    }
                ],
                "token_code_id": token_code_id,
                "owner": factory_owner,
                "whitelist_code_id": whitelist_code_id,
                "coin_registry_address": native_registry_addr
            })
            .to_string()
            .as_str(),
            FACTORY_NAME,
            None,
            "",
        )?;

        let neutron = self.get_mut_chain(NEUTRON_CHAIN_ID);

        neutron
            .contract_addrs
            .entry(FACTORY_NAME.to_owned())
            .or_default()
            .push(contract.address);

        Ok(())
    }

    /// Creates a pool with the specififed denoms.
    pub fn tx_create_pool(
        &self,
        key: &str,
        pair_type: PairType,
        denom_a: &str,
        denom_b: &str,
    ) -> Result<(), Error> {
        // Factory contract instance
        let contracts = self.get_astroport_factory()?;
        let contract_a = contracts
            .get(0)
            .ok_or(Error::MissingContextVariable(String::from(FACTORY_NAME)))?;

        let neutron = self.get_chain(NEUTRON_CHAIN_ID);

        // Create the pair
        let tx = contract_a.execute(
            key,
            serde_json::json!({
            "create_pair": {
                 "pair_type": pair_type,
                 "asset_infos": [
                     {
                         "native_token": {
                             "denom": denom_a
                         }
                     },
                     {
                         "native_token": {
                             "denom": denom_b
                         }
                     }
                 ]
            }})
            .to_string()
            .as_str(),
            "--gas 1000000",
        )?;

        // Get the address of the createed contract via logs
        let tx_hash = tx.tx_hash.ok_or(Error::Misc(String::from(
            "transaction did not produce a tx hash",
        )))?;

        let logs = neutron.rb.query_tx_hash(tx_hash.as_str());

        let addr = logs
            .get("events")
            .and_then(|events| events.as_array())
            .and_then(|events| {
                events.into_iter().find(|event| {
                    event.get("type").and_then(|maybe_ty| maybe_ty.as_str()) == Some("instantiate")
                })
            })
            .and_then(|event| event.get("attributes"))
            .and_then(|attrs| attrs.as_array())
            .and_then(|attrs| attrs.get(0))
            .and_then(|contract_addr_attr| contract_addr_attr.get("value"))
            .and_then(|val| val.as_str())
            .ok_or(Error::ContainerCmd(String::from("query create_pool logs")))?;

        log::debug!("created pool: {}", addr);

        Ok(())
    }
}
