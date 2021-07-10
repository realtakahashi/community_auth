#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;


#[ink::contract]
mod community_auth {
    use ink_storage::{
        collections::{
            HashMap as StorageHashMap,
            hashmap::Entry,
        }
    };
    use ink_prelude::string::String;
    use ink_prelude::vec::Vec;
    use scale::{
        Decode,
        Encode,
    };

    #[derive(Debug, Clone, scale::Encode, scale::Decode,
        ink_storage_derive::PackedLayout, ink_storage_derive::SpreadLayout)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct CommunityData {
        id: u128,
        name: String,
        address: String,
        owner: AccountId,
        councils: Vec<AccountId>,
    }
    
    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct CommunityAuth {
        /// Stores a single `bool` value on the storage.
        // value: bool,
        community: StorageHashMap<u128, CommunityData>,
        id_of_community: u128,
    }

    #[derive(Encode, Decode, Debug, PartialEq, Eq, Copy, Clone)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        NotOwner,
        NotExists,
    }

    impl CommunityAuth {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new() -> Self {
            Self { community:StorageHashMap::new(), id_of_community:0 }
        }

        /// Constructor that initializes the `bool` value to `false`.
        ///
        /// Constructors can delegate to other constructors.
        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new()
        }

        #[ink(message)]
        pub fn create_community(&mut self, name:String, address:String ) {
            let owner = self.env().caller();
            self.id_of_community = self.id_of_community + 1;
            let mut council = Vec::new();
            council.push(owner);
            let new_community = CommunityData {
                id: self.id_of_community,
                name: name,
                address: address,
                owner: owner,
                councils: council,
            };

            self.community.insert(self.id_of_community,new_community);
        }

        #[ink(message)]
        pub fn create_council_for_community(&mut self, community_id:u128, council_members: Vec<AccountId>) -> Result<(), Error> {
            let transaction_caller = self.env().caller();
            let community = match self.community.get(&community_id) {
                Some(community) => community,
                None => return Err(Error::NotExists),
            };
            if community.owner != transaction_caller {
                return Err(Error::NotOwner);
            }
            // todo 投票が終わっているかチェック
            // todo 投票結果とメンバーが同一かをチェック
            let mut new_community = CommunityData {
                id: community.id,
                name: community.name.clone(),
                address: community.address.clone(),
                owner:community.owner,
                councils: council_members,
            };
            new_community.councils.push(transaction_caller);
            let result = self.delete_community(community_id);
            if result != Ok(()){
                return result;
            }
            self.update_community(new_community);
            Ok(())
        }

        

        // todo 
        // create_proposal_for_delete_community
        // コミュニティ削除の提案が出来る。
        // Councilの過半数の賛成で成立するように実装する
        // この提案は一定のブロックカウントの間でのみ有効であるように実装する

        #[ink(message)]
        pub fn get_community(&self, community_id:u128) -> Option<CommunityData> {
            match self.community.get(&community_id) {
                Some(value)=> Some(value.clone()),
                None=> None,
            }
        }

        #[ink(message)]
        pub fn get_latest_community_id(&self) -> u128 {
            self.id_of_community
        }

        fn delete_community(&mut self, id:u128) -> Result<(), Error> {
            let occupied = match self.community.entry(id) {
                Entry::Vacant(_) => return Err(Error::NotExists),
                Entry::Occupied(occupied) => occupied,
            };
            occupied.remove_entry();
            Ok(())
        }

        fn update_community(&mut self, community:CommunityData) {
            self.community.insert(community.id,community);
        }

    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// Imports `ink_lang` so we can use `#[ink::test]`.
        use ink_lang as ink;

        /// We test if the default constructor does its job.
        #[ink::test]
        fn default_works() {
            let community_auth = CommunityAuth::default();
            assert_eq!(community_auth.get_latest_community_id(), 0);
        }

        /// We test a simple use case of our contract.
        #[ink::test]
        fn it_works() {
            let community_auth = CommunityAuth::new();
            assert_eq!(community_auth.get_latest_community_id(), 0);
        }

        #[ink::test]
        fn create_community_works(){
            let mut community_auth = CommunityAuth::new();
            community_auth.create_community("narusedai-jitikai".to_string(), "narusedai-machida-tokyo-japan".to_string());
            let test = community_auth.get_community(1).unwrap();
            assert_eq!(test.name, "narusedai-jitikai".to_string());
            assert_eq!(test.address, "narusedai-machida-tokyo-japan".to_string());

        }

        #[ink::test]
        fn create_council_for_community_works(){
            let accounts =
                ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
                    .expect("Cannot get accounts");
            let mut community_auth = CommunityAuth::new();
            community_auth.create_community("narusedai-jitikai".to_string(), "narusedai-machida-tokyo-japan".to_string());
            let test = community_auth.get_community(1).unwrap();
            assert_eq!(test.councils[0],accounts.alice);
            let mut  councils = Vec::new();
            councils.push(accounts.bob);
            councils.push(accounts.charlie);
            councils.push(accounts.django);
            let _result = community_auth.create_council_for_community(1,councils);
            let test2 = community_auth.get_community(1).unwrap();
            assert_eq!(test2.councils.contains(&accounts.alice),true);
            assert_eq!(test2.councils.contains(&accounts.bob),true);
            assert_eq!(test2.councils.contains(&accounts.charlie),true);
            assert_eq!(test2.councils.contains(&accounts.django),true);
            assert_eq!(test2.name, "narusedai-jitikai".to_string());
            assert_eq!(test2.address, "narusedai-machida-tokyo-japan".to_string());
        }
    }
}
