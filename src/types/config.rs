use super::super::{
    NEUTRON_CHAIN_ADMIN_ADDR, NEUTRON_CHAIN_DENOM, NEUTRON_CHAIN_ID, OSMOSIS_CHAIN_ADMIN_ADDR,
    OSMOSIS_CHAIN_DENOM, OSMOSIS_CHAIN_ID, STRIDE_CHAIN_ADMIN_ADDR, STRIDE_CHAIN_DENOM,
    STRIDE_CHAIN_ID,
};
use derive_builder::Builder;
use serde::Deserialize;

#[derive(Deserialize, Default, Builder, Debug)]
#[builder(setter(into, prefix = "with"))]
pub struct ChainsVec {
    pub chains: Vec<ConfigChain>,
}

impl Into<Vec<ConfigChain>> for ChainsVec {
    fn into(self) -> Vec<ConfigChain> {
        self.chains
    }
}

#[derive(Clone, Deserialize, Default, Builder, Debug)]
#[builder(setter(into, prefix = "with"))]
pub struct ConfigChain {
    pub denom: String,
    pub debugging: bool,
    pub chain_id: String,
    pub admin_addr: String,
}

impl ConfigChainBuilder {
    pub fn default_neutron() -> Self {
        Self {
            denom: Some(String::from(NEUTRON_CHAIN_DENOM)),
            debugging: Some(true),
            chain_id: Some(String::from(NEUTRON_CHAIN_ID)),
            admin_addr: Some(String::from(NEUTRON_CHAIN_ADMIN_ADDR)),
        }
    }

    pub fn default_osmosis() -> Self {
        Self {
            denom: Some(String::from(OSMOSIS_CHAIN_DENOM)),
            debugging: Some(true),
            chain_id: Some(String::from(OSMOSIS_CHAIN_ID)),
            admin_addr: Some(String::from(OSMOSIS_CHAIN_ADMIN_ADDR)),
        }
    }

    pub fn default_stride() -> Self {
        Self {
            denom: Some(String::from(STRIDE_CHAIN_DENOM)),
            debugging: Some(true),
            chain_id: Some(String::from(STRIDE_CHAIN_ID)),
            admin_addr: Some(String::from(STRIDE_CHAIN_ADMIN_ADDR)),
        }
    }
}
