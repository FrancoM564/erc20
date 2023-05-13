#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod contract_publish {
    //use ink::storage::Mapping;
    use ink_prelude::string::String;

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        /// Return if the balance cannot fulfill a request.
        InsufficientBalance,
        InsufficientAllowance,
    }

    /// Specify the ERC-20 result type.
    pub type Result<T> = core::result::Result<T, Error>;

    /// Create storage for a simple ERC-20 contract.
    #[ink(storage)]
    pub struct contract_publish {
        ///Owner address
        owner: AccountId,

        /// Song name on String
        song_name: String,

        ///Song price
        song_value: u32,

        ///File hash address on IPFS
        file_address: String,

        ///List of allowed users
        authorized_users: Vec<AccountId>,

        ///Watermarked image address
        image_address: String,
    }

    #[ink(event)]
    pub struct Publish {
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        name: String,
        value: u32,
    }

    #[ink(event)]
    pub struct Transfer {
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        to: Option<AccountId>,
        value: Balance,
    }

    #[ink(event)]
    pub struct Approval {
        #[ink(topic)]
        owner: AccountId,
        #[ink(topic)]
        spender: AccountId,
        value: Balance,
    }

    impl contract_publish {

        //------------------------------CONSTRUCTOR------------------------------

        /// Create a new ERC-20 contract with an initial supply.
        #[ink(constructor)]
        pub fn new_publish(song_name: String, song_price: u32,file_address: String, image_address: String) -> Self {

            let owner = Self::env().caller();
            let authorized_users : Vec<AccountId> = vec![];

            Self::env().emit_event(Publish{
                from: Some(owner),
                name: song_name.clone(),
                value: song_price.clone(),
            });

            Self {
                owner,
                song_name,
                song_value: song_price,
                file_address,
                authorized_users,
                image_address,
            }
        }

        //Messages

        //------------------------------GETTERS------------------------------

        /*

        /// Returns the total token supply.
        #[ink(message)]
        pub fn total_supply(&self) -> Balance {
            self.total_supply
        }

        /// Returns the account balance for the specified `owner`.
        #[ink(message)]
        pub fn balance_of(&self, owner: AccountId) -> Balance {
            self.balances.get(owner).unwrap_or_default()
        }

        ///Returns allowance balance for the specified owner and spender
        #[ink(message)]
        pub fn allowance(&self, owner: AccountId, spender: AccountId) -> Balance {
            self.allowances.get((owner, spender)).unwrap_or_default()
        }
        */

        #[ink(message)]
        pub fn recover_song_name(&self) -> String{
            self.song_name.clone()
        }

        #[ink(message)]
        pub fn recover_hash_address(&self) -> String{
            self.file_address.clone()
        }

        #[ink(message)]
        pub fn recover_song_price(&self)-> u32{
            self.song_value.clone()
        }

        #[ink(message)]
        pub fn recover_image_address(&self) ->String{
            self.image_address.clone()
        }
        //------------------------------SETTERS------------------------------

        #[ink(message)]
        pub fn buy_song(&mut self, payment: Balance) -> Result<()>{ 
            let buyer = self.env().caller();

            if payment < self.recover_song_price().into() {
                return Err(Error::InsufficientAllowance)
            }

            Ok(())
        }


        /* 
        ///Transfer tokens to the specified account from caller
        [ink(message)]
        pub fn transfer(&mut self, to: AccountId, value: Balance) -> Result<()> {
            let from = self.env().caller();
            self.transfer_from_to(&from, &to, value)
        }
        
        ///Allow an spender acount to spend some tokens from caller
        #[ink(message)]
        pub fn approve(&mut self, spender: AccountId, value: Balance) -> Result<()> {
            let owner = self.env().caller();
            self.allowances.insert((owner, spender), &value);

            self.env().emit_event(Approval {
                owner,
                spender,
                value,
            });

            Ok(())
        }
        
        /// Transfers tokens on the behalf of the `from` account to the `to account
        #[ink(message)]
        pub fn transfer_from(
            &mut self,
            from: AccountId,
            to: AccountId,
            value: Balance,
        ) -> Result<()> {
            let caller = self.env().caller();
            let allowance = self.allowance(from, caller);
            if allowance < value {
                return Err(Error::InsufficientAllowance);
            }

            self.transfer_from_to(&from, &to, value)?;

            self.allowances.insert((from, caller), &(allowance - value));

            Ok(())
        }

        //------------------------------HELPERS------------------------------

        fn transfer_from_to(
            &mut self,
            from: &AccountId,
            to: &AccountId,
            value: Balance,
        ) -> Result<()> {
            let from_balance = self.balance_of(*from);
            if from_balance < value {
                return Err(Error::InsufficientBalance);
            }

            self.balances.insert(&from, &(from_balance - value));
            let to_balance = self.balance_of(*to);
            self.balances.insert(&to, &(to_balance + value));

            Self::env().emit_event(Transfer {
                from: Some(*from),
                to: Some(*to),
                value,
            });

            Ok(())
        }
        */
    }

    //------------------------------TESTS------------------------------

    #[cfg(test)]
    mod tests {
        use ink::{primitives::AccountId};

        use super::*;

        // We define some helper Accounts to make our tests more readable
        fn default_accounts() -> ink::env::test::DefaultAccounts<Environment> {
            ink::env::test::default_accounts::<Environment>()
        }

        fn alice() -> AccountId {
            default_accounts().alice
        }

        fn bob() -> AccountId {
            default_accounts().bob
        }

        fn charlie() -> AccountId {
            default_accounts().charlie
        }

        fn django() -> AccountId {
            default_accounts().django
        }

        #[ink::test]
        fn publish_works(){
            let contract = contract_publish::new_publish(
            "La bebe - ringtone".to_string(), 
            1, 
            "QmZ41fazG24A6H4bicrM2cTPjLWxxsX8tQkrAPzCu2e8AB".to_string(),
            "QmZ2Fg6zDt8p7SLsuVAL2spGAAY2rPp7JShAY3Xk6Ndt8o".to_string());
            assert_eq!(contract.recover_hash_address(),"QmZ41fazG24A6H4bicrM2cTPjLWxxsX8tQkrAPzCu2e8AB");
            assert_eq!(contract.recover_image_address(),"QmZ2Fg6zDt8p7SLsuVAL2spGAAY2rPp7JShAY3Xk6Ndt8o");
            assert_eq!(contract.recover_song_name(),"La bebe - ringtone");
            assert_eq!(contract.recover_song_price(),1);
        }

        #[ink::test]
        fn buy_song_works(){
            let contract = contract_publish::new_publish(
            "La bebe - ringtone".to_string(), 
            1, 
            "QmZ41fazG24A6H4bicrM2cTPjLWxxsX8tQkrAPzCu2e8AB".to_string(),
            "QmZ2Fg6zDt8p7SLsuVAL2spGAAY2rPp7JShAY3Xk6Ndt8o".to_string());
            assert_eq!(contract.recover_hash_address(),"QmZ41fazG24A6H4bicrM2cTPjLWxxsX8tQkrAPzCu2e8AB");
            assert_eq!(contract.recover_image_address(),"QmZ2Fg6zDt8p7SLsuVAL2spGAAY2rPp7JShAY3Xk6Ndt8o");
            assert_eq!(contract.recover_song_name(),"La bebe - ringtone");
            assert_eq!(contract.recover_song_price(),1);
        }





/*
        #[ink::test]
        fn return_hash_works(){
            let contract = Erc20::new(100,String::from("direccionIPFS"),String::from("direccionImagen"));
            assert_eq!(contract.recover_hash_address(),String::from("direccionIPFS"))

        }*/
    }
}
