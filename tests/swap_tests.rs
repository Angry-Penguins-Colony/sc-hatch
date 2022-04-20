mod setup;

use elrond_wasm_debug::rust_biguint;
use setup::*;

#[test]
fn should_swap() {
    let mut setup = setup_contract(sc_swap_esdt::contract_obj);

    setup.fill_output(1u64);

    let input_token = setup.input_token;
    setup.swap(&input_token, setup.input_nonce).assert_ok();

    assert_eq!(
        setup.blockchain_wrapper.get_esdt_balance(
            &setup.user_lambda,
            &input_token,
            setup.input_nonce
        ),
        rust_biguint!(0)
    );

    assert_eq!(
        setup.blockchain_wrapper.get_esdt_balance(
            &setup.contract_wrapper.address_ref(),
            &input_token,
            setup.input_nonce
        ),
        rust_biguint!(1)
    );

    assert_eq!(
        setup.blockchain_wrapper.get_esdt_balance(
            &setup.user_lambda,
            &setup.output_token.clone(),
            setup.output_nonce
        ),
        rust_biguint!(1)
    );
}

#[test]
fn should_err_bad_nonce() {
    let mut setup = setup_contract(sc_swap_esdt::contract_obj);

    setup.fill_output(1u64);

    let token_id = setup.input_token;
    setup
        .swap(&token_id, setup.input_nonce + 1)
        .assert_user_error(sc_swap_esdt::ERR_SWAP_BAD_NONCE);
}

#[test]
fn should_err_bad_token() {
    let mut setup = setup_contract(sc_swap_esdt::contract_obj);

    setup.fill_output(1u64);

    let bad_token = b"HEENOK-667";

    assert_ne!(bad_token.len(), setup.input_token.len());

    setup
        .swap(bad_token, setup.input_nonce)
        .assert_user_error(sc_swap_esdt::ERR_SWAP_BAD_TOKEN);
}

#[test]
fn should_err_no_output_token() {
    let mut setup = setup_contract(sc_swap_esdt::contract_obj);

    let token_id = setup.input_token;
    setup
        .swap(&token_id, setup.input_nonce)
        .assert_user_error(sc_swap_esdt::ERR_SWAP_NO_OUTPUT_TOKEN);
}
