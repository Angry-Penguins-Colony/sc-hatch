mod setup;

use elrond_wasm_debug::rust_biguint;
use setup::*;

#[test]
fn fill() {
    let mut setup = setup_contract(sc_swap_esdt::contract_obj);

    setup
        .fill_output_manual(
            &setup.owner_address.clone(),
            &setup.output_token.clone(),
            setup.output_nonce.clone(),
            1u64,
        )
        .assert_ok();

    assert_eq!(
        setup.blockchain_wrapper.get_esdt_balance(
            &setup.owner_address,
            &setup.output_token,
            setup.output_nonce
        ),
        rust_biguint!(1u64)
    );
}

#[test]
fn fill_while_not_owner() {
    let mut setup = setup_contract(sc_swap_esdt::contract_obj);

    setup
        .fill_output_manual(
            &setup.user_lambda.clone(),
            &setup.output_token.clone(),
            setup.output_nonce.clone(),
            1u64,
        )
        .assert_user_error(sc_swap_esdt::ERR_NOT_OWNER);
}

#[test]
fn fill_while_bad_nonce() {
    let mut setup = setup_contract(sc_swap_esdt::contract_obj);

    setup
        .fill_output_manual(
            &setup.user_lambda.clone(),
            &setup.output_token.clone(),
            setup.output_nonce.clone() + 1,
            1u64,
        )
        .assert_user_error(sc_swap_esdt::ERR_FILL_BAD_NONCE);
}

#[test]
fn fill_while_bad_token() {
    let mut setup = setup_contract(sc_swap_esdt::contract_obj);

    let bad_token = b"HEENOK-667";

    assert_ne!(bad_token.len(), setup.input_token.len());

    setup
        .fill_output_manual(
            &setup.user_lambda.clone(),
            bad_token,
            setup.output_nonce.clone(),
            1u64,
        )
        .assert_user_error(sc_swap_esdt::ERR_FILL_BAD_TOKEN);
}
