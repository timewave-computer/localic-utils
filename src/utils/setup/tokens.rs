use super::super::{
    super::{error::Error, DEFAULT_KEY, NEUTRON_CHAIN_NAME},
    test_context::TestContext,
};

/// A tx creating a tokenfactory token.
pub struct CreateTokenFactoryTokenTxBuilder<'a> {
    key: Option<&'a str>,
    chain_name: Option<String>,
    subdenom: Option<&'a str>,
    test_ctx: &'a mut TestContext,
}

impl<'a> CreateTokenFactoryTokenTxBuilder<'a> {
    pub fn with_key(&mut self, key: &'a str) -> &mut Self {
        self.key = Some(key);

        self
    }

    pub fn with_chain_name(&mut self, chain_name: impl Into<String>) -> &mut Self {
        self.chain_name = Some(chain_name.into());

        self
    }

    pub fn with_subdenom(&mut self, subdenom: &'a str) -> &mut Self {
        self.subdenom = Some(subdenom);

        self
    }

    /// Sends the transaction.
    pub fn send(&mut self) -> Result<(), Error> {
        self.test_ctx.tx_create_tokenfactory_token(
            self.chain_name
                .as_ref()
                .ok_or(Error::MissingBuilderParam(String::from("chain_name")))?,
            self.key
                .ok_or(Error::MissingBuilderParam(String::from("key")))?,
            self.subdenom
                .ok_or(Error::MissingBuilderParam(String::from("subdenom")))?,
        )
    }
}

/// A tx minting a tokens from the token factory.
pub struct MintTokenFactoryTokenTxBuilder<'a> {
    key: Option<&'a str>,
    chain_name: Option<String>,
    denom: Option<&'a str>,
    amount: Option<u128>,
    recipient_addr: Option<&'a str>,
    test_ctx: &'a mut TestContext,
}

impl<'a> MintTokenFactoryTokenTxBuilder<'a> {
    pub fn with_key(&mut self, key: &'a str) -> &mut Self {
        self.key = Some(key);

        self
    }

    pub fn with_chain_name(&mut self, chain_name: impl Into<String>) -> &mut Self {
        self.chain_name = Some(chain_name.into());

        self
    }

    pub fn with_denom(&mut self, denom: &'a str) -> &mut Self {
        self.denom = Some(denom);

        self
    }

    pub fn with_amount(&mut self, amount: u128) -> &mut Self {
        self.amount = Some(amount);

        self
    }

    pub fn with_recipient_addr(&mut self, addr: &'a str) -> &mut Self {
        self.recipient_addr = Some(addr);

        self
    }

    /// Sends the transaction.
    pub fn send(&mut self) -> Result<(), Error> {
        self.test_ctx.tx_mint_tokenfactory_token(
            self.chain_name
                .as_ref()
                .ok_or(Error::MissingBuilderParam(String::from("chain_name")))?,
            self.key
                .ok_or(Error::MissingBuilderParam(String::from("key")))?,
            self.denom
                .ok_or(Error::MissingBuilderParam(String::from("denom")))?,
            self.amount
                .ok_or(Error::MissingBuilderParam(String::from("amount")))?,
            self.recipient_addr,
        )
    }
}

impl TestContext {
    pub fn build_tx_create_tokenfactory_token(&mut self) -> CreateTokenFactoryTokenTxBuilder {
        CreateTokenFactoryTokenTxBuilder {
            key: Some(DEFAULT_KEY),
            chain_name: Some(NEUTRON_CHAIN_NAME.to_owned()),
            subdenom: Default::default(),
            test_ctx: self,
        }
    }

    /// Creates a tokenfactory token with the given subdenom on the given chain.
    pub fn tx_create_tokenfactory_token(
        &mut self,
        chain_name: &str,
        key: &str,
        subdenom: &str,
    ) -> Result<(), Error> {
        let chain = self.get_chain(chain_name);
        let fee_denom = chain.native_denom.as_str();

        let receipt = chain.rb.tx(
            format!("tx tokenfactory create-denom {subdenom} --from {key} --fees 25000{fee_denom} --gas 10000000")
                .as_str(),
            true,
        )?;

        self.guard_tx_errors(
            chain_name,
            receipt
                .get("txhash")
                .and_then(|receipt| receipt.as_str())
                .ok_or(Error::TxMissingLogs)?,
        )?;

        Ok(())
    }

    /// Creates a builder for a tx minting a quantity of a tokenfactory token on the specified chain.
    pub fn build_tx_mint_tokenfactory_token(&mut self) -> MintTokenFactoryTokenTxBuilder {
        MintTokenFactoryTokenTxBuilder {
            key: Some(DEFAULT_KEY),
            chain_name: Some(NEUTRON_CHAIN_NAME.to_owned()),
            denom: Default::default(),
            amount: Default::default(),
            recipient_addr: Default::default(),
            test_ctx: self,
        }
    }

    fn tx_mint_tokenfactory_token(
        &mut self,
        chain_name: &str,
        key: &str,
        denom: &str,
        amount: u128,
        recipient: Option<&str>,
    ) -> Result<(), Error> {
        let chain = self.get_chain(chain_name);
        let fee_denom = chain.native_denom.as_str();

        if let Some(recipient) = recipient {
            let receipt = chain.rb.tx(
                format!("tx tokenfactory mint {amount}{denom} {recipient} --from {key} --fees 500{fee_denom}")
                    .as_str(),
                true,
            )?;

            self.guard_tx_errors(
                chain_name,
                receipt
                    .get("txhash")
                    .and_then(|receipt| receipt.as_str())
                    .ok_or(Error::TxMissingLogs)?,
            )?;

            return Ok(());
        }

        let receipt = chain.rb.tx(
            format!("tx tokenfactory mint {amount}{denom} --from {key} --fees 500{fee_denom}")
                .as_str(),
            true,
        )?;

        self.guard_tx_errors(
            chain_name,
            receipt
                .get("txhash")
                .and_then(|receipt| receipt.as_str())
                .ok_or(Error::TxMissingLogs)?,
        )?;

        Ok(())
    }
}
