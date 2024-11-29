use num::traits::{CheckedAdd, CheckedSub, One, Zero};
use std::{collections::BTreeMap, ops::AddAssign};

pub trait Config {
    type AccountId: Ord + Clone;
    type BlockNumber: Clone + One + Copy + CheckedAdd + CheckedSub + Zero + AddAssign;
    type Nonce: Clone + Copy + CheckedAdd + CheckedSub + Zero + One;
}

#[derive(Debug)]
pub struct Pallet<T: Config> {
    pub block_number: T::BlockNumber,
    // Nonce ->  AccountId is a User and Nonce is a How many transaction this user
    nonce: BTreeMap<T::AccountId, T::Nonce>,
}


impl<T: Config> Pallet<T> {
    pub fn new() -> Self {
        Self {
            block_number: T::BlockNumber::zero(),
            nonce: BTreeMap::new(),
        }
    }

    pub fn get_block_number(&self) -> T::BlockNumber {
        self.block_number
    }

    pub fn block_number(&self) -> T::BlockNumber {
        self.get_block_number()
    }

    pub fn inc_block_number(&mut self) { // Fixed typo
        self.block_number = self
            .block_number
            .checked_add(&T::BlockNumber::one())
            .unwrap();
    }
    pub fn inc_nonce(&mut self, who: &T::AccountId) {
        let nonce = *self.nonce.get(who).unwrap_or(&T::Nonce::zero());
        self.nonce
            .insert(who.clone(), nonce.checked_add(&T::Nonce::one()).unwrap());
    }

    pub fn get_nonce(&self, who: &T::AccountId) -> T::Nonce {
        *self.nonce.get(who).unwrap_or(&T::Nonce::zero())
    }
}




#[cfg(test)]
mod test {
    struct TestConfig;
    impl super::Config for TestConfig {
        type AccountId = String;
        type BlockNumber = u64;
        type Nonce = u64;
    }
    #[test]
    fn init_system() {
        let  system: super::Pallet<TestConfig> = super::Pallet::new();
        assert_eq!(system.block_number, 0);
    }
    #[test]
    fn increment_block() {
        let mut system: super::Pallet<TestConfig> = super::Pallet::new();
        system.inc_block_number(); 
        assert_eq!(system.block_number, 1);
    }

    #[test]
    fn increment_nonce() {
        let mut system: super::Pallet<TestConfig> = super::Pallet::new();
        system.inc_nonce(&"salikur".to_string());
        assert_eq!(system.get_nonce(&"salikur".to_string()), 1);
    }
}
