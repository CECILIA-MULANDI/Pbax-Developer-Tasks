mod balances;
mod system;
mod types {
	pub type AccountID = String;
	pub type Balance = u128;
	pub type BlockNumber = u32;
	pub type Nonce = u32;
}
#[derive(Debug)]
pub struct Runtime {
	system: system::Pallet<types::AccountID, types::BlockNumber, types::Nonce>,
	balances: balances::Pallet<types::AccountID, types::Balance>,
}

impl Runtime {
	// Create a new instance of the main Runtime, by creating a new instance of each pallet.
	fn new() -> Self {
		Self { system: system::Pallet::new(), balances: balances::Pallet::new() }
	}
}
fn main() {
	// creating a sample runtime
	let mut runtime = Runtime::new();
	let alice = "Alice".to_string();
	let bob = "bob".to_string();
	let charlie = "charlie".to_string();
	// setting a balance for a sample user
	runtime.balances.set_balance(&alice, 100);

	// increment block number & assert correct block_number
	runtime.system.inc_block_number();
	assert_eq!(runtime.system.block_number(), 1);

	// make transcations - first one

	runtime.system.inc_nonce(&alice);
	let _transaction_one = runtime
		.balances
		.transfer(alice.clone(), bob, 30)
		.map_err(|e| eprintln!("{}", e));

	// make transcations - second one
	runtime.system.inc_nonce(&alice);
	let _transaction_two =
		runtime.balances.transfer(alice, charlie, 20).map_err(|e| eprintln!("{}", e));

	/* TODO: Print the final runtime state after all transactions. */
	println!("The current runtime is: {:#?}", runtime);
}
