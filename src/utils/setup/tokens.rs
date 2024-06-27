use super::super::{
    super::{error::Error, DEFAULT_KEY, NEUTRON_CHAIN_ID},
    test_context::TestContext,
};

/// A tx creating a token registry.
pub struct CreateTokenFactoryTokenTxBuilder<'a> {
    key: Option<&'a str>,
    chain_id: Option<&'a str>,
    subdenom: Option<&'a str>,
    test_ctx: &'a mut TestContext,
}

impl<'a> CreateTokenFactoryTokenTxBuilder<'a> {
    pub fn with_key(&mut self, key: &'a str) -> &mut Self {
        self.key = Some(key);

        self
    }

    pub fn with_chain_id(&mut self, chain_id: &'a str) -> &mut Self {
        self.chain_id = Some(chain_id);

        self
    }

    pub fn with_subdenom(&mut self, subdenom: &'a str) -> &mut Self {
        self.subdenom = Some(subdenom);

        self
    }

    /// Sends the transaction.
    pub fn send(&mut self) -> Result<(), Error> {
        self.test_ctx.tx_create_tokenfactory_token(
            self.chain_id
                .ok_or(Error::MissingBuilderParam(String::from("chain_id")))?,
            self.key
                .ok_or(Error::MissingBuilderParam(String::from("key")))?,
            self.subdenom
                .ok_or(Error::MissingBuilderParam(String::from("subdenom")))?,
        )
    }
}

impl TestContext {
    pub fn build_tx_create_tokenfactory_token(&mut self) -> CreateTokenFactoryTokenTxBuilder {
        CreateTokenFactoryTokenTxBuilder {
            key: Some(DEFAULT_KEY),
            chain_id: Some(NEUTRON_CHAIN_ID),
            subdenom: Default::default(),
            test_ctx: self,
        }
    }

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
