mod balances;
mod system;

#[derive(Debug)]
pub struct Runtime {
	system: system::Pallet,
	balances: balances::Pallet,
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
	assert_eq!(runtime.system.block_number(), 1);

	runtime.system.inc_nonce(&kurniadi);

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
