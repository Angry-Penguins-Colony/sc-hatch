mod setup;

use elrond_wasm_debug::rust_biguint;
use setup::*;

#[test]
fn should_swap() {
    let mut setup = setup_contract(sc_swap_esdt::contract_obj);

    let nonce = 1u64;
    setup.fill_output(1u64, 1u64, 1u64);

    let input_token = setup.input_token;
    setup.swap(&input_token, setup.input_nonce, 1).assert_ok();

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
            nonce
        ),
        rust_biguint!(1)
    );
}

#[test]
fn should_swap_twice() {
    let mut setup = setup_contract(sc_swap_esdt::contract_obj);

    setup.fill_output(1u64, 1u64, setup.input_nonce);
    setup.fill_output(1u64, 2u64, setup.input_nonce);

    let input_token = setup.input_token;
    setup.swap(&input_token, setup.input_nonce, 2).assert_ok();

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
        rust_biguint!(2)
    );

    assert_eq!(
        setup.blockchain_wrapper.get_esdt_balance(
            &setup.user_lambda,
            &setup.output_token.clone(),
            1u64
        ),
        rust_biguint!(1)
    );

    assert_eq!(
        setup.blockchain_wrapper.get_esdt_balance(
            &setup.user_lambda,
            &setup.output_token.clone(),
            2u64
        ),
        rust_biguint!(1)
    );
}

#[test]
fn should_swap_five() {
    let mut setup = setup_contract(sc_swap_esdt::contract_obj);

    let nonces = [1, 2, 3];

    for n in nonces {
        setup.fill_output(1u64, n, 1u64);
    }

    let input_token = setup.input_token;

    for _ in nonces {
        setup.swap(&input_token, setup.input_nonce, 1).assert_ok();
    }

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
        rust_biguint!(nonces.len())
    );

    for n in nonces {
        assert_eq!(
            setup.blockchain_wrapper.get_esdt_balance(
                &setup.user_lambda,
                &setup.output_token.clone(),
                n
            ),
            rust_biguint!(1)
        );
    }
}

#[test]
fn should_err_bad_token() {
    let mut setup = setup_contract(sc_swap_esdt::contract_obj);

    setup.fill_output(1u64, 1u64, 1u64);

    let bad_token = b"HEENOK-667";

    assert_ne!(bad_token.len(), setup.input_token.len());

    setup
        .swap(bad_token, setup.input_nonce, 1)
        .assert_user_error(sc_swap_esdt::ERR_SWAP_BAD_TOKEN);
}

#[test]
fn should_err_no_output_token() {
    let mut setup = setup_contract(sc_swap_esdt::contract_obj);

    let token_id = setup.input_token;
    setup
        .swap(&token_id, setup.input_nonce, 1)
        .assert_user_error(sc_swap_esdt::ERR_SWAP_NO_OUTPUT_TOKEN);
}
