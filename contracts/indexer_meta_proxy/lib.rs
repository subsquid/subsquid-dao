#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

pub use self::indexer_meta_proxy::IndexerMetaProxy;

#[ink::contract]
mod indexer_meta_proxy {
    use indexer_meta::IndexerMeta;
    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct IndexerMetaProxy {
        /// Stores a single `bool` value on the storage.
        indexer: IndexerMeta,
    }

    impl IndexerMetaProxy {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(init_value: IndexerMeta) -> Self {
            Self {
                indexer: init_value,
            }
        }

        /// Simply returns the current value of our `bool`.
        #[ink(message)]
        pub fn get(&self) -> IndexerMeta {
            self.indexer.clone()
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
            let indexer_meta_proxy = IndexerMetaProxy::default();
            assert_eq!(true);
        }
    }
}
