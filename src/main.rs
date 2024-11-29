
use crate::support::Dispatch;
mod balances;
mod proof_of_existence;
mod support;
mod system;

mod types {
    use crate::{support, RuntimeCall};

    pub type AccountId = String;
    pub type Balance = u128;
    pub type BlockNumber = u64;
    pub type Nonce = u64;
    pub type Extrinsic = support::Extrinsic<AccountId, RuntimeCall>;
    pub type Header = support::Header<BlockNumber>;
    pub type Block = support::Block<Header, Extrinsic>;
    pub type Content = &'static str;
}

impl system::Config for Runtime {
    type AccountId = types::AccountId;
    type BlockNumber = types::BlockNumber;
    type Nonce = types::Nonce;
}
impl balances::Config for Runtime {
    type Balance = types::Balance;
}
impl proof_of_existence::Config for Runtime {
    type Content = types::Content;
}

#[derive(Debug)]
#[macros::runtime]
pub struct Runtime {
    system: system::Pallet<Runtime>,
    balances: balances::Pallet<Runtime>,
    proof_of_existence: proof_of_existence::Pallet<Runtime>,
}

fn main() {
    print!("hello");

    let mut runtime = Runtime::new();

    let salikur = "salikur".to_string();
    let salima = "salima".to_string();
    let kashiat = "kashiat".to_string();
    runtime.balances.set_balance(&salikur, 100);

    let block_1 = types::Block {
        header: support::Header { block_number: 1 },
        extrinsics: vec![
            support::Extrinsic {
                caller: salikur.clone(),
                call: RuntimeCall::balances(balances::Call::transfer {
                    to: kashiat.clone(),
                    ammount: 40,
                }),
            },
            support::Extrinsic {
                caller: kashiat.clone(),
                call: RuntimeCall::balances(balances::Call::transfer {
                    to: salima.clone(),
                    ammount: 20,
                }),
            },
        ],
    };
    let block_2 = types::Block {
        header: support::Header { block_number: 2 },
        extrinsics: vec![
            support::Extrinsic {
                caller: salima.clone(),
                call: RuntimeCall::proof_of_existence(proof_of_existence::Call::create_claim {
                    claim: "my_document",
                }),
            },
            support::Extrinsic {
                caller: salikur.clone(),
                call: RuntimeCall::proof_of_existence(proof_of_existence::Call::create_claim {
                    claim: "my_document 2",
                }),
            },
        ],
    };

    runtime
        .execute_block(block_1)
        .expect("worong block execution");
    runtime.execute_block(block_2);

    print!("{:#?}", runtime);
}
