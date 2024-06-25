use super::{
    super::{error::Error, AUCTION_CONTRACT_NAME, NEUTRON_CHAIN_ID},
    test_context::TestContext,
};
use localic_std::modules::cosmwasm::CosmWasm;
use std::path::PathBuf;

impl TestContext {
    /// Get a new CosmWasm instance for a contract identified by a name.
    pub fn get_contract(&self, name: &str) -> Result<CosmWasm, Error> {
        let chain = self.get_chain(NEUTRON_CHAIN_ID);

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
        let neutron = self.get_chain(NEUTRON_CHAIN_ID);

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
}
