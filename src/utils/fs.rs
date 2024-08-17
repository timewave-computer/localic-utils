use super::{
    super::{error::Error, DEFAULT_KEY, NEUTRON_CHAIN_NAME, WASM_EXTENSION},
    test_context::TestContext,
};
use localic_std::modules::cosmwasm::CosmWasm;
use log::{error, info};
use std::{
    collections::HashMap,
    ffi::OsStr,
    fs::{self, File},
    io::{Read, Write},
};

/// A tx uploading contract artifacts.
pub struct UploadContractsTxBuilder<'a> {
    key: Option<&'a str>,
    test_ctx: &'a mut TestContext,
}

impl<'a> UploadContractsTxBuilder<'a> {
    pub fn with_key(&mut self, key: &'a str) -> &mut Self {
        self.key = Some(key);

        self
    }

    /// Sends the transaction.
    pub fn send(&mut self) -> Result<(), Error> {
        self.test_ctx.tx_upload_contracts(
            self.key
                .ok_or(Error::MissingBuilderParam(String::from("key")))?,
        )
    }

    /// Sends the transaction using a path, chain and local cache path
    pub fn send_with_local_cache(
        &mut self,
        path: &str,
        chain_name: &str,
        local_cache_path: &str,
    ) -> Result<(), Error> {
        self.test_ctx.tx_upload_contracts_with_local_cache(
            self.key
                .ok_or(Error::MissingBuilderParam(String::from("key")))?,
            path,
            chain_name,
            local_cache_path,
        )
    }
}

impl TestContext {
    pub fn build_tx_upload_contracts(&mut self) -> UploadContractsTxBuilder {
        UploadContractsTxBuilder {
            key: Some(DEFAULT_KEY),
            test_ctx: self,
        }
    }

    fn tx_upload_contracts(&mut self, key: &str) -> Result<(), Error> {
        fs::read_dir(&self.artifacts_dir)?
            .filter_map(|dir_ent| dir_ent.ok())
            .filter(|dir_ent| {
                dir_ent.path().extension().and_then(OsStr::to_str) == Some(WASM_EXTENSION)
            })
            .map(|ent| ent.path())
            .map(fs::canonicalize)
            .try_for_each(|maybe_abs_path| {
                let path = maybe_abs_path?;
                let neutron_local_chain = self.get_mut_chain(NEUTRON_CHAIN_NAME);

                let mut cw = CosmWasm::new(&neutron_local_chain.rb);

                let code_id = cw.store(key, &path)?;

                let id = path
                    .file_stem()
                    .and_then(|stem| stem.to_str())
                    .ok_or(Error::Misc(String::from("failed to format file path")))?;
                neutron_local_chain
                    .contract_codes
                    .insert(id.to_string(), code_id);

                Ok(())
            })
    }

    fn tx_upload_contracts_with_local_cache(
        &mut self,
        key: &str,
        path: &str,
        chain_name: &str,
        local_cache_path: &str,
    ) -> Result<(), Error> {
        if fs::metadata(path).is_ok_and(|m| m.is_dir()) {
            info!("Path {} exists, deploying contracts...", path);
        } else {
            error!(
                "Path {} does not exist, you might have to build and optimize contracts",
                path
            );
            return Err(Error::Misc(String::from("Path does not exist")));
        };

        let artifacts = fs::read_dir(path).unwrap();

        let mut dir_entries = vec![];
        for dir in artifacts.into_iter() {
            dir_entries.push(dir.unwrap());
        }

        let local_ic_session = self.log_file.start_time;
        let session_cache_path =
            local_cache_path.replace(".json", &format!("_{local_ic_session}.json"));

        // Use a local cache to avoid storing the same contract multiple times, useful for local testing
        let mut content = String::new();
        let cache: HashMap<String, u64> = match File::open(&session_cache_path) {
            Ok(mut file) => {
                if let Err(err) = file.read_to_string(&mut content) {
                    error!("Failed to read cache file: {}", err);
                    HashMap::new()
                } else {
                    serde_json::from_str(&content).unwrap_or_default()
                }
            }
            Err(_) => {
                // If the file does not exist, we'll create it later
                HashMap::new()
            }
        };

        let local_chain = self.get_mut_chain(chain_name);
        // Add all cache entries to the local chain
        for (id, code_id) in cache {
            local_chain.contract_codes.insert(id, code_id);
        }

        for entry in dir_entries {
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) == Some(WASM_EXTENSION) {
                let abs_path = path.canonicalize().unwrap();
                let mut cw = CosmWasm::new(&local_chain.rb);
                let id = abs_path.file_stem().unwrap().to_str().unwrap();

                // To avoid storing multiple times during the same execution
                if local_chain.contract_codes.contains_key(id) {
                    info!(
                        "Contract {} already deployed on chain {}, skipping...",
                        id, chain_name
                    );
                    continue;
                }

                let code_id = cw.store(key, abs_path.as_path()).unwrap();

                local_chain.contract_codes.insert(id.to_string(), code_id);
            }
        }

        let contract_codes = serde_json::to_string(&local_chain.contract_codes).unwrap();
        let mut file = File::create(session_cache_path).unwrap();
        file.write_all(contract_codes.as_bytes()).unwrap();

        Ok(())
    }
}
