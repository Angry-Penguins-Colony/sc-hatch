use elrond_wasm::{
    contract_base::ContractBase,
    types::{Address, TokenIdentifier},
};
use elrond_wasm_debug::{rust_biguint, testing_framework::*, tx_mock::TxResult, DebugApi};
use sc_swap_esdt::SwapEsdt;

#[allow(dead_code)]
const WASM_PATH: &'static str = "output/swap_esdt.wasm";

#[allow(dead_code)]
pub struct ContractSetup<ContractObjBuilder>
where
    ContractObjBuilder: 'static + Copy + Fn() -> sc_swap_esdt::ContractObj<DebugApi>,
{
    pub blockchain_wrapper: BlockchainStateWrapper,
    pub owner_address: Address,
    pub user_lambda: Address,
    pub contract_wrapper:
        ContractObjWrapper<sc_swap_esdt::ContractObj<DebugApi>, ContractObjBuilder>,
    pub input_token: [u8; 11],
    pub input_nonce: u64,
    pub output_token: [u8; 13],
    pub output_nonce: u64,
}

impl<ContractObjBuilder> ContractSetup<ContractObjBuilder>
where
    ContractObjBuilder: 'static + Copy + Fn() -> sc_swap_esdt::ContractObj<DebugApi>,
{
    #[allow(dead_code)]
    pub fn swap(&mut self, token_id: &[u8], nonce: u64) -> TxResult {
        self.blockchain_wrapper.set_nft_balance(
            &self.user_lambda,
            token_id,
            nonce,
            &rust_biguint!(1),
            &{},
        );

        return self.blockchain_wrapper.execute_esdt_transfer(
            &self.user_lambda,
            &self.contract_wrapper,
            token_id,
            nonce,
            &rust_biguint!(1),
            |sc| {
                sc.swap(
                    sc.call_value().esdt_value(),
                    sc.call_value().token(),
                    sc.call_value().esdt_token_nonce(),
                )
            },
        );
    }

    #[allow(dead_code)]
    pub fn fill_manual(
        &mut self,
        address: &Address,
        token_id: &[u8],
        nonce: u64,
        balance: u64,
    ) -> TxResult {
        self.blockchain_wrapper.set_nft_balance(
            address,
            token_id,
            nonce,
            &rust_biguint!(balance),
            &{},
        );

        return self.blockchain_wrapper.execute_esdt_transfer(
            address,
            &self.contract_wrapper,
            token_id,
            nonce,
            &rust_biguint!(balance),
            |sc| {
                sc.fill(
                    sc.call_value().esdt_value(),
                    sc.call_value().token(),
                    sc.call_value().esdt_token_nonce(),
                )
            },
        );
    }

    #[allow(dead_code)]
    pub fn fill(&mut self, balance: u64) {
        self.fill_manual(
            &self.owner_address.clone(),
            &self.output_token.clone(),
            self.output_nonce.clone(),
            balance,
        )
        .assert_ok();
    }
}

#[allow(dead_code)]
pub fn setup_contract<ContractObjBuilder>(
    cf_builder: ContractObjBuilder,
) -> ContractSetup<ContractObjBuilder>
where
    ContractObjBuilder: 'static + Copy + Fn() -> sc_swap_esdt::ContractObj<DebugApi>,
{
    DebugApi::dummy();

    let input_token = *b"INPUT-00000";
    let input_nonce = 1;
    let output_token = *b"OUTPUT-aaaaaa";
    let output_nonce = 1;

    let rust_zero = rust_biguint!(0u64);
    let mut blockchain_wrapper = BlockchainStateWrapper::new();
    let owner_address = blockchain_wrapper.create_user_account(&rust_zero);
    let user_lambda = blockchain_wrapper.create_user_account(&rust_zero);
    let cf_wrapper = blockchain_wrapper.create_sc_account(
        &rust_zero,
        Some(&owner_address),
        cf_builder,
        WASM_PATH,
    );

    blockchain_wrapper
        .execute_tx(&owner_address, &cf_wrapper, &rust_zero, |sc| {
            sc.init(
                TokenIdentifier::from_esdt_bytes(&input_token),
                input_nonce,
                TokenIdentifier::from_esdt_bytes(&output_token),
                output_nonce,
            );
        })
        .assert_ok();

    blockchain_wrapper.add_mandos_set_account(cf_wrapper.address_ref());

    ContractSetup {
        blockchain_wrapper,
        owner_address,
        contract_wrapper: cf_wrapper,
        input_token,
        input_nonce,
        output_token,
        output_nonce,
        user_lambda,
    }
}
