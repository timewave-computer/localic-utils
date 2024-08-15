use super::super::{
    super::{error::Error, DEFAULT_KEY, NEUTRON_CHAIN_NAME},
    test_context::TestContext,
};
use cosmwasm_std::Coin;
use serde_json::Value;

pub struct Instantiate2TxBuilder<'a> {
    key: &'a str,
    chain_name: &'a str,
    admin: Option<&'a str>,
    code_id: Option<u64>,
    label: Option<&'a str>,
    msg: Option<Value>,
    funds: Option<Coin>,

    // Not automatically hex-encoded.
    // Assume the user performs hex encoding
    salt: Option<&'a str>,
    fix_msg: Option<bool>,
    test_ctx: &'a mut TestContext,
}

impl<'a> Instantiate2TxBuilder<'a> {
    pub fn with_key(&mut self, key: &'a str) -> &mut Self {
        self.key = key;

        self
    }

    pub fn with_chain_name(&mut self, chain_name: &'a str) -> &mut Self {
        self.chain_name = chain_name;

        self
    }

    pub fn with_admin(&mut self, admin: &'a str) -> &mut Self {
        self.admin = Some(admin);

        self
    }

    pub fn with_code_id(&mut self, code_id: u64) -> &mut Self {
        self.code_id = Some(code_id);

        self
    }

    pub fn with_label(&mut self, label: &'a str) -> &mut Self {
        self.label = Some(label);

        self
    }

    pub fn with_msg(&mut self, msg: Value) -> &mut Self {
        self.msg = Some(msg);

        self
    }

    pub fn with_funds(&mut self, funds: Coin) -> &mut Self {
        self.funds = Some(funds);

        self
    }

    /// Sets the salt. Value must be hex encoded.
    pub fn with_salt(&mut self, salt: &'a str) -> &mut Self {
        self.salt = Some(salt);

        self
    }

    pub fn with_fix_msg(&mut self, fix_msg: bool) -> &mut Self {
        self.fix_msg = Some(fix_msg);

        self
    }

    /// Sends the built instantiate 2 tx.
    pub fn send(&mut self) -> Result<(), Error> {
        self.test_ctx.tx_instantiate2(
            self.key,
            self.chain_name,
            self.admin,
            self.code_id.expect("missing builder param code_id"),
            self.label.expect("missing builder param label"),
            self.msg.as_ref().expect("missing builder param msg"),
            self.funds.as_ref(),
            self.salt.as_ref().expect("missing builder param salt"),
            self.fix_msg,
        )
    }
}

impl TestContext {
    pub fn build_tx_instantiate2<'a>(&'a mut self) -> Instantiate2TxBuilder<'a> {
        Instantiate2TxBuilder {
            key: DEFAULT_KEY,
            chain_name: NEUTRON_CHAIN_NAME,
            admin: None,
            code_id: None,
            label: None,
            msg: None,
            funds: None,
            salt: None,
            fix_msg: None,
            test_ctx: self,
        }
    }

    fn tx_instantiate2(
        &mut self,
        key: &str,
        chain_name: &str,
        admin: Option<&str>,
        code_id: u64,
        label: &str,
        msg: &Value,
        funds: Option<&Coin>,
        salt: &str,
        fix_msg: Option<bool>,
    ) -> Result<(), Error> {
        let chain = self.get_chain(chain_name);

        // Optional flags
        let admin_part = admin
            .map(|admin| format!("--admin {admin}"))
            .unwrap_or(String::from("--no-admin"));
        let amt_part = funds
            .map(|funds| format!("--amount {funds}"))
            .unwrap_or_default();
        let fix_msg_part = fix_msg
            .map(|fix_msg| format!("--fix_msg {fix_msg}"))
            .unwrap_or_default();

        let receipt = chain.rb.tx(
            &format!("tx wasm instantiate2 {code_id} {msg} {salt} --label {label} {admin_part} {amt_part} {fix_msg_part} --from {key}"),
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
