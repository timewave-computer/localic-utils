use crate::TRANSFER_PORT;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

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

#[derive(Debug)]
pub struct DenomTrace {
    pub path: String,
    pub base_denom: String,
}

impl DenomTrace {
    pub fn ibc_denom(&self) -> String {
        if !self.path.is_empty() {
            return format!("ibc/{}", self.hash());
        }
        self.base_denom.clone()
    }

    fn hash(&self) -> String {
        let trace = format!("{}/{}", self.path, self.base_denom);
        let mut hasher = Sha256::new();
        hasher.update(trace.as_bytes());
        format!("{:x}", hasher.finalize()).to_uppercase()
    }
}

pub fn get_multihop_ibc_denom(native_denom: &str, channel_trace: Vec<&str>) -> String {
    let mut port_channel_trace = vec![];

    for channel in channel_trace {
        port_channel_trace.push(TRANSFER_PORT);
        port_channel_trace.push(channel);
    }

    let prefixed_denom = format!("{}/{}", port_channel_trace.join("/"), native_denom);

    let src_denom_trace = parse_denom_trace(prefixed_denom);
    src_denom_trace.ibc_denom()
}

pub fn parse_denom_trace(raw_denom: String) -> DenomTrace {
    let denom_split = raw_denom.split('/').collect::<Vec<&str>>();

    if denom_split[0] == raw_denom {
        return DenomTrace {
            path: "".to_string(),
            base_denom: raw_denom.to_string(),
        };
    }

    let (path, base_denom) = extract_path_and_base_from_full_denom(denom_split);

    DenomTrace { path, base_denom }
}

pub fn extract_path_and_base_from_full_denom(full_denom_items: Vec<&str>) -> (String, String) {
    let mut path: Vec<&str> = Vec::new();
    let mut base_denom: Vec<&str> = Vec::new();

    let length = full_denom_items.len();
    let mut i = 0;
    while i < length {
        if i < length - 1 && length > 2 {
            path.push(full_denom_items[i]);
            path.push(full_denom_items[i + 1]);
        } else {
            base_denom = full_denom_items[i..].to_vec();
            break;
        }
        i += 2;
    }

    (path.join("/"), base_denom.join("/"))
}
