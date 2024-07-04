use super::{
    super::{error::Error, DEFAULT_KEY, NEUTRON_CHAIN_NAME, WASM_EXTENSION},
    test_context::TestContext,
};
use localic_std::modules::cosmwasm::CosmWasm;
use std::{ffi::OsStr, fs};

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
}
