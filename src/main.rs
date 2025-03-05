mod balances;
mod proof_of_existence;
mod system;
mod utils;

use crate::utils::Dispatch;

mod types {
	pub type AccountId = String;
	pub type Balance = u128;
	pub type BlockNumber = u32;
	pub type Nonce = u32;
	pub type Extrinsic = crate::utils::Extrinsic<AccountId, crate::RuntimeCall>;
	pub type Header = crate::utils::Header<BlockNumber>;
	pub type Block = crate::utils::Block<Header, Extrinsic>;
	pub type Content = &'static str;
}

#[derive(Debug)]
pub struct Runtime {
	system: system::Pallet<Self>,
	balances: balances::Pallet<Self>,
	proof_of_existence: proof_of_existence::Pallet<Self>,
}

pub enum RuntimeCall {
	Balances(balances::Call<Runtime>),
	ProofOfExistence(proof_of_existence::Call<Runtime>),
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

impl crate::utils::Dispatch for Runtime {
	type Caller = <Runtime as system::Config>::AccountId;
	type Call = RuntimeCall;

	fn dispatch(
		&mut self,
		caller: Self::Caller,
		runtime_call: Self::Call,
	) -> utils::DispatchResult {
		match runtime_call {
			RuntimeCall::Balances(call) => {
				self.balances.dispatch(caller, call)?;
			},
			RuntimeCall::ProofOfExistence(call) => {
				self.proof_of_existence.dispatch(caller, call)?;
			},
		}

		Ok(())
	}
}

impl Runtime {
	fn new() -> Self {
		Self {
			system: system::Pallet::new(),
			balances: balances::Pallet::new(),
			proof_of_existence: proof_of_existence::Pallet::new(),
		}
	}

	fn execute_block(&mut self, block: types::Block) -> utils::DispatchResult {
		self.system.inc_block_number();

		if block.header.block_number != self.system.block_number() {
			return Err("block number does not match what is expected");
		}

		for (i, utils::Extrinsic { caller, call }) in block.extrinsics.into_iter().enumerate() {
			self.system.inc_nonce(&caller);
			let _res = self.dispatch(caller, call).map_err(|e| {
				eprintln!(
					"Extrinsic Error\n\tBlock Number: {}\n\tExtrinsic Number: {}\n\tError: {}",
					block.header.block_number, i, e
				)
			});
		}

		Ok(())
	}
}

fn main() {
	let mut runtime = Runtime::new();
	let kurniadi = "Kurniadi".to_string();
	let ahmad = "Ahmad".to_string();
	let wijaya = "Wijaya".to_string();

	runtime.balances.set_balance(&kurniadi, 200000);

	let block_1 = types::Block {
		header: utils::Header { block_number: 1 },
		extrinsics: vec![
			utils::Extrinsic {
				caller: kurniadi.clone(),
				call: RuntimeCall::Balances(balances::Call::Transfer {
					to: ahmad.clone(),
					amount: 50000,
				}),
			},
			utils::Extrinsic {
				caller: kurniadi,
				call: RuntimeCall::Balances(balances::Call::Transfer {
					to: wijaya.clone(),
					amount: 20000,
				}),
			},
		],
	};

	let block_2 = types::Block {
		header: utils::Header { block_number: 2 },
		extrinsics: vec![
			utils::Extrinsic {
				caller: ahmad.clone(),
				call: RuntimeCall::ProofOfExistence(proof_of_existence::Call::CreateClaim {
					claim: "Hello, world!",
				}),
			},
			utils::Extrinsic {
				caller: wijaya.clone(),
				call: RuntimeCall::ProofOfExistence(proof_of_existence::Call::CreateClaim {
					claim: "Hello, world!",
				}),
			},
		],
	};

	let block_3 = types::Block {
		header: utils::Header { block_number: 3 },
		extrinsics: vec![
			utils::Extrinsic {
				caller: ahmad,
				call: RuntimeCall::ProofOfExistence(proof_of_existence::Call::RevokeClaim {
					claim: "Hello, world!",
				}),
			},
			utils::Extrinsic {
				caller: wijaya,
				call: RuntimeCall::ProofOfExistence(proof_of_existence::Call::CreateClaim {
					claim: "Hello, world!",
				}),
			},
		],
	};

	runtime.execute_block(block_1).expect("invalid block");
	runtime.execute_block(block_2).expect("invalid block");
	runtime.execute_block(block_3).expect("invalid block");

	println!("{:#?}", runtime);
}
