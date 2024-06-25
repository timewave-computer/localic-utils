use derive_builder::Builder;
use serde::Deserialize;

#[derive(Deserialize, Default, Builder, Debug)]
#[builder(setter(into))]
pub struct ChainsVec {
    pub chains: Vec<ConfigChain>,
}

impl Into<Vec<ConfigChain>> for ChainsVec {
    fn into(self) -> Vec<ConfigChain> {
        self.chains
    }
}

#[derive(Clone, Deserialize, Default, Builder, Debug)]
#[builder(setter(into))]
pub struct ConfigChain {
    pub denom: String,
    pub debugging: bool,
    pub chain_id: String,
    pub admin_addr: String,
}
