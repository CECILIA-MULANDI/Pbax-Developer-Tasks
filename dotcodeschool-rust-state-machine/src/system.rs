use core::ops::AddAssign;
use num::traits::{One, Zero};
use std::{collections::BTreeMap, ops::Add};
// type AccountID = String;
// type BlockNumber = u32;
// type Nonce = u32;
/// This is the System Pallet.
/// It handles low level state needed for your blockchain.
#[derive(Debug)]
pub struct Pallet<AccountID, BlockNumber, Nonce> {
	block_number: BlockNumber,
	nonce: BTreeMap<AccountID, Nonce>,
}

impl<AccountID, BlockNumber, Nonce> Pallet<AccountID, BlockNumber, Nonce>
where
	AccountID: Ord + Clone,
	BlockNumber: Zero + One + AddAssign + Copy,
	Nonce: Zero + One + Copy,
{
	/// Create a new instance of the System Pallet.
	pub fn new() -> Self {
		/* TODO: Return a new instance of the `Pallet` struct. */
		Self { block_number: BlockNumber::zero(), nonce: BTreeMap::new() }
	}
	pub fn block_number(&self) -> BlockNumber {
		self.block_number
	}
	pub fn inc_block_number(&mut self) {
		self.block_number += BlockNumber::one();
	}
	pub fn inc_nonce(&mut self, who: &AccountID) {
		let nonce = *self.nonce.get(who).unwrap_or(&Nonce::zero());

		let new_nonce = nonce + Nonce::one();
		self.nonce.insert(who.clone(), new_nonce);
	}
}

#[cfg(test)]
mod tests {
	#[test]
	fn init_system() {
		let mut system = super::Pallet::<String, u32, u32>::new();
		system.inc_block_number();
		system.inc_nonce(&"alice".to_string());
		assert_eq!(system.block_number(), 1);
		assert_eq!(system.nonce.get(&"alice".to_string()).unwrap_or(&0), &1);
		assert_eq!(system.nonce.get(&"bob".to_string()).unwrap_or(&0), &0);
	}
}
