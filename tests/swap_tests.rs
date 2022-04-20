mod setup;

use setup::*;

#[test]
fn should_swap() {
    let mut setup = setup_contract(sc_swap_esdt::contract_obj);

    let token_id = setup.input_token;
    setup.swap(&token_id, setup.input_nonce).assert_ok();
}

#[test]
fn should_err_bad_nonce() {
    let mut setup = setup_contract(sc_swap_esdt::contract_obj);

    let token_id = setup.input_token;
    setup
        .swap(&token_id, setup.input_nonce + 1)
        .assert_user_error(sc_swap_esdt::ERR_BAD_NONCE);
}

#[test]
fn should_err_bad_token() {
    let mut setup = setup_contract(sc_swap_esdt::contract_obj);

    let bad_token = b"HEENOK-667";

    assert_ne!(bad_token.len(), setup.input_token.len());

    setup
        .swap(bad_token, setup.input_nonce)
        .assert_user_error(sc_swap_esdt::ERR_BAD_TOKEN);
}

#[test]
fn should_err_no_output_token() {
    let mut setup = setup_contract(sc_swap_esdt::contract_obj);

    let token_id = setup.input_token;
    setup
        .swap(&token_id, setup.input_nonce)
        .assert_user_error(sc_swap_esdt::ERR_NO_OUTPUT_TOKEN);
}
