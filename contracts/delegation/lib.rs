#![cfg_attr(not(feature = "std"), no_std)]

pub use self::delegation::Delegation;

use ink_lang as ink;

#[ink::contract]
pub mod delegation {
    use ink_env;
    use ink_prelude::collections::BTreeMap;
    use ink_storage::collections::HashMap as StorageHashMap;
    use ink_lang::EmitEvent;
    use registry_proxy::RegistryProxy;
    use epoch_proxy::EpochProxy;

    #[ink(storage)]
    pub struct Delegation {
        delegates: StorageHashMap<AccountId, BTreeMap<AccountId, (Balance, BlockNumber)>>,
        registry: RegistryProxy,
        epoch: EpochProxy,
    }

    /// Emitted whenever a new Link is being registered.
    #[ink(event)]
    pub struct Delegate {
        #[ink(topic)]
        name: Hash,
        #[ink(topic)]
        from: AccountId,
        #[ink(topic)]
        value: Balance,
    }

    #[ink(event)]
    pub struct Undelegate {
        #[ink(topic)]
        name: Hash,
        #[ink(topic)]
        from: AccountId,
        #[ink(topic)]
        value: Balance,
    }

    impl Delegation {
        /// Initializes the value to the initial value.
        #[ink(constructor)]
        pub fn new(init_value: RegistryProxy, epoch: EpochProxy) -> Self {
            Self {
                registry: init_value,
                epoch,
                delegates: Default::default(),
            }
        }

        fn not_expired(&self, hash_name: &Hash) -> Option<(AccountId, BlockNumber, u32)> {
            self.registry.get().not_expired(hash_name.clone())
        }

        fn get_min_threshold(&self) -> BlockNumber {
            BlockNumber::from(10u32)
        }

        fn get_current_epoch(&self) -> u32 {
            self.epoch.get().get_current_epoch()
        }         

        #[ink(message, payable, selector = 0xBABEBABE)]
        pub fn delegate(&mut self, name: Hash, from: AccountId) {
            let payment = self.env().transferred_balance();
            ink_env::debug_println!("delegate payment: {}", payment);
            if let Some((ref o, _b, _d)) = self.not_expired(&name) {
                let epoch = self.get_current_epoch();
                // let bn = self.env().block_number();
                self.delegates
                    .entry(o.clone())
                    .and_modify(|old_value_map| {
                        old_value_map
                            .entry(from.clone())
                            .and_modify(|ov| {
                                let nd = ov.0.saturating_add(payment.into());
                                *ov = (nd, ov.1)
                                // *ov = ov.saturating_add(payment.into());
                            })
                            .or_insert((payment.into(), epoch.clone()));
                        // .or_insert(payment.into());
                    })
                    .or_insert({
                        let mut map = BTreeMap::new();
                        let _ = map.insert(from, (payment.into(), epoch.clone()));
                        // let _ = map.insert(from, payment.into());
                        map
                    });
                    EmitEvent::<Self>::emit_event(
                        self.env(),
                        Delegate {
                            name,
                            from,
                            value: payment.into(),
                        }
                    );
            } else {
                ink_env::debug_println!("name not found");
                panic!("name not found");
            }
        }

        #[ink(message)]
        pub fn undelegate(&mut self, name: Hash) {
            let caller = self.env().caller();
            let epoch = self.get_current_epoch();
            // let bn = self.env().block_number();
            let min = self.get_min_threshold();
            if let Some((ref o, _b, _d)) = self.not_expired(&name) {
                if let Some(a) = self.delegates.get(&o) {
                    if let Some(d) = a.get(&caller) {
                        let p = epoch - d.1;
                        if p < min {
                            ink_env::debug_println!("min threshold not met: epoch/on/min/p {:?}/{:?}/{:?}/{:?}", epoch, d.1, min,p);
                            panic!("min threshold not met");
                        }
                    } else {
                        ink_env::debug_println!("investor not found");
                        panic!("investor not found");
                    }
                } else {
                    ink_env::debug_println!("delegate not found");
                    panic!("delegate not found");
                }
                let success = {
                    if let Some(a) = self.delegates.get(&o) {
                        if let Some(d) = a.get(&caller) {
                            let value = (*d).0;
                            // let value = *d;
                            match self.env().transfer(caller.clone(), value) {
                                Err(ink_env::Error::BelowSubsistenceThreshold) => {
                                    panic!(
                                        "requested transfer would have brought contract\
                                        below subsistence threshold!"
                                    )
                                }
                                Err(_) => panic!("transfer failed!"),
                                Ok(_) => Some(value),
                            }
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                };
                if let Some(value) = success {
                    if let Some(a) = self.delegates.get_mut(&o) {
                        if let Some(_) = a.remove(&caller) {
                            EmitEvent::<Self>::emit_event(
                                self.env(),
                                Undelegate {
                                    name,
                                    from: caller.clone(),
                                    value,
                                }
                            );
                        } else {
                            ink_env::debug_println!("failed to remove investor");
                            panic!("failed to remove investor");
                        }
                    } else {
                        ink_env::debug_println!("operator not found");
                        panic!("operator not found");
                    }
                } else {
                    ink_env::debug_println!("transfer failed");
                    panic!("transfer failed");
                }
            } else {
                ink_env::debug_println!("name not found");
                panic!("name not found");
            }
        }

        #[ink(message)]
        pub fn get_delegate(
            &self,
            investor: AccountId,
            name: Hash,
        ) -> Option<(Balance, BlockNumber)> {
            if let Some((ref o, _b, _d)) = self.not_expired(&name) {
                if let Some(a) = self.delegates.get(&o) {
                    a.get(&investor).cloned()
                } else {
                    None
                }
            } else {
                None
            }
        }
    }
}
