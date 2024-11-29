# This is my own Blockchain project

## Feature 

### Primary Language 

[Rust](https://www.rust-lang.org/)

## Scalability Features

### Block Structure (support.rs):

*** A Block Structure that handles multiple extrinsics (tranactions) . ***

```Rust

pub struct Block<Header, Extrinsic>{
    pub header: Header,
    pub extrinsics: Vec<Extrinsic>
}

```

## Efficient Data Management (BTreeMap):

*** BTreeMap for storing claims and balance ensures that data is stored in a way that allows for efficient retrieval and modification. ***

```Rust

#[derive(Debug)]
pub struct Pallet<T: Config> {
    pub balances: BTreeMap<T::AccountId, T::Balance>,
}

```

### Module Structure

- mod balances
- mod main
- mod system
- mod proof_of_existence

*** Each module is responsible for a specific aspect of the blockchain, allowing for a clear separation of concerns and easier maintenance. ***

## Ensuring Data Integrity  and Immutability:

### Blockchain Immutable Ledger: 
*** The blockchain is an immutable ledger that stores all transactions in a sequential and tamper-proof manner . ***

## Block Structure (support.rs):

```Rust

pub struct Block<Header, Extrinsic>{
    pub header: Header,
    pub extrinsics: Vec<Extrinsic>
}

```

*** The block structure is designed to handle multiple extrinsics (transactions) and includes a header that contains metadata about the block . ***

## Proof of Existence (proof_of_existence):

```Rust
pub struct Pallet<T: Config> {
    claims: BTreeMap<T::Content, T::AccountId>,
}

```
*** The proof of existence pallet stores claims in a BTreeMap , allowing for efficient retrieval and modification of claims . ***

## System Module (System.rs):

```Rust


#[derive(Debug)]
pub struct Pallet<T: Config> {
    pub block_number: T::BlockNumber,
    nonce: BTreeMap<T::AccountId, T::Nonce>,
}

```
*** The system module stores the block number and nonce for each account in a BTreeMap , ensuring efficient retrieval and modification of system state . ***

## Transaction Validation:

```Rust
 pub fn create_claim(&mut self, caller:T::AccountId, claim:T::Content)-> DispatchResult{

        match self.get_claim(&claim) {
            Some(_)=> Err("Claim already exists"),
            None=>{
                self.claims.insert(claim, caller);

                Ok(())
            }
            
        }
        
    }

```

*** The create_claim function checks if an existing claim exists for the given content . If it does, it returns an error . If not , it inserts the claim into the BTreeMap and returns an Ok result . ***

##  Transfer Balance:

```Rust
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

```
*** The transfer function transfers a specified amount from one account to another . It first retrieves the balance of the caller and the recipient . Then it calculates the new balances for both accounts and updates them in the BTreeMap . If the transfer amount is invalid (i.e., negative), it returns an error . ***

##  Get Balance:

```Rust

pub fn get_balance(&self, who: &T::AccountId) -> T::Balance {
        *self.balances.get(who).unwrap_or(&T::Balance::zero())
    }

```
*** The get_balance function retrieves the balance of a given account . It uses the BTreeMap to look up the account and returns its balance . ***

##  Set Balance:

```Rust
 pub fn set_balance(&mut self, who: &T::AccountId, ammount: T::Balance) {
        self.balances.insert(who.clone(), ammount);
    }

```
*** The set_balance function updates the balance of a given account . It inserts the new balance into the BTreeMap . ***

## Increment Nonce:

```Rust
 pub fn inc_nonce(&mut self, who: &T::AccountId) {
        let nonce = *self.nonce.get(who).unwrap_or(&T::Nonce::zero());
        self.nonce
            .insert(who.clone(), nonce.checked_add(&T::Nonce::one()).unwrap());
    }

```

*** The inc_nonce function increments the nonce of a given account . It retrieves the current nonce from the BTreeMap , increments it by one, and updates the BTreeMap with the new nonce . ***

