use num::traits::{CheckedAdd, CheckedSub, Zero};
use std::collections::BTreeMap;

// type AccountID = String;
// type Balance = u128;
pub trait Config {
	type AccountID: Ord + Clone;
	type Balance: Zero + CheckedAdd + CheckedSub + Copy;
}
#[derive(Debug)]
pub struct Pallet<T: Config> {
	balances: BTreeMap<T::AccountID, T::Balance>,
}

impl<T: Config> Pallet<T> {
	pub fn new() -> Self {
		Self { balances: BTreeMap::new() }
	}
	pub fn set_balance(&mut self, who: &T::AccountID, amount: T::Balance) {
		self.balances.insert(who.clone(), amount);
	}

	pub fn balance(&self, who: &T::AccountID) -> T::Balance {
		*self.balances.get(who).unwrap_or(&T::Balance::zero())
	}
	pub fn transfer(
		&mut self,
		caller: T::AccountID,
		to: T::AccountID,
		amount: T::Balance,
	) -> Result<(), &'static str> {
		// get balance of caller & to
		let caller_balance = self.balance(&caller);
		let to_balance = self.balance(&to);
		let new_caller_balance = caller_balance
			.checked_sub(&amount)
			.ok_or("You do not have enough funds to do the transfer")?;
		let new_to_balance = to_balance.checked_add(&amount).ok_or("An overflow occurred")?;
		self.set_balance(&caller, new_caller_balance);
		self.set_balance(&to, new_to_balance);
		Ok(())
	}
}

#[cfg(test)]
mod tests {
	struct TestConfig;
	impl super::Config for TestConfig {
		type AccountID = String;
		type Balance = u128;
	}
	#[test]
	fn init_balances() {
		let mut balances = super::Pallet::<TestConfig>::new();
		assert_eq!(balances.balance(&"alice".to_string()), 0);
		balances.set_balance(&"alice".to_string(), 100);
		assert_eq!(balances.balance(&"alice".to_string()), 100);
		assert_eq!(balances.balance(&"bob".to_string()), 0);
	}
	#[test]
	fn transfer_balance() {
		let mut balances = super::Pallet::<TestConfig>::new();
		balances.set_balance(&"alice".to_string(), 100);
		balances.set_balance(&"bob".to_string(), 50);

		assert_eq!(
			balances.transfer("alice".to_string(), "bob".to_string(), 200),
			Err("You do not have enough funds to do the transfer")
		);
		assert_eq!(balances.transfer("alice".to_string(), "bob".to_string(), 30), Ok(()));
		assert_eq!(balances.transfer("bob".to_string(), "alice".to_string(), 10), Ok(()));
		assert_eq!(balances.balance(&"alice".to_string()), 80);
		assert_eq!(balances.balance(&"bob".to_string()), 70)
	}
}
