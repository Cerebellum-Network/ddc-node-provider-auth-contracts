#![cfg_attr(not(feature = "std"), no_std)]
#![feature(proc_macro_hygiene)] // for tests in a separate file
#![deny(unused_must_use, unused_variables)]

use ink_lang as ink;

#[ink::contract]
mod node_provider_auth_white_list {
    use ink_prelude::vec::Vec;
    use ink_storage::Mapping;
    use scale::{Encode, Decode};
    use ink_storage::traits::SpreadAllocate;

    type NodePubKey = Vec<u8>;
    type NodeType = u8;

    #[ink(storage)]
    #[derive(SpreadAllocate, Default)]
    pub struct WhiteListAuthContract {
        pub admin: AccountId,
        pub list: Mapping<NodePubKey, bool>,
    }


    impl WhiteListAuthContract {

        #[ink(constructor)]
        pub fn new() -> Self {
            ink_lang::utils::initialize_contract(|contract: &mut Self| {
                let caller = Self::env().caller();
                contract.admin = caller;
            })
        }

        // This endpoint is triggered by the Cluster Pallet when a node provider joins a cluster
        #[ink(message, selector=0x96b0453e)]
        pub fn is_authorized(&self, _node_provider: AccountId, node_pub_key: NodePubKey, _node_variant: NodeType) -> bool {
            self.has_node_pub_key(node_pub_key)
        }

        #[ink(message)]
        pub fn add_node_pub_key(&mut self, node_pub_key: NodePubKey) -> Result<()> {
            let caller = Self::env().caller();
            if self.admin != caller {
                return Err(Error::OnlyAdmin)
            }

            self.list.insert(node_pub_key, &true);
            Ok(())
        }

        #[ink(message)]
        pub fn remove_node_pub_key(&mut self, node_pub_key: NodePubKey) -> Result<()> {
            let caller = Self::env().caller();
            if self.admin != caller {
                return Err(Error::OnlyAdmin)
            }

            self.list.remove(node_pub_key);
            Ok(())
        }

        #[ink(message)]
        pub fn has_node_pub_key(&self, node_pub_key: NodePubKey) -> bool {
            self.list.contains(&node_pub_key)
        }

        fn equal(&self, key1: &NodePubKey, key2: &NodePubKey) -> bool {
            if key1.len() != key2.len() {
                return false;
            }
            for (a, b) in key1.iter().zip(key2.iter()) {
                if a != b {
                    return false;
                }
            }
            true
        }
    }

    #[derive(Debug, PartialEq, Eq, Encode, Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        OnlyAdmin,
    }

    pub type Result<T> = core::result::Result<T, Error>;

    impl From<Error> for ink_env::Error {
        fn from(_: Error) -> Self {
            ink_env::Error::Unknown
        }
    }

}