mod balances;
mod support;
mod system;
use crate::support::Dispatch;
mod types {
	pub type AccountID = String;
	pub type Balance = u128;
	pub type BlockNumber = u32;
	pub type Nonce = u32;
	pub type Extrinsic = crate::support::Extrinsic<AccountID, crate::RuntimeCall>;
	pub type Header = crate::support::Header<BlockNumber>;
	pub type Block = crate::support::Block<Header, Extrinsic>;
}
pub enum RuntimeCall {}
#[derive(Debug)]
pub struct Runtime {
	system: system::Pallet<Self>,
	balances: balances::Pallet<Self>,
}
impl system::Config for Runtime {
	type AccountID = types::AccountID;
	type BlockNumber = types::BlockNumber;
	type Nonce = types::Nonce;
}
impl balances::Config for Runtime {
	type Balance = types::Balance;
}

impl Runtime {
	// Create a new instance of the main Runtime, by creating a new instance of each pallet.
	fn new() -> Self {
		Self { system: system::Pallet::new(), balances: balances::Pallet::new() }
	}
	// Execute a block of extrinsics. Increments the block number.
	fn execute_block(&mut self, block: types::Block) -> support::DispatchResult {
		/* TODO:
			- Increment the system's block number.
			- Check that the block number of the incoming block matches the current block number,
			  or return an error.
			- Iterate over the extrinsics in the block...
				- Increment the nonce of the caller.
				- Dispatch the extrinsic using the `caller` and the `call` contained in the extrinsic.
				- Handle errors from `dispatch` same as we did for individual calls: printing any
				  error and capturing the result.
				- You can extend the error message to include information like the block number and
				  extrinsic number.
		*/
		self.system.inc_block_number();
		if block.header.block_number != self.system.block_number() {
			return Err("Block number does not match what is expected!");
		}
		for (i, support::Extrinsic { caller, call }) in block.extrinsics.into_iter().enumerate() {
			self.system.inc_nonce(&caller);
			let _res = self.dispatch(caller, call).map_err(|e| {
				eprintln!(
					"Extrinsic Error\n\tBlock Number: {}\n\tExtrinsic Number: {}\n\tError: {}",
					block.header.block_number, i, e
				);
			});
		}
		Ok(())
	}
}
impl crate::support::Dispatch for Runtime {
	type Caller = <Runtime as system::Config>::AccountID;
	type Call = RuntimeCall;
	// Dispatch a call on behalf of a caller. Increments the caller's nonce.
	//
	// Dispatch allows us to identify which underlying module call we want to execute.
	// Note that we extract the `caller` from the extrinsic, and use that information
	// to determine who we are executing the call on behalf of.
	fn dispatch(
		&mut self,
		caller: Self::Caller,
		runtime_call: Self::Call,
	) -> support::DispatchResult {
		unimplemented!();
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
