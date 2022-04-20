mod setup;

use elrond_wasm::contract_base::ContractBase;
use elrond_wasm_debug::rust_biguint;
use sc_swap_esdt::SwapEsdt;
use setup::*;

#[test]
fn should_swap() {
    let mut setup = setup_contract(sc_swap_esdt::contract_obj);

    setup.blockchain_wrapper.set_nft_balance(
        &setup.owner_address,
        &setup.input_token,
        setup.input_nonce,
        &rust_biguint!(1),
        &{},
    );

    setup
        .blockchain_wrapper
        .execute_esdt_transfer(
            &setup.owner_address,
            &setup.contract_wrapper,
            &setup.input_token,
            setup.input_nonce,
            &rust_biguint!(1),
            |sc| {
                sc.swap(
                    sc.call_value().esdt_value(),
                    sc.call_value().token(),
                    sc.call_value().esdt_token_nonce(),
                )
            },
        )
        .assert_ok();
}
