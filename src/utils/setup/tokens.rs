use super::super::{super::error::Error, test_context::TestContext};

impl TestContext {
    /// Creates a tokenfactory token with the given subdenom on the given chain.
    pub fn tx_create_tokenfactory_token(
        &mut self,
        chain_id: &str,
        key: &str,
        subdenom: &str,
    ) -> Result<(), Error> {
        let chain = self.get_chain(chain_id);

        let _ = chain.rb.tx(
            format!("tx tokenfactory create-denom {subdenom} --from {key}").as_str(),
            true,
        )?;

        Ok(())
    }
}
