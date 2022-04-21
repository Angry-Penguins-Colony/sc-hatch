#![no_std]

elrond_wasm::imports!();

pub const ERR_NOT_OWNER: &str = "Endpoint can only be called by owner";

pub const ERR_SWAP_BAD_NONCE: &str = "The token nonce sent is not the one expected";
pub const ERR_SWAP_BAD_TOKEN: &str = "The token identifier sent is not the one expected";
pub const ERR_SWAP_NO_OUTPUT_TOKEN: &str = "There is nothing to swap. The balance is empty.";

pub const ERR_FILL_BAD_NONCE: &str = "The token nonce sent is not the one expected";
pub const ERR_FILL_BAD_TOKEN: &str = "The token identifier sent is not the one expected";
pub const ERR_FILL_BAD_PAYMENT: &str = "You can only fill with NFT.";

pub const ERR_CLAIM_INPUT_BALANCE_EMPTY: &str = "The input balance is empty.";

/// An empty contract. To be used as a template when starting a new contract from scratch.
#[elrond_wasm::derive::contract]
pub trait SwapEsdt {
    #[storage_mapper("input_token")]
    fn input_token(&self) -> SingleValueMapper<TokenIdentifier>;

    #[storage_mapper("input_nonce")]
    fn input_nonce(&self) -> SingleValueMapper<u64>;

    #[storage_mapper("output_token")]
    fn output_token(&self) -> SingleValueMapper<TokenIdentifier>;

    #[storage_mapper("available_penguin_nonce")]
    fn available_output_nonce(&self) -> VecMapper<u64>;

    #[init]
    fn init(&self, input_token: TokenIdentifier, input_nonce: u64, output_token: TokenIdentifier) {
        self.input_token().set(input_token);
        self.input_nonce().set(input_nonce);
        self.output_token().set(output_token);
    }

    #[endpoint(hatch)]
    #[payable("*")]
    fn swap(
        &self,
        #[payment] payment: BigUint,
        #[payment_token] token: TokenIdentifier,
        #[payment_nonce] nonce: u64,
    ) {
        require!(token == self.input_token().get(), ERR_SWAP_BAD_TOKEN);
        require!(nonce == self.input_nonce().get(), ERR_SWAP_BAD_NONCE);

        for _ in 0u64..payment.to_u64().unwrap() {
            let nonce = self.get_random_nonce();

            let output_balance = self
                .blockchain()
                .get_sc_balance(&self.output_token().get(), nonce);

            require!(output_balance >= payment, ERR_SWAP_NO_OUTPUT_TOKEN);

            let caller = self.blockchain().get_caller();
            self.send().direct(
                &caller,
                &self.output_token().get(),
                nonce,
                &BigUint::from(1u32),
                &[],
            );
        }
    }

    fn get_random_nonce(&self) -> u64 {
        let available_nfts_len = self.available_output_nonce().len();

        require!(available_nfts_len > 0, ERR_SWAP_NO_OUTPUT_TOKEN);

        let mut rand_source = RandomnessSource::<Self::Api>::new();
        let random_index = rand_source.next_usize_in_range(0, available_nfts_len) + 1;
        let random_nonce = self.available_output_nonce().get(random_index);

        self.available_output_nonce().swap_remove(random_index);

        return random_nonce;
    }

    #[endpoint]
    #[only_owner]
    fn claim_inputs_tokens(&self) {
        self.claim_tokens(&self.input_token().get(), self.input_nonce().get());
    }

    #[endpoint]
    #[only_owner]
    fn claim_outputs_tokens(&self) {
        self.blockchain().check_caller_is_owner();

        let owner = self.blockchain().get_owner_address();
        let one = BigUint::from(1u32);

        for n in self.available_output_nonce().iter() {
            self.send()
                .direct(&owner, &self.output_token().get(), n, &one, &[]);
        }

        self.available_output_nonce().clear();
    }

    fn claim_tokens(&self, token: &TokenIdentifier, nonce: u64) {
        self.blockchain().check_caller_is_owner();

        let balance = self.blockchain().get_sc_balance(token, nonce);

        require!(balance > 0, ERR_CLAIM_INPUT_BALANCE_EMPTY);

        let owner = self.blockchain().get_owner_address();
        self.send().direct(&owner, token, nonce, &balance, &[]);
    }

    #[endpoint]
    #[payable("*")]
    #[only_owner]
    fn fill_output(
        &self,
        #[payment] payment: BigUint,
        #[payment_token] token: TokenIdentifier,
        #[payment_nonce] nonce: u64,
    ) {
        self.blockchain().check_caller_is_owner();

        require!(token == self.output_token().get(), ERR_FILL_BAD_TOKEN);
        require!(payment == BigUint::from(1u32), ERR_FILL_BAD_PAYMENT);

        self.available_output_nonce().push(&nonce);
    }
}
