use eyre::Result;
use namada_vp_prelude::*;

const VP_NAME: &str = "vp_example";

fn log(msg: &str) {
    log_string(format!("[{}] {}", VP_NAME, msg))
}

#[validity_predicate]
fn validate_tx(
    ctx: &Ctx,
    tx_data: Vec<u8>,
    addr: Address,
    keys_changed: BTreeSet<storage::Key>,
    verifiers: BTreeSet<Address>,
) -> VpResult {
    log(&format!(
        "validate_tx called with addr: {}, keys_changed: {:#?}, tx_data: \
         {} bytes, verifiers: {:?}",
        addr,
        keys_changed,
        tx_data.len(),
        verifiers
    ));

    match validate_tx_aux(ctx, tx_data, addr, keys_changed, verifiers) {
        Ok(result) => Ok(result),
        Err(err) => {
            log(&format!("ERROR: {:?}", err));
            panic!("{:?}", err); // TODO: don't panic
        }
    }
}

fn validate_tx_aux(
    ctx: &Ctx,
    _tx_data: Vec<u8>,
    _vp_addr: Address,
    keys_changed: BTreeSet<storage::Key>,
    _verifiers: BTreeSet<Address>,
) -> Result<bool> {
    for key in keys_changed.iter() {
        let pre = ctx.read_bytes_pre(key);
        let post = ctx.read_bytes_post(key);
        log_string(format!(
            "validate_tx key: {}, pre: {:#?}, post: {:#?}",
            key, pre, post,
        ));
    }
    Ok(true)
}

#[cfg(test)]
mod test {
    use super::*;

    use std::str::FromStr;

    use namada_tests::{
        tx::{tx_host_env, TestTxEnv},
        vp::vp_host_env,
    };
    use namada_vp_prelude::{
        address, key::RefTo, storage, storage_api::StorageWrite, token::Amount, BTreeSet,
    };

    use namada::{proto::Tx, types::key::common::SecretKey};
    use rand::prelude::ThreadRng;

    fn random_key() -> SecretKey {
        let mut rng: ThreadRng = rand::thread_rng();
        let sk: SecretKey = {
            use namada::types::key::{ed25519, SecretKey, SigScheme};
            ed25519::SigScheme::generate(&mut rng).try_to_sk().unwrap()
        };
        sk
    }

    #[test]
    fn test_no_op() {
        let mut tx_env = TestTxEnv::default();

        let vp_owner = address::testing::established_address_1();
        let user = address::testing::established_address_2();
        let token = address::nam();
        // allowance must be enough to cover the gas costs of any txs made in this test
        let allowance = Amount::from(10_000_000);

        tx_env.spawn_accounts([&vp_owner, &user, &token]);
        tx_env.credit_tokens(&user, &token, allowance);
        let privileged_sk = random_key();
        tx_env.write_public_key(&vp_owner, &privileged_sk.ref_to());

        vp_host_env::init_from_tx(vp_owner.clone(), tx_env, |_| {
            let key_under_vp = storage::Key::from_str(&format!("#{}", vp_owner.encode()))
                .unwrap()
                .push(&"some arbitary key segment".to_string())
                .unwrap();
            tx_host_env::ctx()
                .write(&key_under_vp, "some arbitrary value")
                .unwrap();
        });

        let tx = Tx::new(vec![], None).sign(&random_key());

        let vp_env = vp_host_env::take();
        let keys_changed: BTreeSet<storage::Key> = vp_env.all_touched_storage_keys();
        let verifiers: BTreeSet<Address> = BTreeSet::default();
        vp_host_env::set(vp_env);
        assert!(validate_tx(
            vp_host_env::ctx(),
            tx.data.unwrap(),
            vp_owner,
            keys_changed,
            verifiers
        )
        .unwrap());
    }
}
