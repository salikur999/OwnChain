use num::traits::{CheckedAdd, CheckedSub, Zero};
use std::collections::BTreeMap;

//trait working other languages is interface , c#, java
//trait scope something that implements is must be implemented
// Config is inherited of system::Config 
pub trait Config: crate::system::Config {
    type Balance: Clone + Copy + CheckedAdd + CheckedSub + Zero;
}

#[derive(Debug)]
//struct is a data type 
//other language is object and class
pub struct Pallet<T: Config> {
    // AccountId is a user and Balance is  a user of the account
    pub balances: BTreeMap<T::AccountId, T::Balance>,
}



#[macros::call]
impl <T: Config> Pallet<T> {
    pub fn transfer(
        &mut self,
        caller: T::AccountId,
        to: T::AccountId,
        ammount: T::Balance,
    ) -> Result<(), &'static str> {
        let caller_balance = self.get_balance(&caller);
        let to_balance = self.get_balance(&to);

        let new_caller_balance = caller_balance
            .checked_sub(&ammount)
            .ok_or("Invalid ammount")?;
        let new_to_balance = to_balance.checked_add(&ammount).ok_or("Invalid ammount")?;

        self.set_balance(&caller, new_caller_balance);
        self.set_balance(&to, new_to_balance);

        Ok(())
    }
}
impl<T: Config> Pallet<T> {
    pub fn new() -> Self {
        Self {
            balances: BTreeMap::new(),
        }
    }

    pub fn set_balance(&mut self, who: &T::AccountId, ammount: T::Balance) {
        self.balances.insert(who.clone(), ammount);
    }

    pub fn get_balance(&self, who: &T::AccountId) -> T::Balance {
        *self.balances.get(who).unwrap_or(&T::Balance::zero())
    }

    
}

#[cfg(test)]
mod tests {
    use crate::system;

    struct TestConfig;
    impl system::Config for TestConfig {
        type AccountId = String;
        type BlockNumber = u64;
        type Nonce = u64;
    }
    // super is using cll prante
    impl super::Config for TestConfig {
        type Balance = u128;
    }

    #[test]
    fn init_balance() {
        let mut pallet: super::Pallet<TestConfig> = super::Pallet::new();
        assert_eq!(pallet.get_balance(&"salikur".to_string()), 0);
        pallet.set_balance(&"salikur".to_string(), 100);
        assert_eq!(pallet.get_balance(&"salikur".to_string()), 100);
    }

    #[test]
    fn transfer_balance() {
        let mut pallet: super::Pallet<TestConfig> = super::Pallet::new();

        pallet.set_balance(&"salikur".to_string(), 100);
        let _ = pallet.transfer(
            "salikur".to_string().clone(),
            "sabrina".to_string().clone(),
            50,
        );
        assert_eq!(pallet.get_balance(&"salikur".to_string()), 50);
        assert_eq!(pallet.get_balance(&"sabrina".to_string()), 50);
    }

    #[test]
    fn transfer_invlaid() {
        let mut pallet: super::Pallet<TestConfig> = super::Pallet::new();
        assert_eq!(pallet.get_balance(&"salikur".to_string()), 0);
        pallet.set_balance(&"salikur".to_string(), 100);
        let _ = pallet.transfer(
            "salikur".to_string().clone(),
            "sabrina".to_string().clone(),
            110,
        );
        assert_eq!(pallet.get_balance(&"sabrina".to_string()), 0);
    }
}
