use super::super::{
    super::{error::Error, DEFAULT_KEY, DEFAULT_TRANSFER_PORT, NEUTRON_CHAIN_NAME},
    test_context::{LocalChain, TestContext},
};

pub struct TransferTxBuilder<'a> {
    key: &'a str,
    src_chain_name: &'a str,
    recipient: Option<&'a str>,
    denom: Option<&'a str>,
    amount: Option<u128>,
    memo: Option<&'a str>,
    port: &'a str,
    test_ctx: &'a mut TestContext,
}

impl<'a> TransferTxBuilder<'a> {
    pub fn with_key(&mut self, key: &'a str) -> &mut Self {
        self.key = key;

        self
    }

    pub fn with_chain_name(&mut self, chain_name: &'a str) -> &mut Self {
        self.src_chain_name = chain_name;

        self
    }

    pub fn with_recipient(&mut self, recipient: &'a str) -> &mut Self {
        self.recipient = Some(recipient);

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

    pub fn with_port(&mut self, port: &'a str) -> &mut Self {
        self.port = port;

        self
    }

    pub fn with_memo(&mut self, memo: &'a str) -> &mut Self {
        self.memo = Some(memo);

        self
    }

    /// Sends the built IBC transfer tx.
    pub fn send(&mut self) -> Result<(), Error> {
        self.test_ctx.tx_transfer(
            self.key,
            self.src_chain_name,
            self.recipient
                .ok_or(Error::MissingBuilderParam(String::from("recipient")))?,
            self.denom
                .ok_or(Error::MissingBuilderParam(String::from("denom")))?,
            self.amount
                .ok_or(Error::MissingBuilderParam(String::from("amount")))?,
            self.port,
            self.memo,
        )
    }
}

impl TestContext {
    /// Creates a builder building a transaction transfering funds over IBC.
    pub fn build_tx_transfer(&mut self) -> TransferTxBuilder {
        TransferTxBuilder {
            key: DEFAULT_KEY,
            src_chain_name: NEUTRON_CHAIN_NAME,
            recipient: Default::default(),
            denom: Default::default(),
            amount: Default::default(),
            memo: None,
            port: DEFAULT_TRANSFER_PORT,
            test_ctx: self,
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn tx_transfer(
        &mut self,
        key: &str,
        src_chain_name: &str,
        recipient: &str,
        denom: &str,
        amount: u128,
        port: &str,
        memo: Option<&str>,
    ) -> Result<(), Error> {
        let dest_chain: &LocalChain = self
            .chains
            .values()
            .find(|chain| recipient.starts_with(&chain.chain_prefix))
            .ok_or(Error::MissingContextVariable(format!("chain::{recipient}")))?;

        let chain = self.get_chain(src_chain_name);
        let fee_denom = &chain.native_denom;

        let channel = self
            .transfer_channel_ids
            .get(&(chain.chain_name.clone(), dest_chain.chain_name.clone()))
            .ok_or(Error::MissingContextVariable(format!(
                "channel_id::{}-{}",
                chain.chain_name, dest_chain.chain_name
            )))?;

        let memo_part = memo.map(|m| format!(" --memo={}", m)).unwrap_or_default();

        let receipt = chain.rb.tx(&format!(
            "tx ibc-transfer transfer {port} {channel} {recipient} {amount}{denom} --fees=100000{fee_denom} --from={key}{}",
            memo_part
        ), true)?;

        self.guard_tx_errors(
            src_chain_name,
            receipt
                .get("txhash")
                .and_then(|receipt| receipt.as_str())
                .ok_or(Error::TxMissingLogs)?,
        )?;

        Ok(())
    }
}
