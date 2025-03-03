mod balances;
mod system;

mod types {
	pub type AccountId = String;
	pub type Balance = u128;
	pub type BlockNumber = u32;
	pub type Nonce = u32;
}

#[derive(Debug)]
pub struct Runtime {
	system: system::Pallet<Self>,
	balances: balances::Pallet<Self>,
}

impl system::Config for Runtime {
	type AccountId = types::AccountId;
	type BlockNumber = types::BlockNumber;
	type Nonce = types::Nonce;
}

impl balances::Config for Runtime {
	type AccountId = types::AccountId;
	type Balance = types::Balance;
}

impl Runtime {
	fn new() -> Self {
		Self { system: system::Pallet::new(), balances: balances::Pallet::new() }
	}
}

fn main() {
	let mut runtime = Runtime::new();
	let kurniadi = "kurniadi".to_string();
	let ahmad = "ahmad".to_string();
	let wijaya = "wijaya".to_string();

	runtime.balances.set_balance(&kurniadi, 200000);
	runtime.system.inc_block_number();
	runtime.system.inc_nonce(&kurniadi);

	assert_eq!(runtime.system.block_number(), 1);

	let _res = runtime
		.balances
		.transfer(kurniadi.clone(), ahmad, 10000)
		.map_err(|e| eprintln!("{}", e));

	runtime.system.inc_nonce(&kurniadi);

	let _res = runtime
		.balances
		.transfer(kurniadi, wijaya, 50000)
		.map_err(|e| eprintln!("{}", e));

	let current_balance = runtime.balances.balance(&"kurniadi".to_string());

	println!("Kurniadi current balances: {}", current_balance);
	println!("{:#?}", runtime);
}
