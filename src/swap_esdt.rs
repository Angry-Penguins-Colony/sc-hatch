#![no_std]

elrond_wasm::imports!();

pub const ERR_BAD_NONCE: &str = "The token nonce sent is not the one expected";
pub const ERR_BAD_TOKEN: &str = "The token identifier sent is not the one expected";
pub const ERR_NO_OUTPUT_TOKEN: &str = "There is nothing to swap. The balance is empty.";

/// An empty contract. To be used as a template when starting a new contract from scratch.
#[elrond_wasm::derive::contract]
pub trait SwapEsdt {
    #[storage_mapper("input_token")]
    fn input_token(&self) -> SingleValueMapper<TokenIdentifier>;

    #[storage_mapper("input_nonce")]
    fn input_nonce(&self) -> SingleValueMapper<u64>;

    #[storage_mapper("output_token")]
    fn output_token(&self) -> SingleValueMapper<TokenIdentifier>;

    #[storage_mapper("output_nonce")]
    fn output_nonce(&self) -> SingleValueMapper<u64>;

    #[init]
    fn init(
        &self,
        input_token: TokenIdentifier,
        input_nonce: u64,
        output_token: TokenIdentifier,
        output_nonce: u64,
    ) {
        self.input_token().set(input_token);
        self.input_nonce().set(input_nonce);
        self.output_token().set(output_token);
        self.output_nonce().set(output_nonce);
    }

    #[endpoint(hatch)]
    #[payable("*")]
    fn swap(
        &self,
        #[payment] payment: BigUint,
        #[payment_token] token: TokenIdentifier,
        #[payment_nonce] nonce: u64,
    ) {
        sc_panic!("Not implemented");
    }

    #[endpoint]
    #[only_owner]
    fn claim_eggs(&self) {
        sc_panic!("Not implemented");
    }
}
