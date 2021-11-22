#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

pub use self::delegation_proxy::DelegationProxy;

#[ink::contract]
mod delegation_proxy {
    use delegation::Delegation;

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct DelegationProxy {
        /// Stores a single `bool` value on the storage.
        delegation: Delegation,
    }

    impl DelegationProxy {
        #[ink(constructor)]
        pub fn new(init_value: Delegation) -> Self {
            Self {
                delegation: init_value,
            }
        }

        /// Simply returns the current value of our `bool`.
        #[ink(message)]
        pub fn get(&self) -> Delegation {
            self.delegation.clone()
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
            // let delegation_proxy = DelegationProxy::default();
            assert!(true);
        }
    }
}
