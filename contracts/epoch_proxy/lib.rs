#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

pub use self::epoch_proxy::EpochProxy;

#[ink::contract]
mod epoch_proxy {
    use epoch::Epoch;
    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct EpochProxy {
        /// Stores a single `bool` value on the storage.
        epoch: Epoch,
    }

    impl EpochProxy {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(init_value: Epoch) -> Self {
            Self { epoch: init_value }
        }

        /// Simply returns the current value of our `bool`.
        #[ink(message)]
        pub fn get(&self) -> Epoch {
            self.epoch.clone()
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
            let epoch_proxy = EpochProxy::default();
            assert_eq!(true);
        }
    }
}
