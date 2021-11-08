#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

pub use self::subscription_proxy::SubscriptionProxy;

#[ink::contract]
mod subscription_proxy {
    use subscription::Subscription;
    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct SubscriptionProxy {
        /// Stores a single `Subscription` value on the storage.
        subscription: Subscription,
    }

    impl SubscriptionProxy {
        /// Constructor that initializes the `Subscription` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(init_value: Subscription) -> Self {
            Self { subscription: init_value }
        }

        /// Simply returns the current value of our subscription contract.
        #[ink(message)]
        pub fn get(&self) -> Subscription {
            self.subscription.clone()
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
            let subscription_proxy = SubscriptionProxy::default();
            assert_eq!(true);
        }
    }
}
