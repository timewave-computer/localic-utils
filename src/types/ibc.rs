use serde::Serialize;

#[derive(Serialize)]
pub struct Trace {
    pub channel_id: String,
    pub port_id: String,
    pub base_denom: String,
    pub dest_denom: String,
}
