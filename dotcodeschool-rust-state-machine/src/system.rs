use core::ops::AddAssign;
use num::traits::{One, Zero};
use std::collections::BTreeMap;
pub trait Config {
	type AccountID: Ord + Clone;
	type BlockNumber: Zero + One + AddAssign + Copy;
	type Nonce: Zero + One + Copy;
}
#[derive(Debug)]
pub struct Pallet<T: Config> {
	block_number: T::BlockNumber,
	nonce: BTreeMap<T::AccountID, T::Nonce>,
}

impl<T: Config> Pallet<T> {
	/// Create a new instance of the System Pallet.
	pub fn new() -> Self {
		/* TODO: Return a new instance of the `Pallet` struct. */
		Self { block_number: T::BlockNumber::zero(), nonce: BTreeMap::new() }
	}
	pub fn block_number(&self) -> T::BlockNumber {
		self.block_number
	}
	pub fn inc_block_number(&mut self) {
		self.block_number += T::BlockNumber::one();
	}
	pub fn inc_nonce(&mut self, who: &T::AccountID) {
		let nonce = *self.nonce.get(who).unwrap_or(&T::Nonce::zero());

		let new_nonce = nonce + T::Nonce::one();
		self.nonce.insert(who.clone(), new_nonce);
	}
}

#[cfg(test)]
mod tests {
	#[test]
	fn init_system() {
		struct TestConfig;
		impl super::Config for TestConfig {
			type AccountID = String;
			type BlockNumber = u32;
			type Nonce = u32;
		}
		let mut system = super::Pallet::<TestConfig>::new();
		system.inc_block_number();
		system.inc_nonce(&"alice".to_string());
		assert_eq!(system.block_number(), 1);
		assert_eq!(system.nonce.get(&"alice".to_string()).unwrap_or(&0), &1);
		assert_eq!(system.nonce.get(&"bob".to_string()).unwrap_or(&0), &0);
	}
}
