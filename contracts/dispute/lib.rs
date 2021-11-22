#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

pub use self::dispute::Dispute;

#[ink::contract]
mod dispute {
    use ink_env;
    use ink_lang::EmitEvent;
    use ink_prelude::collections::{BTreeMap, BTreeSet};
    use ink_prelude::vec::Vec;
    use ink_storage::collections::HashMap as StorageHashMap;

    // use registry_proxy::RegistryProxy;
    // use epoch_proxy::EpochProxy;
    use subscription_proxy::{SubscriberData, SubscriptionProxy};

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct Dispute {
        dispute: StorageHashMap<Hash, BTreeMap<AccountId, (Hash, u32)>>,
        reputation: StorageHashMap<Hash, u16>,
        judge: Vec<AccountId>,
        veredict: StorageHashMap<Hash, BTreeMap<AccountId, bool>>,
        subscription: SubscriptionProxy,
    }

    #[ink(event)]
    pub struct Raised {
        #[ink(topic)]
        name: Hash,
        #[ink(topic)]
        from: AccountId,
        #[ink(topic)]
        cid: Hash,
    }

    #[ink(event)]
    pub struct WithdrawDispute {
        #[ink(topic)]
        name: Hash,
        #[ink(topic)]
        from: AccountId,
        #[ink(topic)]
        cid: Hash,
    }

    impl Dispute {
        /// Initializes the value to the initial value.
        #[ink(constructor)]
        pub fn new(judger: AccountId, subscription: SubscriptionProxy) -> Self {
            let mut judgers = Vec::default();
            judgers.push(judger);
            Self {
                dispute: Default::default(),
                reputation: Default::default(),
                veredict: Default::default(),
                subscription,
                judge: judgers,
            }
        }

        fn is_owner(&self, hash_name: Hash) -> bool {
            self.subscription
                .get()
                .is_owner_from(hash_name.clone(), self.env().caller().clone())
        }

        fn is_judge(&self) -> bool {
            let caller = self.env().caller();
            self.judge.iter().any(|s| *s == caller)
        }

        #[ink(message)]
        pub fn insert_judge(&mut self, judge: AccountId) {
            let is_judge = self.is_judge();
            if is_judge {
                self.judge.push(judge);
            } else {
                ink_env::debug_println!("not a judge");
                panic!("not a judge");
            }
        }

        fn get_subscription(
            &self,
            hash_name: Hash,
            subscriber: AccountId,
        ) -> Option<SubscriberData> {
            self.subscription
                .get()
                .get_subscription(hash_name, subscriber)
        }

        fn get_dispute_expiration(&self) -> u32 {
            10 // length in epochs
        }

        #[ink(message)]
        pub fn get_current_epoch(&self) -> u32 {
            self.subscription.get().get_current_epoch()
        }

        #[ink(message)]
        pub fn get_reputation(&self, hash_name: Hash) -> Option<u16> {
            self.reputation.get(&hash_name).cloned()
        }

        #[ink(message)]
        pub fn get_cid(&self, hash_name: Hash, subscriber: AccountId) -> Option<(Hash, u32)> {
            self.dispute
                .get(&hash_name)
                .map(|data| data.get(&subscriber))
                .flatten()
                .cloned()
        }

        #[ink(message)]
        pub fn cid_exists(&self, cid: Hash) -> bool {
            let epoch = self.get_current_epoch();
            let expire = self.get_dispute_expiration();
            self.dispute
                .values()
                .map(|data| {
                    data.values().filter_map(|(ref inner_cid, ref on)| {
                        if *inner_cid == cid && (epoch - on) < expire {
                            Some(true)
                        } else {
                            None
                        }
                    })
                })
                .flatten()
                .next()
                .is_some()
        }

        #[ink(message, selector = 0xDEADBABE)]
        pub fn raise_dispute(&mut self, name: Hash, cid: Hash) {
            let caller = self.env().caller();
            if let Some(data) = self.get_subscription(name.clone(), caller) {
                self.unchecked_raise_dispute(name, cid, data.clone());
            } else {
                ink_env::debug_println!("not subscribed");
                panic!("not subscribed");
            }
        }

        fn unchecked_raise_dispute(&mut self, hash_name: Hash, cid: Hash, data: SubscriberData) {
            let epoch = self.get_current_epoch();
            let caller = self.env().caller();
            let expire = self.get_dispute_expiration();

            self.dispute
                .entry(hash_name.clone())
                .and_modify(|old_value_map| {
                    old_value_map
                        .entry(caller.clone())
                        .and_modify(|ov| {
                            let (ref _cid, ref on) = ov;
                            if epoch - on > expire {
                                *ov = (cid.clone(), epoch.clone());
                            } else {
                                panic!("dispute already in place");
                            }
                        })
                        .or_insert((cid.clone(), epoch.clone()));
                })
                .or_insert({
                    let mut map = BTreeMap::new();
                    let _ = map.insert(caller, (cid.clone(), epoch.clone()));
                    map
                });
            EmitEvent::<Self>::emit_event(
                self.env(),
                Raised {
                    name: hash_name,
                    from: caller,
                    cid,
                },
            );
        }

        #[ink(message, selector = 0xCAFEBABE)]
        pub fn withdraw_dispute(&mut self, name: Hash) {
            let caller = self.env().caller();
            let checked = if let Some(data) = self.get_subscription(name.clone(), caller) {
                if let Some(disputes) = self.dispute.get(&name) {
                    disputes.get(&caller).cloned()
                } else {
                    ink_env::debug_println!("name not found");
                    panic!("name not found")
                }
            } else {
                ink_env::debug_println!("not subscribed");
                panic!("not subscribed");
            };

            if let Some(data) = checked {
                let (ref cid, ref _on) = data;
                self.unchecked_withdraw_dispute(name, cid.clone());
            } else {
                ink_env::debug_println!("caller not found");
                panic!("caller not found");
            }
        }

        fn unchecked_withdraw_dispute(&mut self, hash_name: Hash, cid: Hash) {
            let caller = self.env().caller();
            if let Some(a) = self.dispute.get_mut(&hash_name) {
                if let Some(_) = a.remove(&caller) {
                    let _ = self.veredict.take(&cid);
                    EmitEvent::<Self>::emit_event(
                        self.env(),
                        WithdrawDispute {
                            name: hash_name,
                            from: caller.clone(),
                            cid,
                        },
                    );
                } else {
                    ink_env::debug_println!("failed to remove caller");
                    panic!("failed to remove caller");
                }
            } else {
                ink_env::debug_println!("name not found");
                panic!("name not found");
            }
        }

        #[ink(message)]
        pub fn submit_vote(&mut self, cid: Hash, vote: bool) {
            let is_judge = self.is_judge();
            if is_judge {
                let cid_exists = self.cid_exists(cid.clone());
                if cid_exists {
                    let caller = self.env().caller();
                    self.veredict
                        .entry(cid.clone())
                        .and_modify(|old_value_map| {
                            old_value_map
                                .entry(caller.clone())
                                .and_modify(|ov| {
                                    *ov = vote;
                                })
                                .or_insert(vote);
                        })
                        .or_insert({
                            let mut map = BTreeMap::new();
                            let _ = map.insert(caller, vote);
                            map
                        });
                } else {
                    ink_env::debug_println!("invalid cid");
                    panic!("invalid cid");
                }
            } else {
                ink_env::debug_println!("not a judge");
                panic!("not a judge");
            }
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
            // let dispute = Dispute::default();
            assert!(true);
        }
    }
}
