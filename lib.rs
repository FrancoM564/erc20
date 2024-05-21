#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod contract_publish {

    // use ink::env::call::{ExecutionInput, Selector};
    // use ink::env::debug_println;
    use ink::prelude::string::String;
    use ink::storage::Mapping;

    #[derive(scale::Decode, scale::Encode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        CallerIsOwner,
        CallerIsNotOwner,
        NotOnPossibleBuyersList,
        NotOnBuyersList,
        InsufficientBalance,
        AlreadyOnList,
        TransferError,
    }

    // #[derive(Debug)]
    #[derive(scale::Decode, scale::Encode)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    #[derive(Debug)]
    pub struct SongInfo {
        song_name: String,
        song_duration: String,
        artist_name: String,
        album: String,
        watermark_image_ipfs: String,
    }

    #[derive(scale::Decode, scale::Encode)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub struct ClientSongInfoResponse {
        song_info: SongInfo,
        price: Balance,
    }

    #[derive(scale::Decode, scale::Encode)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub struct DistributedStorageInfo {
        location: String,
        key: String,
    }

    #[derive(scale::Decode, scale::Encode)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub struct BuyerPublicKey {
        key: String,
    }

    /// Specify the ERC-20 result type.
    pub type ClientResult<T> = core::result::Result<T, Error>;

    /// Create storage for a simple ERC-20 contract.

    #[ink(event)]
    pub struct SongPublish {
        #[ink(topic)]
        owner: AccountId,
        #[ink(topic)]
        artist: String,
        #[ink(topic)]
        song_name: String,
        price: Balance,
    }

    #[ink(event)]
    pub struct SongBuyIntent {
        #[ink(topic)]
        from: AccountId,
        #[ink(topic)]
        owner: AccountId,
        #[ink(topic)]
        song_address: AccountId,
    }

    #[ink::event]
    pub struct SongBuyConfirmation {
        #[ink(topic)]
        buyer: AccountId,
        #[ink(topic)]
        author: String,
        #[ink(topic)]
        song_address: AccountId,
    }

    #[ink(storage)]
    pub struct ContractPublish {
        //Song info
        song_info: SongInfo,
        ///Owner address
        owner: AccountId,
        //Song price
        price: Balance,
        //Users that bought the song and were signed
        buyers: Mapping<AccountId, DistributedStorageInfo>,
        ///List of users with intention to buy
        possible_buyers_keys: Mapping<AccountId, BuyerPublicKey>,
    }

    impl ContractPublish {
        //------------------------------CONSTRUCTOR------------------------------

        /// Publica tu cancion almacenada en IPFS.
        #[ink(constructor)]
        pub fn publish_song(
            song_name: String,
            song_price: Balance,
            author_name: String,
            song_duration: String,
            album_name: String,
            image_address: String,
        ) -> Self {
            let owner = Self::env().caller();

            Self::env().emit_event(SongPublish {
                owner: owner.clone(),
                artist: author_name.clone(),
                price: song_price.clone(),
                song_name: song_name.clone(),
            });

            Self {
                song_info: SongInfo {
                    album: album_name,
                    artist_name: author_name,
                    song_duration,
                    song_name,
                    watermark_image_ipfs: image_address,
                },
                owner,
                price: song_price,
                buyers: Mapping::default(),
                possible_buyers_keys: Mapping::default(),
            }
        }

        //Messages
        //------------------------------GETTERS------------------------------
        #[ink(message)]
        pub fn get_song_info(&self) -> ClientSongInfoResponse {
            let return_value: SongInfo = SongInfo {
                song_name: self.song_info.song_name.clone(),
                song_duration: self.song_info.song_duration.clone(),
                artist_name: self.song_info.artist_name.clone(),
                album: self.song_info.album.clone(),
                watermark_image_ipfs: self.song_info.watermark_image_ipfs.clone(),
            };

            return ClientSongInfoResponse {
                song_info: return_value,
                price: self.price,
            };
        }

        #[ink(message, payable)]
        pub fn post_buy_intention(&mut self, buyer_public_key: String) -> ClientResult<String> {
            if Self::is_caller_owner(&self) {
                return Err(Error::CallerIsOwner);
            }

            if self.possible_buyers_keys.contains(&self.env().caller()) {
                return Err(Error::AlreadyOnList);
            }

            if self.env().transferred_value() < self.price {
                return Err(Error::InsufficientBalance);
            }

            self.possible_buyers_keys.insert(
                self.env().caller(),
                &BuyerPublicKey {
                    key: buyer_public_key,
                },
            );

            self.env().emit_event(SongBuyIntent {
                from: self.env().caller(),
                owner: self.owner,
                song_address: self.env().account_id(),
            });

            return Ok(String::from("Buy intention posted"));
        }

        #[ink(message)]
        pub fn get_buyer_public_key(&self, buyer_key: AccountId) -> ClientResult<String> {
            if !Self::is_caller_owner(&self) {
                return Err(Error::CallerIsNotOwner);
            }

            let posible_user_key = self.possible_buyers_keys.get(buyer_key);

            match posible_user_key {
                None => return Err(Error::NotOnPossibleBuyersList),
                Some(key) => return Ok(key.key),
            }
        }

        #[ink(message)]
        pub fn set_new_allowed_buyer(
            &mut self,
            encripted_symmetric_key: String,
            ipfs_song_address: String,
            buyer: AccountId,
        ) -> ClientResult<String> {

            if !self.possible_buyers_keys.contains(buyer) {
                return Err(Error::NotOnPossibleBuyersList)
            }

            if self.env().transfer(self.owner, self.price).is_err() {
                return Err(Error::TransferError);
            }

            self.possible_buyers_keys.remove(buyer);

            let new_saved_entry = DistributedStorageInfo {
                location: ipfs_song_address,
                key: encripted_symmetric_key,
            };

            self.buyers.insert(buyer, &new_saved_entry);

            self.env().emit_event(SongBuyConfirmation {
                buyer,
                author: self.song_info.artist_name.clone(),
                song_address: self.env().account_id(),
            });

            return Ok(String::from("Client added to buyers list"));
        }

        #[ink(message)]
        pub fn get_address_and_key_buyer(&self) -> ClientResult<DistributedStorageInfo> {

            if !self.buyers.contains(self.env().caller()) {
                return Err(Error::NotOnBuyersList)
            }

            let buyer_data = self.buyers.get(self.env().caller());

            match buyer_data {
                None => return Err(Error::NotOnBuyersList),
                Some(data) => return Ok(data)
            }
        }
        //------------------------------HELPERS------------------------------

        fn is_caller_owner(&self) -> bool {
            let caller = self.env().caller();
            return caller == self.owner;
        }
    }

    //------------------------------TESTS------------------------------

    #[cfg(test)]
    mod tests {
        use ink::primitives::AccountId;

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
        fn publish_works() {
            let contract = ContractPublish::new_publish(
                "La bebe - ringtone".to_string(),
                1,
                "QmZ41fazG24A6H4bicrM2cTPjLWxxsX8tQkrAPzCu2e8AB".to_string(),
                "QmZ2Fg6zDt8p7SLsuVAL2spGAAY2rPp7JShAY3Xk6Ndt8o".to_string(),
            );
            assert_eq!(
                contract.recover_hash_address(),
                "QmZ41fazG24A6H4bicrM2cTPjLWxxsX8tQkrAPzCu2e8AB"
            );
            assert_eq!(
                contract.recover_image_address(),
                "QmZ2Fg6zDt8p7SLsuVAL2spGAAY2rPp7JShAY3Xk6Ndt8o"
            );
            assert_eq!(contract.recover_song_name(), "La bebe - ringtone");
            assert_eq!(contract.recover_song_price(), 1);
        }
    }
}
