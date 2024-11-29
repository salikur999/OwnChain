use core::fmt::Debug;
use std::collections::BTreeMap;

use crate::support::DispatchResult;

pub trait Config: crate::system::Config {
    type Content: Debug + Ord;
}

#[derive(Debug)]
pub struct Pallet<T: Config> {
    claims: BTreeMap<T::Content, T::AccountId>,
}

#[macros::call]
impl <T: Config>  Pallet<T>{
    pub fn create_claim(&mut self, caller:T::AccountId, claim:T::Content)-> DispatchResult{

        match self.get_claim(&claim) {
            Some(_)=> Err("Claim already exists"),
            None=>{
                self.claims.insert(claim, caller);

                Ok(())
            }
            
        }
        
    }

    pub fn revoke_claim(&mut self, caller:T::AccountId, claim:T::Content)-> DispatchResult{
        let claim_owner = self.get_claim(&claim).ok_or("Claim does not exist")?;
        if claim_owner != &caller{
            return Err("Caller is not the owner of the claim");
        }
        self.claims.remove(&claim);
        Ok(())

    }
}

impl<T: Config> Pallet<T> {
    pub fn new() -> Self {
        Self {
            claims: BTreeMap::new(),
        }
    }

    pub fn get_claim(&self, claim: &T::Content)->Option<&T::AccountId>{

        self.claims.get(claim)
       
    }
   
}


#[cfg(test)]
mod tests {

    use super::*;
  pub struct TestConfig;

    impl Config for TestConfig{
        type Content = &'static str;
    } 
    impl crate::system::Config for TestConfig {
        type AccountId = &'static str;
        type BlockNumber = u64;
        type Nonce = u64;
    }
    #[test]
    fn basic_proof_of_existence(){
        let mut poe = super::Pallet::<TestConfig>::new();
        let _= poe.create_claim("salikur", "my_document");

        assert_eq!(poe.get_claim(&"my_document"), Some(&"salikur"));

        let res = poe.revoke_claim(&"sabrina", "my_document");
        assert_eq!(res, Err("Caller is not the owner of the claim"));
    }
}
