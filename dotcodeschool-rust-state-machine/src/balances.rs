use std::collections::BTreeMap;
#[derive(Debug)]
pub struct Pallet {
	balances: BTreeMap<String, u128>,
}

impl Pallet {
	pub fn new() -> Self {
		Self { balances: BTreeMap::new() }
	}
	pub fn set_balance(&mut self, who: &String, amount: u128) {
		self.balances.insert(who.clone(), amount);
	}
	pub fn balance(&self, who: &String) -> u128 {
		*self.balances.get(who).unwrap_or(&0)
	}
	pub fn transfer(
		&mut self,
		caller: String,
		to: String,
		amount: u128,
	) -> Result<(), &'static str> {
		// get balance of caller & to
		let caller_balance = self.balance(&caller);
		let to_balance = self.balance(&to);
		let new_caller_balance = caller_balance
			.checked_sub(amount)
			.ok_or("You do not have enough funds to do the transfer")?;
		let new_to_balance = to_balance.checked_add(amount).ok_or("An overflow occurred")?;
		self.set_balance(&caller, new_caller_balance);
		self.set_balance(&to, new_to_balance);
		Ok(())
	}
}

#[cfg(test)]
mod tests {
	#[test]
	fn init_balances() {
		let mut balances = super::Pallet::new();
		assert_eq!(balances.balance(&"alice".to_string()), 0);
		balances.set_balance(&"alice".to_string(), 100);
		assert_eq!(balances.balance(&"alice".to_string()), 100);
		assert_eq!(balances.balance(&"bob".to_string()), 0);
	}
	#[test]
	fn transfer_balance() {
		/* TODO: Create a test that checks the following:
			- That `alice` cannot transfer funds she does not have.
			- That `alice` can successfully transfer funds to `bob`.
			- That the balance of `alice` and `bob` is correctly updated.
		*/
		let mut balances = super::Pallet::new();
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
