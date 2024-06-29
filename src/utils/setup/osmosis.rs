use super::super::{
    super::{error::Error, DEFAULT_KEY, OSMOSIS_CHAIN_NAME},
    test_context::TestContext,
};
use cosmwasm_std::Decimal;

pub struct CreateOsmoPoolTxBuilder<'a> {
    key: &'a str,
    weights: Vec<(u64, &'a str)>,
    initial_deposit: Vec<(u64, &'a str)>,
    swap_fee: Decimal,
    exit_fee: Decimal,
    future_governor: &'a str,
    test_ctx: &'a mut TestContext,
}

impl<'a> CreateOsmoPoolTxBuilder<'a> {
    pub fn with_key(&mut self, key: &'a str) -> &mut Self {
        self.key = key;

        self
    }

    pub fn with_weight(&mut self, denom: &'a str, weight: u64) -> &mut Self {
        self.weights.push((weight, denom));

        self
    }

    pub fn with_initial_deposit(&mut self, denom: &'a str, deposit: u64) -> &mut Self {
        self.initial_deposit.push((deposit, denom));

        self
    }

    pub fn with_swap_fee(&mut self, swap_fee: Decimal) -> &mut Self {
        self.swap_fee = swap_fee;

        self
    }

    pub fn with_exit_fee(&mut self, exit_fee: Decimal) -> &mut Self {
        self.exit_fee = exit_fee;

        self
    }

    pub fn with_future_governor(&mut self, future_governor: &'a str) -> &mut Self {
        self.future_governor = future_governor;

        self
    }

    /// Sends the transaction, returning the pool ID if it was created successfully.
    pub fn send(&mut self) -> Result<u64, Error> {
        self.test_ctx.tx_create_osmo_pool(
            self.key,
            self.weights.iter().cloned(),
            self.initial_deposit.iter().cloned(),
            self.swap_fee,
            self.exit_fee,
            self.future_governor,
        )
    }
}

impl TestContext {
    pub fn build_tx_create_osmo_pool(&mut self) -> CreateOsmoPoolTxBuilder {
        CreateOsmoPoolTxBuilder {
            key: DEFAULT_KEY,
            weights: Default::default(),
            initial_deposit: Default::default(),
            swap_fee: Decimal::percent(0),
            exit_fee: Decimal::percent(0),
            future_governor: "128h",
            test_ctx: self,
        }
    }

    /// Creates an osmosis pool with the given denoms.
    fn tx_create_osmo_pool<'a>(
        &mut self,
        key: &str,
        weights: impl Iterator<Item = (u64, &'a str)>,
        initial_deposit: impl Iterator<Item = (u64, &'a str)>,
        swap_fee: Decimal,
        exit_fee: Decimal,
        future_governor: &'a str,
    ) -> Result<u64, Error> {
        let osmosis = self.get_chain(OSMOSIS_CHAIN_NAME);

        // Osmosisd requires a JSON file to specify the
        // configuration of the pool being created
        let poolfile_str = serde_json::json!({
            "weights": weights.map(|(weight, denom)| format!("{weight}{denom}")).collect::<Vec<_>>().join(","),
            "initial-deposit": initial_deposit.map(|(deposit, denom)| format!("{deposit}{denom}")).collect::<Vec<_>>().join(","),
            "swap-fee": swap_fee,
            "exit-fee": exit_fee,
            "future-governor": future_governor,
        })
        .to_string();

        let chain_id = &osmosis.rb.chain_id;
        let remote_poolfile_path = format!("/var/cosmos-chain/{chain_id}/pool_file.json");

        // Write the poolfile to a file
        let res = osmosis.rb.exec(
            &format!("/bin/sh -c 'echo <<-END {poolfile_str} > {remote_poolfile_path}'"),
            true,
        );

        println!("{}", res);

        // Create pool
        let res = osmosis.rb.tx(
            format!("tx poolmanager create-pool --pool-file {remote_poolfile_path} --from {key} --fees 500uosmo")
            .as_str(),
            true,
        )?;

        println!("{}", res);

        Ok(0)
    }
}
