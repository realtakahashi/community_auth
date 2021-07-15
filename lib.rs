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
    }

    #[derive(Encode, Decode, Debug, PartialEq, Eq, Copy, Clone)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        NotOwner,
        NotExists,
        AlreadyExists,
    }

    #[ink(event)]
    pub struct CreateComunity {
        #[ink(topic)]
        owner: AccountId,
        #[ink(topic)]
        name: String,
        #[ink(topic)]
        community_id: u128,
    }
    #[ink(event)]
    pub struct CreateCouncil {
        #[ink(topic)]
        owner: AccountId,
        #[ink(topic)]
        name: String,
        #[ink(topic)]
        community_id: u128,
        #[ink(topic)]
        council:Vec<AccountId>
    }

    impl CommunityAuth {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new() -> Self {
            Self { community:StorageHashMap::new()}
        }

        /// Constructor that initializes the `bool` value to `false`.
        ///
        /// Constructors can delegate to other constructors.
        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new()
        }

        #[ink(message)]
        pub fn create_community(&mut self, community_id:u128, name:String, address:String ) -> Result<(),Error> {
            let owner = self.env().caller();

            if self.get_community(community_id).is_some() {
                return Err(Error::AlreadyExists);
            } 

            let mut council = Vec::new();
            council.push(owner);
            let new_community = CommunityData {
                id: community_id,
                name: name.clone(),
                address: address,
                owner: owner,
                councils: council,
            };

            self.community.insert(community_id,new_community);
            self.env().emit_event(CreateComunity{owner:owner, name:name, community_id:community_id});
            Ok(())
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
            self.update_community(new_community.clone());
            self.env().emit_event(CreateCouncil{owner:transaction_caller, name:new_community.name.clone(), 
                community_id:new_community.id, council:new_community.councils });
            Ok(())
        }

        // todo 
        // create_proposal_for_create_new_councils
        // 新しいCouncilを作成するための提案を実施する
        // コミュニティメンバーの過半数の賛成で成立するように実装する
        // この提案は一定のブロックカウントの間でのみ有効であるように実装する

        // todo
        // vote_proposal_for_create_new_councils
        // 提出された提案に対して結果を投票する
        // 過半数以上が投票された段階で発動する
        // ブロックカウントチェックを行う。

        // todo
        // create_proposal_for_add_community_member
        // コミュニティメンバー追加を依頼するための提案を作成することが出来る
        // Councilの過半数の賛成で成立するように実装する
        // この提案は一定のブロックカウントの間でのみ有効であるように実装する


        // todo
        // vote_proposal_for_add_community_member
        // 提出された提案に対して結果を投票する
        // 過半数以上が投票された段階で発動する
        // ブロックカウントチェックを行う。

        // todo 
        // create_proposal_for_delete_community
        // コミュニティ削除の提案が出来る。
        // Councilだけがこの提案を実施出来る。
        // Councilの過半数の賛成で成立するように実装する
        // この提案は一定のブロックカウントの間でのみ有効であるように実装する

        // todo
        // vote_proposal_for_delete_community
        // 提出された提案に対して結果を投票する
        // 過半数以上が投票された段階で発動する
        // ブロックカウントチェックを行う。

        // todo
        // remove_myself_from_community
        // コミュニティから脱退する
        // 自分だけが呼ぶことが出来る

        // todo
        // 

        #[ink(message)]
        pub fn get_community(&self, community_id:u128) -> Option<CommunityData> {
            match self.community.get(&community_id) {
                Some(value)=> Some(value.clone()),
                None=> None,
            }
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
        use ink_env::{
            call,
            test,
        };
        /// Imports `ink_lang` so we can use `#[ink::test]`.
        use ink_lang as ink;

        #[ink::test]
        fn create_community_works(){
            let mut community_auth = CommunityAuth::new();
            assert_eq!(community_auth.create_community(1, "narusedai-jitikai".to_string(), 
                "narusedai-machida-tokyo-japan".to_string()),Ok(()));
            let test = community_auth.get_community(1).unwrap();
            assert_eq!(test.name, "narusedai-jitikai".to_string());
            assert_eq!(test.address, "narusedai-machida-tokyo-japan".to_string());
        }

        #[ink::test]
        fn fail_for_community_already_exists(){
            let mut community_auth = CommunityAuth::new();
            assert_eq!(community_auth.create_community(1, "narusedai-jitikai".to_string(), 
                "narusedai-machida-tokyo-japan".to_string()),Ok(()));
            let test = community_auth.get_community(1).unwrap();
            assert_eq!(test.name, "narusedai-jitikai".to_string());
            assert_eq!(test.address, "narusedai-machida-tokyo-japan".to_string());
            assert_eq!(community_auth.create_community(1, "narusedai-jitikai".to_string(), 
                "narusedai-machida-tokyo-japan".to_string()),Err(Error::AlreadyExists));
        }

        #[ink::test]
        fn create_council_for_community_works(){
            let accounts =
                ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
                    .expect("Cannot get accounts");
            let mut community_auth = CommunityAuth::new();
            assert_eq!(community_auth.create_community(1,"narusedai-jitikai".to_string(), 
                "narusedai-machida-tokyo-japan".to_string()),Ok(()));
            let test = community_auth.get_community(1).unwrap();
            assert_eq!(test.councils[0],accounts.alice);
            let mut  councils = Vec::new();
            councils.push(accounts.bob);
            councils.push(accounts.charlie);
            councils.push(accounts.django);
            assert_eq!(community_auth.create_council_for_community(1,councils),Ok(()));
            let test2 = community_auth.get_community(1).unwrap();
            assert_eq!(test2.councils.contains(&accounts.alice),true);
            assert_eq!(test2.councils.contains(&accounts.bob),true);
            assert_eq!(test2.councils.contains(&accounts.charlie),true);
            assert_eq!(test2.councils.contains(&accounts.django),true);
            assert_eq!(test2.name, "narusedai-jitikai".to_string());
            assert_eq!(test2.address, "narusedai-machida-tokyo-japan".to_string());
        }

        #[ink::test]
        fn fail_for_community_does_not_exists(){
            let accounts =
                ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
                    .expect("Cannot get accounts");
            let mut community_auth = CommunityAuth::new();
            assert_eq!(community_auth.create_community(1,"narusedai-jitikai".to_string(), 
                "narusedai-machida-tokyo-japan".to_string()),Ok(()));
            let test = community_auth.get_community(1).unwrap();
            assert_eq!(test.councils[0],accounts.alice);
            let mut  councils = Vec::new();
            councils.push(accounts.bob);
            councils.push(accounts.charlie);
            councils.push(accounts.django);
            assert_eq!(community_auth.create_council_for_community(2,councils),Err(Error::NotExists));
        }

        #[ink::test]
        fn fail_for_community_is_not_owner(){
            let accounts =
                ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
                    .expect("Cannot get accounts");
            let mut community_auth = CommunityAuth::new();
            assert_eq!(community_auth.create_community(1,"narusedai-jitikai".to_string(), 
                "narusedai-machida-tokyo-japan".to_string()),Ok(()));
            let test = community_auth.get_community(1).unwrap();
            assert_eq!(test.councils[0],accounts.alice);
            let mut  councils = Vec::new();
            councils.push(accounts.bob);
            councils.push(accounts.charlie);
            councils.push(accounts.django);
            set_sender(accounts.eve);
            assert_eq!(community_auth.create_council_for_community(1,councils),Err(Error::NotOwner));
        }

        fn set_sender(sender: AccountId) {
            let callee = ink_env::account_id::<ink_env::DefaultEnvironment>()
                .unwrap_or([0x0; 32].into());
            test::push_execution_context::<Environment>(
                sender,
                callee,
                1000000,
                1000000,
                test::CallData::new(call::Selector::new([0x00; 4])), // dummy
            );
        }
    }
}
