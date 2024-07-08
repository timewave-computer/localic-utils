use crate::{
    GAIA_CHAIN_ADMIN_ADDR, GAIA_CHAIN_DENOM, GAIA_CHAIN_ID, GAIA_CHAIN_NAME, GAIA_CHAIN_PREFIX,
    NEUTRON_CHAIN_ID,
};

use super::super::{
    NEUTRON_CHAIN_ADMIN_ADDR, NEUTRON_CHAIN_DENOM, NEUTRON_CHAIN_NAME, NEUTRON_CHAIN_PREFIX,
    OSMOSIS_CHAIN_ADMIN_ADDR, OSMOSIS_CHAIN_DENOM, OSMOSIS_CHAIN_ID, OSMOSIS_CHAIN_NAME,
    OSMOSIS_CHAIN_PREFIX, STRIDE_CHAIN_ADMIN_ADDR, STRIDE_CHAIN_DENOM, STRIDE_CHAIN_ID,
    STRIDE_CHAIN_NAME, STRIDE_CHAIN_PREFIX,
};
use derive_builder::Builder;
use serde::Deserialize;

#[derive(Deserialize, Default, Builder, Debug)]
#[builder(setter(into, prefix = "with"))]
pub struct ChainsVec {
    pub chains: Vec<ConfigChain>,
}

impl From<ChainsVec> for Vec<ConfigChain> {
    fn from(val: ChainsVec) -> Vec<ConfigChain> {
        val.chains
    }
}

#[derive(Clone, Deserialize, Default, Builder, Debug)]
#[builder(setter(into, prefix = "with"))]
pub struct ConfigChain {
    pub denom: String,
    pub debugging: bool,
    pub chain_id: String,
    pub chain_name: String,
    pub chain_prefix: String,
    pub admin_addr: String,
}

impl ConfigChainBuilder {
    pub fn default_gaia() -> Self {
        Self {
            denom: Some(String::from(GAIA_CHAIN_DENOM)),
            debugging: Some(true),
            chain_id: Some(String::from(GAIA_CHAIN_ID)),
            chain_name: Some(String::from(GAIA_CHAIN_NAME)),
            chain_prefix: Some(String::from(GAIA_CHAIN_PREFIX)),
            admin_addr: Some(String::from(GAIA_CHAIN_ADMIN_ADDR)),
        }
    }

    pub fn default_neutron() -> Self {
        Self {
            denom: Some(String::from(NEUTRON_CHAIN_DENOM)),
            debugging: Some(true),
            chain_id: Some(String::from(NEUTRON_CHAIN_ID)),
            chain_name: Some(String::from(NEUTRON_CHAIN_NAME)),
            chain_prefix: Some(String::from(NEUTRON_CHAIN_PREFIX)),
            admin_addr: Some(String::from(NEUTRON_CHAIN_ADMIN_ADDR)),
        }
    }

    pub fn default_osmosis() -> Self {
        Self {
            denom: Some(String::from(OSMOSIS_CHAIN_DENOM)),
            debugging: Some(true),
            chain_id: Some(String::from(OSMOSIS_CHAIN_ID)),
            chain_name: Some(String::from(OSMOSIS_CHAIN_NAME)),
            chain_prefix: Some(String::from(OSMOSIS_CHAIN_PREFIX)),
            admin_addr: Some(String::from(OSMOSIS_CHAIN_ADMIN_ADDR)),
        }
    }

    pub fn default_stride() -> Self {
        Self {
            denom: Some(String::from(STRIDE_CHAIN_DENOM)),
            debugging: Some(true),
            chain_id: Some(String::from(STRIDE_CHAIN_ID)),
            chain_name: Some(String::from(STRIDE_CHAIN_NAME)),
            chain_prefix: Some(String::from(STRIDE_CHAIN_PREFIX)),
            admin_addr: Some(String::from(STRIDE_CHAIN_ADMIN_ADDR)),
        }
    }
}
