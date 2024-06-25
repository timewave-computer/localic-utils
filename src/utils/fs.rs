use super::{
    super::{error::Error, NEUTRON_CHAIN_ID, WASM_EXTENSION},
    test_context::TestContext,
};
use localic_std::modules::cosmwasm::CosmWasm;
use std::{ffi::OsStr, fs};

impl TestContext {
    pub fn tx_upload_contracts(&mut self, key: &str) -> Result<(), Error> {
        fs::read_dir(&self.artifacts_dir)?
            .filter_map(|dir_ent| dir_ent.ok())
            .filter(|dir_ent| {
                dir_ent.path().extension().and_then(OsStr::to_str) == Some(WASM_EXTENSION)
            })
            .map(|ent| ent.path())
            .map(fs::canonicalize)
            .try_for_each(|maybe_abs_path| {
                let path = maybe_abs_path?;
                let neutron_local_chain = self.get_mut_chain(NEUTRON_CHAIN_ID);

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
