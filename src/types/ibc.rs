use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct Trace {
    pub channel_id: String,
    pub port_id: String,
    pub base_denom: String,
    pub dest_denom: String,
}

#[derive(Deserialize)]
pub struct Channel {
    pub channel_id: String,
    pub connection_hops: Vec<String>,
    pub counterparty: Counterparty,
    pub ordering: String,
    pub port_id: String,
    pub state: String,
    pub version: String,
}

#[derive(Deserialize, Debug)]
pub struct Counterparty {
    pub channel_id: String,
    pub port_id: String,
}
