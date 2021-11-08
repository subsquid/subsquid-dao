#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

pub use self::indexer_meta::IndexerMeta;

#[ink::contract]
mod indexer_meta {
    use ink_lang::EmitEvent;
    use ink_prelude::collections::BTreeMap;
    use ink_prelude::string::String;
    use ink_storage::collections::HashMap as StorageHashMap;
    use registry_proxy::RegistryProxy;

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct IndexerMeta {
        link: StorageHashMap<Hash, String>,
        capabilities: StorageHashMap<Hash, BTreeMap<Hash, String>>,
        registry: RegistryProxy,
    }

    /// Emitted whenever a new Link is being registered.
    #[ink(event)]
    pub struct Link {
        #[ink(topic)]
        name: Hash,
        #[ink(topic)]
        link: String,
    }

    /// Emitted whenever a new Link is being registered.
    #[ink(event)]
    pub struct Capabilities {
        #[ink(topic)]
        name: Hash,
        #[ink(topic)]
        property: Hash,
        #[ink(topic)]
        value: String,
    }

    impl IndexerMeta {
        #[ink(constructor)]
        pub fn new(init_value: RegistryProxy) -> Self {
            Self {
                registry: init_value,
                link: Default::default(),
                capabilities: Default::default(),
            }
        }

        #[ink(message, selector = 0xCAFEDEAD)]
        pub fn set_link(&mut self, name: Hash, link: String) {
            if self.is_owner(name) {
                self.set_link_unchecked(name, link);
            } else {
                ink_env::debug_println!("not the owner");
                panic!("not the owner");
            }
        }

        fn is_owner(&self, hash_name: Hash) -> bool {
            self.registry
                .get()
                .is_owner_from(hash_name.clone(), self.env().caller().clone())
        }

        fn set_link_unchecked(&mut self, name_hash: Hash, link: String) {
            ink_env::debug_println!("link name_hash: {:?}", name_hash);
            // if let Some((_a, _b, _d)) = self.expired(name_hash) {
            //     self.unregister_unchecked(name_hash);
            // } else {
            self.link
                .entry(name_hash)
                .and_modify(|old_value| *old_value = link.clone())
                .or_insert(link.clone());

            EmitEvent::<Self>::emit_event(
                self.env(),
                Link {
                    name: name_hash,
                    link,
                },
            );
            // }
        }

        #[ink(message, selector = 0xCAFE)]
        pub fn set_capability(&mut self, name: Hash, property: Hash, value: String) {
            if self.is_owner(name) {
                self.set_capability_unchecked(name, property, value);
            } else {
                ink_env::debug_println!("not the owner");
                panic!("not the owner");
            }
        }

        fn set_capability_unchecked(&mut self, name_hash: Hash, property: Hash, value: String) {
            ink_env::debug_println!("capability name_hash: {:?}", name_hash);
            // if let Some((_a, _b, _d)) = self.expired(name_hash) {
            //     self.unregister_unchecked(name_hash);
            // } else {
            self.capabilities
                .entry(name_hash)
                .and_modify(|old_value| {
                    old_value.insert(property.clone(), value.clone());
                })
                .or_insert({
                    let mut map = BTreeMap::default();
                    let _ = map.insert(property.clone(), value.clone());
                    map
                });

            EmitEvent::<Self>::emit_event(
                self.env(),
                Capabilities {
                    name: name_hash,
                    property,
                    value,
                },
            );
        }

        #[ink(message)]
        pub fn get_capabilities(&self, name: Hash) -> BTreeMap<Hash, String> {
            let cap = self.capabilities.get(&name).cloned();
            if let Some(c) = cap {
                c
            } else {
                BTreeMap::default()
            }
        }

        #[ink(message)]
        pub fn get_link(&self, name: Hash) -> Option<String> {
            self.link.get(&name).cloned()
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
            let indexer_meta = IndexerMeta::default();
            assert_eq!(true);
        }
    }
}
