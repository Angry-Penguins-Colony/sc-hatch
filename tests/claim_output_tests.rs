mod setup;

use elrond_wasm_debug::rust_biguint;
use setup::*;

#[test]
fn claim_output() {
    let mut setup = setup_contract(sc_swap_esdt::contract_obj);

    let nonce = 1u64;

    setup.fill_output(1u64, nonce, 1u64);
    setup.fill_output(1u64, nonce + 1, 2u64);

    setup
        .claim_outputs(&setup.owner_address.clone(), 1u64)
        .assert_ok();

    assert_eq!(
        setup
            .blockchain_wrapper
            .get_esdt_balance(&setup.owner_address, &setup.output_token, nonce),
        rust_biguint!(1u64)
    );

    assert_eq!(
        setup.blockchain_wrapper.get_esdt_balance(
            &setup.contract_wrapper.address_ref(),
            &setup.output_token,
            nonce
        ),
        rust_biguint!(0u64)
    );

    assert_eq!(
        setup.blockchain_wrapper.get_esdt_balance(
            &setup.contract_wrapper.address_ref(),
            &setup.output_token,
            nonce + 1
        ),
        rust_biguint!(1u64)
    );
}

#[test]
fn claim_output_while_not_owned() {
    let mut setup = setup_contract(sc_swap_esdt::contract_obj);

    let nonce = 1u64;

    setup.blockchain_wrapper.set_nft_balance(
        &setup.contract_wrapper.address_ref(),
        &setup.output_token.clone(),
        nonce,
        &rust_biguint!(5u64),
        &{},
    );

    setup
        .claim_outputs(&setup.user_lambda.clone(), 1u64)
        .assert_user_error(sc_swap_esdt::ERR_NOT_OWNER);
}
