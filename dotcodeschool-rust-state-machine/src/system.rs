/* TODO: You might need to update your imports. */
use std::collections::BTreeMap;
/// This is the System Pallet.
/// It handles low level state needed for your blockchain.
#[derive(Debug)]
pub struct Pallet {
	/// The current block number.
	/* TODO: Create a field `block_number` that stores a `u32`. */
	/// A map from an account to their nonce.
	/* TODO: Create a field `nonce` that is a `BTreeMap` from `String` to `u32`. */
	block_number: u32,
	nonce: BTreeMap<String, u32>,
}

impl Pallet {
	/// Create a new instance of the System Pallet.
	pub fn new() -> Self {
		/* TODO: Return a new instance of the `Pallet` struct. */
		Self { block_number: 0, nonce: BTreeMap::new() }
	}
	pub fn block_number(&self) -> u32 {
		self.block_number
	}
	pub fn inc_block_number(&mut self) {
		self.block_number += 1;
	}
	pub fn inc_nonce(&mut self, who: &String) {
		let current_nonce = self.nonce.entry(who.clone()).or_insert(0);
		*current_nonce += 1;
	}
}

#[cfg(test)]
mod tests {
	#[test]
	fn init_system() {
		/* TODO: Create a test which checks the following:
			- Increment the current block number.
			- Increment the nonce of `alice`.

			- Check the block number is what we expect.
			- Check the nonce of `alice` is what we expect.
			- Check the nonce of `bob` is what we expect.
		*/
		let mut system = super::Pallet::new();
		system.inc_block_number();
		system.inc_nonce(&"alice".to_string());
		assert_eq!(system.block_number(), 1);
		assert_eq!(system.nonce.get(&"alice".to_string()).unwrap_or(&0), &1);
		assert_eq!(system.nonce.get(&"bob".to_string()).unwrap_or(&0), &0);
	}
}
