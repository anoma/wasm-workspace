use eyre::Result;

use test_runner::client;

pub(crate) fn run(
    _client: &client::Client,
    _vp_implicit_alias: &str,
    _vp_alias: &str,
    _owner_alias: &str,
) -> Result<bool> {
    // TODO: execute the example tx
    println!("TEST");

    Ok(true)
}
