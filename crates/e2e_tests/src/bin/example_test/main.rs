use test_runner::chain;
use test_runner::client;
use test_runner::env;
use test_runner::wallet;

mod test;

const TENDERMINT_RPC_ENV_VAR: &str = "ANOMA_LEDGER_ADDRESS";

fn main() {
    chain::join();

    let vp_implicit_alias = wallet::random_alias("vp-implicit");
    let vp_alias = wallet::random_alias("vp-established");
    let owner_implicit_alias = wallet::random_alias("owner-implicit");
    let owner_alias = wallet::random_alias("owner-established");

    let ledger_address = env::get_var_or_die(TENDERMINT_RPC_ENV_VAR);
    let current_dir = std::env::current_dir().unwrap();

    let vp_example_path = format!("{}/wasm/vp_example.wasm", current_dir.to_string_lossy());
    let _tx_example_path = format!("{}/wasm/tx_example.wasm", current_dir.to_string_lossy());

    let client = client::Client::new(&ledger_address);

    chain::provision_chain(
        &client,
        &vp_example_path,
        &vp_implicit_alias,
        &vp_alias,
        &owner_implicit_alias,
        &owner_alias,
    );

    match test::run(&client, &vp_implicit_alias, &vp_alias, &owner_alias) {
        Ok(passed) => {
            if passed {
                std::process::exit(0);
            } else {
                std::process::exit(2);
            }
        }
        Err(err) => {
            eprintln!("Error while running test: {:?}", err);
            std::process::exit(1)
        }
    };
}
