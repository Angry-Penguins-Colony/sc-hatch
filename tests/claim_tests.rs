mod setup;

use elrond_wasm_debug::rust_biguint;
use setup::*;

#[test]
fn claim() {
    let mut setup = setup_contract(sc_swap_esdt::contract_obj);

    setup.fill(5u64);

    setup.claim(&setup.owner_address.clone()).assert_ok();

    assert_eq!(
        setup.blockchain_wrapper.get_esdt_balance(
            &setup.owner_address,
            &setup.output_token,
            setup.output_nonce
        ),
        rust_biguint!(5u64)
    );

    assert_eq!(
        setup.blockchain_wrapper.get_esdt_balance(
            &setup.contract_wrapper.address_ref(),
            &setup.output_token,
            setup.output_nonce
        ),
        rust_biguint!(0u64)
    );
}

#[test]
fn claim_while_not_owned() {
    let mut setup = setup_contract(sc_swap_esdt::contract_obj);

    setup.fill(5u64);

    setup
        .claim(&setup.owner_address.clone())
        .assert_user_error(sc_swap_esdt::ERR_NOT_OWNER);
}
