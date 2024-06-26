use super::super::{
    super::{
        error::Error,
        types::contract::{DeployedContractInfo, PairType},
        FACTORY_NAME, NEUTRON_CHAIN_ID, TOKEN_REGISTRY_NAME,
    },
    test_context::TestContext,
};

impl TestContext {
    /// Instantiates the token registry.
    pub fn create_token_registry(&mut self, key: &str, owner_addr: &str) -> Result<(), Error> {
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
    pub fn create_factory(&mut self, key: &str, factory_owner: &str) -> Result<(), Error> {
        let neutron = self.get_chain(NEUTRON_CHAIN_ID);

        let native_registry_addr = neutron.contract_addrs.get(TOKEN_REGISTRY_NAME).ok_or(
            Error::MissingContextVariable(String::from(
                "contract_codes::astroport_native_coin_registry",
            )),
        )?;

        let mut contract_a = self.get_contract(FACTORY_NAME)?;
        let contract_a_code =
            contract_a
                .code_id
                .ok_or(Error::MissingContextVariable(String::from(
                    "contract_codes::astroport_factory",
                )))?;

        let contract = contract_a.instantiate(
            key,
            serde_json::json!({
                "pair_configs": [
                    {
                        "code_id": contract_a_code,
                        "pair_type": {
                             "xyk": {}
                        },
                        "total_fee_bps": 100,
                        "maker_fee_bps": 10,
                        "is_disabled": false,
                        "is_generator_disabled": false
                    },
                    {
                        "code_id": contract_a_code,
                        "pair_type": {
                             "stable": {}
                        },
                        "total_fee_bps": 100,
                        "maker_fee_bps": 10,
                        "is_disabled": false,
                        "is_generator_disabled": false
                    }
                ],
                "token_code_id": 0,
                "owner": factory_owner,
                "whitelist_code_id": 0,
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
    pub fn create_pool(
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
            .ok_or(Error::MissingContextVariable(String::from(
                "astroport_factory",
            )))?;

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
            "",
        )?;

        let logs = tx
            .raw_log
            .ok_or(Error::ContainerCmd(String::from("create_pair")))
            .and_then(|log| serde_json::from_str(log.as_str()).map_err(|_| Error::Serialization))?;

        println!("{:?}", logs);

        /*let _ = contract_a.execute(
            ACC_0_KEY,
            serde_json::json!({
            "register": {
                 "pair_contract_addr":
            }})
            .to_string()
            .as_str(),
            "",
        )?;*/

        Ok(())
    }
}
