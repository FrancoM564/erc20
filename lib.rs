#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod contract_publish {

    use ink::storage::Mapping;
    use ink::prelude::string::String;
    use ink::prelude::vec::Vec;

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        /// Return if the balance cannot fulfill a request.
        InsufficientBalance,
        AlreadyOnList,
        TransferError,
    }

    /// Specify the ERC-20 result type.
    pub type Result<T> = core::result::Result<T, Error>;

    /// Create storage for a simple ERC-20 contract.
    #[ink(storage)]
    pub struct ContractPublish {

        ///Contract balance
        balance: Balance,

        ///Owner address
        owner: AccountId,

        /// Song name on String
        song_name: String,

        ///Song price
        song_value: Balance,

        ///File hash address on IPFS
        file_address: String,

        ///List of allowed users
        authorized_users: Mapping<AccountId,bool>,

        ///Watermarked image address
        image_address: String,
    }

    #[ink(event)]
    pub struct Publish {
        #[ink(topic)]
        from: AccountId,
        #[ink(topic)]
        name: String,
        value: Balance,
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
    pub struct Buy {
        #[ink(topic)]
        from: AccountId,
        #[ink(topic)]
        of: String,
    }

    impl ContractPublish {

        //------------------------------CONSTRUCTOR------------------------------

        /// Publica tu cancion almacenada en IPFS.
        #[ink(constructor)]
        pub fn new_publish(song_name: String, song_price: Balance,file_address: String, image_address: String) -> Self {

            let owner = Self::env().caller();
            let authorized_users = Mapping::default();
            let balance = Balance::default();

            Self::env().emit_event(Publish{
                from: owner,
                name: song_name.clone(),
                value: song_price,
            });

            Self {
                balance,
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
        #[ink(message)]
        pub fn recover_song_name(&self) -> String{
            self.song_name.clone()
        }

        #[ink(message)]
        pub fn recover_song_price(&self)-> u128{
            self.song_value
        }

        #[ink(message)]
        pub fn recover_image_address(&self) ->String{
            self.image_address.clone()
        }
        
        //------------------------------SETTERS------------------------------

        #[ink(message, payable)]
        pub fn buy_song(&mut self) -> Result<(String,Balance)>{ 

            let caller = self.env().caller();

            assert!(caller != self.owner, "The caller is the owner, it doesn't make sense");

            if self.env().transferred_value() < self.recover_song_price() {
                return Err(Error::InsufficientBalance)
            }

            if self.authorized_users.contains(&self.env().caller()){
                return Err(Error::AlreadyOnList)
            }

            if self.env().transfer(self.owner, self.song_value).is_err(){
                panic!(
                    "For some reason the transaction couldn't be completed"
                )
            }

            self.authorized_users.insert(caller,&true);

            Self::env().emit_event(Buy{
                from: caller,
                of: self.song_name.clone(),
            });

            Ok((self.song_name.clone(),self.env().balance()))

        }

        #[ink(message)]
        pub fn recover_hash_address(&self) -> String{
            
            let caller = self.env().caller();

            if caller == self.owner{
                return self.file_address.clone()
            }

            if self.authorized_users.contains(caller){

                if self.authorized_users.get(caller).unwrap_or(false){
                    return self.file_address.clone()
                }else{
                    return String::from("No tienes permiso")
                }
            }

            return String::from("No has comprado este archivo")
        }

        //------------------------------HELPERS------------------------------
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
            let contract = ContractPublish::new_publish(
            "La bebe - ringtone".to_string(), 
            1, 
            "QmZ41fazG24A6H4bicrM2cTPjLWxxsX8tQkrAPzCu2e8AB".to_string(),
            "QmZ2Fg6zDt8p7SLsuVAL2spGAAY2rPp7JShAY3Xk6Ndt8o".to_string());
            assert_eq!(contract.recover_hash_address(),"QmZ41fazG24A6H4bicrM2cTPjLWxxsX8tQkrAPzCu2e8AB");
            assert_eq!(contract.recover_image_address(),"QmZ2Fg6zDt8p7SLsuVAL2spGAAY2rPp7JShAY3Xk6Ndt8o");
            assert_eq!(contract.recover_song_name(),"La bebe - ringtone");
            assert_eq!(contract.recover_song_price(),1);
        }
    }
}
