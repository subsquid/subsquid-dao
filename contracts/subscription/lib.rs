#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

pub use self::subscription::Subscription;

#[ink::contract]
mod subscription {
    use epoch_proxy::EpochProxy;
    use ink_env;
    use ink_lang::EmitEvent;
    use ink_prelude::collections::BTreeMap;
    use ink_storage::collections::HashMap as StorageHashMap;
    use registry_proxy::RegistryProxy;

    #[cfg_attr(
        feature = "std",
        derive(::scale_info::TypeInfo, ::ink_storage::traits::StorageLayout,)
    )]
    #[derive(
        Default,
        Debug,
        PartialEq,
        Eq,
        Clone,
        scale::Encode,
        scale::Decode,
        ::ink_storage::traits::SpreadLayout,
        ::ink_storage::traits::PackedLayout,
    )]
    pub struct SubscriberData {
        pub balance: Balance,
        pub on: BlockNumber,
    }

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct Subscription {
        /// Stores a single `bool` value on the storage.
        subscription: StorageHashMap<Hash, BTreeMap<AccountId, SubscriberData>>,
        registry: RegistryProxy,
        epoch: EpochProxy,
    }

    #[ink(event)]
    pub struct Subscribe {
        #[ink(topic)]
        name: Hash,
        #[ink(topic)]
        from: AccountId,
        #[ink(topic)]
        value: Balance,
    }

    #[ink(event)]
    pub struct Unsubscribe {
        #[ink(topic)]
        name: Hash,
        #[ink(topic)]
        from: AccountId,
        #[ink(topic)]
        value: Balance,
    }

    #[ink(event)]
    pub struct Claimed {
        #[ink(topic)]
        name: Hash,
        #[ink(topic)]
        value: BTreeMap<AccountId, BlockNumber>,
        #[ink(topic)]
        total: Balance,
    }

    impl Subscription {
        /// Initializes the value to the initial value.
        #[ink(constructor)]
        pub fn new(init_value: RegistryProxy, epoch: EpochProxy) -> Self {
            Self {
                registry: init_value,
                epoch,
                subscription: Default::default(),
            }
        }

        pub fn get_indexer_rate(&self) -> Balance {
            self.registry.get().get_indexer_rate()
        }

        fn get_current_epoch(&self) -> u32 {
            self.epoch.get().get_current_epoch()
        }

        fn get_subscription_usage(&self, on: u32, curr: u32) -> Balance {
            self.get_indexer_rate() * (curr - on) as Balance
        }

        fn get_min_subscriber_period(&self) -> BlockNumber {
            BlockNumber::from(2u32)
        }

        fn not_expired(&self, hash_name: &Hash) -> Option<(AccountId, BlockNumber, u32)> {
            self.registry.get().not_expired(hash_name.clone())
        }

        fn is_owner(&self, hash_name: Hash) -> bool {
            self.registry
                .get()
                .is_owner_from(hash_name.clone(), self.env().caller().clone())
        }

        #[ink(message, payable, selector = 0xBABECAFE)]
        pub fn subscribe(&mut self, name: Hash, from: AccountId) {
            let payment = self.env().transferred_balance();
            ink_env::debug_println!("subscribe payment: {}", payment);
            assert!(
                self.env().transferred_balance() >= self.get_indexer_rate() * 30,
                "mininum subscription is 30 blocks"
            );
            // let on = self.env().block_number();
            let on = self.get_current_epoch();
            self.subscription
                .entry(name.clone())
                .and_modify(|old_value_map| {
                    old_value_map
                        .entry(from.clone())
                        .and_modify(|ov| {
                            let _b = (*ov).balance.saturating_add(payment.into());
                        })
                        .or_insert(SubscriberData {
                            balance: payment.into(),
                            on,
                        });
                })
                .or_insert({
                    let mut map = BTreeMap::new();
                    let _ = map.insert(
                        from,
                        SubscriberData {
                            balance: payment.into(),
                            on,
                        },
                    );
                    map
                });
            EmitEvent::<Self>::emit_event(
                self.env(),
                Subscribe {
                    name,
                    from,
                    value: payment.into(),
                },
            );
        }

        #[ink(message)]
        pub fn unsubscribe(&mut self, name: Hash) {
            let caller = self.env().caller();
            // let bn = self.env().block_number();
            let epoch = self.get_current_epoch();
            let min = self.get_min_subscriber_period();
            if let Some(a) = self.subscription.get(&name) {
                if let Some(d) = a.get(&caller) {
                    ink_env::debug_println!("epoch/min/on: {:?}/{:?}/{:?}", epoch, min, d.on);
                    if (epoch - d.on) < min {
                        ink_env::debug_println!("min threshold not met");
                        panic!("min threshold not met");
                    }
                } else {
                    ink_env::debug_println!("subscriber not found");
                    panic!("subscriber not found");
                }
            } else {
                ink_env::debug_println!("name not found");
                panic!("name not found");
            }
            let success = {
                if let Some(a) = self.subscription.get(&name) {
                    if let Some(d) = a.get(&caller) {
                        let value = (*d).balance;
                        let usage = self.get_subscription_usage((*d).on, epoch);
                        let ret = value - usage;
                        if ret > 0 {
                            match self.env().transfer(caller.clone(), ret) {
                                Err(ink_env::Error::BelowSubsistenceThreshold) => {
                                    panic!(
                                        "requested transfer would have brought contract\
                                        below subsistence threshold!"
                                    )
                                }
                                Err(_) => panic!("transfer failed!"),
                                Ok(_) => {
                                    if let Some((ref o, _b, _d)) = self.not_expired(&name) {
                                        match self.env().transfer(o.clone(), usage) {
                                            Err(ink_env::Error::BelowSubsistenceThreshold) => {
                                                panic!(
                                                    "requested transfer would have brought contract\
                                                    below subsistence threshold!"
                                                )
                                            }
                                            Err(_) => panic!("transfer failed!"),
                                            Ok(_) => Some(ret),
                                        }
                                    } else {
                                        ink_env::debug_println!("name not found");
                                        panic!("name not found");
                                    }
                                }
                            }
                        } else {
                            Some(0)
                        }
                    } else {
                        None
                    }
                } else {
                    None
                }
            };
            if let Some(value) = success {
                if let Some(a) = self.subscription.get_mut(&name) {
                    if let Some(_) = a.remove(&caller) {
                        EmitEvent::<Self>::emit_event(
                            self.env(),
                            Unsubscribe {
                                name,
                                from: caller.clone(),
                                value,
                            },
                        );
                    } else {
                        ink_env::debug_println!("failed to remove subscriber");
                        panic!("failed to remove subscriber");
                    }
                } else {
                    ink_env::debug_println!("name not found");
                    panic!("name not found");
                }
            } else {
                ink_env::debug_println!("transfer failed");
                panic!("transfer failed");
            }
        }

        #[ink(message)]
        pub fn get_subscription(&self, name: Hash, from: AccountId) -> Option<SubscriberData> {
            self.subscription
                .get(&name)
                .map(|data| data.get(&from))
                .flatten()
                .cloned()
        }

        #[ink(message)]
        pub fn claim_fees(&mut self, name: Hash) {
            if self.is_owner(name) {
                self.claim_fees_unchecked(name);
            } else {
                ink_env::debug_println!("not the owner");
                panic!("not the owner");
            }
        }

        pub fn claim_fees_unchecked(&mut self, name: Hash) {
            let bn = { self.get_current_epoch() };
            let claimed = if let Some(a) = self.subscription.get(&name) {
                let mut total = Balance::from(0u128);
                let mut value = BTreeMap::new();
                for (sub_id, sub_data) in a.iter() {
                    let usage = self.get_subscription_usage(sub_data.on, bn);
                    total = total.saturating_add(usage);
                    let t = bn - sub_data.on;
                    value.insert(sub_id.clone(), t);
                }
                Claimed { name, value, total }
            } else {
                ink_env::debug_println!("name not found");
                panic!("name not found");
            };
            if let Some(a) = self.subscription.get_mut(&name) {
                for (_, sub_data) in a.iter_mut() {
                    (*sub_data).on = bn;
                }
            } else {
                ink_env::debug_println!("name not found");
                panic!("name not found");
            }
            match self.env().transfer(self.env().caller(), claimed.total) {
                Err(ink_env::Error::BelowSubsistenceThreshold) => {
                    panic!(
                        "requested transfer would have brought contract\
                        below subsistence threshold!"
                    )
                }
                Err(_) => panic!("transfer failed!"),
                Ok(_) => (),
            }
            EmitEvent::<Self>::emit_event(self.env(), claimed);
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
            let subscription = Subscription::default();
            assert_eq!(true);
        }
    }
}
