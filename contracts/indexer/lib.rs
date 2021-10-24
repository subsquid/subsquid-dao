#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

// const DEFAULT_ENDOWMENT: Balance = 1_000_000;
// const DEFAULT_GAS_LIMIT: Balance = 1_000_000;

#[ink::contract]
mod subsquid_indexer {
    use ink_env::{self, hash::Blake2x256};
    use ink_prelude::string::String;
    use ink_prelude::vec::Vec;
    use ink_prelude::collections::BTreeMap;
    use ink_storage::collections::HashMap as StorageHashMap;
    // use ink_storage::collections::StorageDoubleMap;
    use scale::Encode;

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    #[derive(Default)]
    pub struct Indexer {
        /// Stores a name with it's owner
        registry: StorageHashMap<Hash, (AccountId, BlockNumber, u32)>,
        commit_name: StorageHashMap<Hash, Hash>,
        commit: StorageHashMap<Hash, BlockNumber>,
        link: StorageHashMap<Hash,String>,
        capabilities: StorageHashMap<Hash, BTreeMap<Hash, String>>,
        delegates: StorageHashMap<AccountId, BTreeMap<AccountId, Balance>>,
    }

    /// Emitted whenever a new name is being registered.
    #[ink(event)]
    pub struct Register {
        #[ink(topic)]
        name: Hash,
        #[ink(topic)]
        from: AccountId,
    }

    /// Emitted whenever a new name is being registered.
    #[ink(event)]
    pub struct Unregister {
        #[ink(topic)]
        name: Hash,
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

    impl Indexer {
        /// Constructors can delegate to other constructors.
        #[ink(constructor)]
        pub fn new() -> Self {
            Default::default()
        }

        /// Simply returns the current Hash value of our `name`.
        #[ink(message)]
        pub fn get_hash(&self, name: String) -> Hash {
            Hash::from(self.env().hash_bytes::<Blake2x256>(name.as_bytes()))
        }

        #[ink(message)]
        pub fn rent_price(&self, name: String, duration: u32) -> Balance {
            let rp = name.as_bytes().len() as u32 * duration * 1_000;
            ink_env::debug_println!(
                "rent_price {} for {} is {}", name, duration, rp
            );
            Balance::from(rp)
        }

        #[ink(message)]
        pub fn valid(&self, name: String) -> bool {
            let l = name.as_bytes().len();
            ink_env::debug_println!("name size {} is {}", name, l);
            l <= 256
        }

        #[ink(message)]
        pub fn available(&self, name: String) -> bool {
            if self.valid(name.clone()) {
                let h = self.get_hash(name);
                if let Some((_, ref b, ref d)) = self.registry.get(&h) {
                    let t = self.env().block_number() - *b;
                    ink_env::debug_println!("available: {} == {}", t, *d);
                    *d <= t
                } else {
                    true
                }
            } else {
                false
            }
        }

        #[ink(message, selector = 0x1EECBEEF)]
        pub fn make_commitment(&self, name: String, owner: AccountId, secret: u32) -> Hash {
            let mut out = [0; 32];
            let mut pimage: Vec<u8> = Vec::new();
            pimage.extend_from_slice(name.as_bytes());
            let enc_owner = owner.encode();
            pimage.extend_from_slice(&enc_owner[..]);
            pimage.extend_from_slice(&secret.to_be_bytes());
            ink_env::debug_println!("preimage: {:?}", pimage);
            ink_env::hash_bytes::<Blake2x256>(&pimage, &mut out);
            ink_env::debug_println!("commitment: {:?}", out);
            Hash::from(out)
        }

        pub fn not_expired(&self, hash_name: Hash) -> Option<(&AccountId, &BlockNumber, &u32)> {
            if let Some((ref a, ref b, ref d)) = self.registry.get(&hash_name) {
                let t = self.env().block_number() - *b;
                ink_env::debug_println!("not_expired: {} == {}", t, *d);
                if t <= *d {
                    Some((a, b, d))
                } else {
                    None
                }
            } else {
                None
            }
        }

        pub fn expired(&self, hash_name: Hash) -> Option<(&AccountId, &BlockNumber, &u32)> {
            if let Some((ref a, ref b, ref d)) = self.registry.get(&hash_name) {
                let t = self.env().block_number() - *b;
                ink_env::debug_println!("expired: {} == {}", t, *d);
                if t > *d {
                    Some((a, b, d))
                } else {
                    None
                }
            } else {
                None
            }
        }

        fn not_expired_commit(&self, b: &BlockNumber) -> bool {
            let t = self.env().block_number() - *b;
            ink_env::debug_println!("not_expired_commit: {}", t);
            t <= 100
        }

        fn commit_check(&self, commitment: &Hash) {
            if let Some(ref b) = self.commit.get(commitment) {
                let valid = self.not_expired_commit(b);
                ink_env::debug_println!("commit still valid: {}", valid);
                assert!(valid, "commit still valid");
            }
            if let Some(name_hash) = self.commit_name.get(commitment) {
                let not_expired = self.not_expired(*name_hash).is_some();
                ink_env::debug_println!("name avaliable: {}", not_expired);
                assert!(not_expired, "name not avaliable");
            }
        }

        fn commit_register(&self, commitment: &Hash) {
            if let Some(ref b) = self.commit.get(commitment) {
                let not_expired_commit = self.not_expired_commit(b);
                ink_env::debug_println!(
                    "commit not expired: {}", not_expired_commit
                );
                assert!(not_expired_commit, "commit expired");
            } else {
                ink_env::debug_println!("no commit");
                panic!("no commit")
            }
        }

        #[ink(message, payable, selector = 0xDEADBEEF)]
        pub fn commit(&mut self, commitment: Hash) {
            ink_env::debug_println!("received payment: {}", self.env().transferred_balance());
            assert!(
                self.env().transferred_balance() >= 10,
                "mininum payment is ten"
            );
            self.commit_check(&commitment);
            self.commit.insert(commitment, self.env().block_number());
            ink_env::debug_println!("commited");
        }

        fn unlock_balance(&mut self, name_hash: Hash) {
            if let Some((ref a, _b, _d)) = self.registry.get(&name_hash) {
                match self.env().transfer(*a, 10) {
                    Err(ink_env::Error::BelowSubsistenceThreshold) => {
                        panic!(
                            "requested transfer would have brought contract\
                            below subsistence threshold!"
                        )
                    }
                    Err(_) => panic!("transfer failed!"),
                    Ok(_) => {}
                }
            }
            let _ = self.registry.take(&name_hash);
            let mut cn_values = self.commit_name.iter();
            let entry = cn_values.next();
            let mut nh = Hash::default();
            while entry.is_some() {
                if let Some(ref inner) = entry {
                    if *inner.1 == name_hash {
                        nh = *inner.0;
                        break;
                    }
                }
            }
            if nh != Hash::default() {
                let _ = self.commit_name.take(&nh);
            }
        }

        #[ink(message, payable, selector = 0xCAFEBABE)]
        pub fn register(&mut self, name: String, from: AccountId, duration: u32, secret: u32) {
            ink_env::debug_println!("register payment: {}", self.env().transferred_balance());
            let r = self.rent_price(name.clone(), duration);
            assert!(
                self.env().transferred_balance() >= r + 10,
                "payment was not enough for rent plus 10 (locked balance)"
            );
            let commitment = self.make_commitment(name.clone(), from, secret);
            ink_env::debug_println!("commitment: {:?}", commitment);
            self.commit_register(&commitment);
            ink_env::debug_println!("name: {:?}", name);
            let available = self.available(name.clone());
            ink_env::debug_println!("avaliable: {:?}", available);
            assert!(available, "not available");
            let name_hash = self.get_hash(name);
            ink_env::debug_println!("name_hash: {:?}", name_hash);
            if let Some((_a, _b, _d)) = self.expired(name_hash) {
                self.unregister_unchecked(name_hash);
            }
            self.registry
                .insert(name_hash, (from, self.env().block_number(), duration));
            self.commit_name.insert(commitment, name_hash);
            self.commit.take(&commitment);
            self.env().emit_event(Register {
                name: name_hash,
                from,
            });
        }

        fn unregister_unchecked(&mut self, name: Hash) {
            // let msg =
            //     ink_prelude::format!("unregister payment: {}", self.env().transferred_balance());
            ink_env::debug_println!("unregister payment: {}", self.env().transferred_balance());
            self.unlock_balance(name);
            let _ = self.link.take(&name);
            let _ = self.capabilities.take(&name);
            self.env().emit_event(Unregister { name });
        }

        fn set_link_unchecked(&mut self, name_hash: Hash, link: String) {
            ink_env::debug_println!("link name_hash: {:?}", name_hash);
            if let Some((_a, _b, _d)) = self.expired(name_hash) {
                self.unregister_unchecked(name_hash);
            } else {
                // let _ = self.link.take(&name_hash);
                // self.link.insert(name_hash, link);
                self.link.entry(name_hash)
                    .and_modify(|old_value| *old_value = link.clone())
                    .or_insert(link.clone());
                                        
                self.env().emit_event(Link { name: name_hash, link });                
            }
        }

        fn set_capability_unchecked(&mut self, name_hash: Hash, property: Hash, value: String) {
            ink_env::debug_println!("capability name_hash: {:?}", name_hash);
            if let Some((_a, _b, _d)) = self.expired(name_hash) {
                self.unregister_unchecked(name_hash);
            } else {
                self.capabilities.entry(name_hash)
                    .and_modify(|old_value| {
                        old_value.insert(property.clone(), value.clone());
                    })
                    .or_insert({
                        let mut map = BTreeMap::default();
                        let _ = map.insert(property.clone(), value.clone());
                        map
                    });
                                        
                self.env().emit_event(Capabilities { name: name_hash, property, value});
            }
        }        

        pub fn is_owner(&self, name: Hash) -> bool {
            if let Some((ref o, _b, _d)) = self.registry.get(&name) {
                self.env().caller() == *o
            } else {
                false
            }
        }

        #[ink(message, selector = 0xDEADBABE)]
        pub fn unregister(&mut self, name: Hash) {
            if self.is_owner(name) {
                self.unregister_unchecked(name);
            } else {
                ink_env::debug_println!("not the owner");
                panic!("not the owner");
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
        
        #[ink(message, selector = 0xCAFE)]
        pub fn set_capability(&mut self, name: Hash, property: Hash, value: String) {
            if self.is_owner(name) {
                self.set_capability_unchecked(name, property, value);
            } else {
                ink_env::debug_println!("not the owner");
                panic!("not the owner");
            }
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

        #[ink(message, payable, selector = 0xBABEBABE)]
        pub fn delegate(&mut self, name: Hash, from: AccountId) {
            let payment = self.env().transferred_balance();
            ink_env::debug_println!("delegate payment: {}", payment);
            if let Some((ref o, _b, _d)) = self.registry.get(&name) {
                self.delegates.entry(o.clone())
                    .and_modify(|old_value_map| {
                        old_value_map.entry(from.clone())
                            .and_modify(|ov| {
                                let _ = ov.saturating_add(payment.into());
                            })
                            .or_insert(payment.into());
                    })
                    .or_insert({
                        let mut map = BTreeMap::default();
                        let _ = map.insert(from, payment.into());
                        map
                    });

            } else {
                ink_env::debug_println!("name not found");
                panic!("name not found");
            }
        }

        // #[ink(message, payable, selector = "0xCAFEBEEF")]
        // pub fn renew(&mut self, name: String, duration: u32) {
        //     let hash_name = self.get_hash(name);
        //     if let Some((ref o, ref b, ref d)) = self.registry.get(&hash_name) {
        //         let t = self.env().block_number() - *b;
        //         ink_env::debug_println(format!("elapsed: {} == {}", t, *d).as_str());

        //     } else {
        //         ink_env::debug_println("not registered");
        //         panic!("not registered");
        //     }
        // }
    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(not(feature = "ink-experimental-engine"))]
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;
        use ink_env::{call, test};
        /// Imports `ink_lang` so we can use `#[ink::test]`.
        use ink_lang as ink;

        /// We test if the default constructor does its job.
        #[ink::test]
        fn default_works() {
            let _ = Indexer::default();
            assert!(true);
        }

        /// We test a simple use case of our contract.
        #[ink::test]
        fn register_works() {
            // given
            let contract_balance = 100;
            let accounts = default_accounts();
            set_sender(accounts.alice);
            set_balance(contract_id(), contract_balance);
            let mut myns = Indexer::default();

            // when
            set_sender(accounts.eve);
            set_balance(accounts.eve, 100);

            assert!(myns.available("myname".to_owned()));
            let commitment = myns.make_commitment("myname".to_owned(), accounts.eve, 1);
            commit(&accounts.eve, &commitment);
            myns.commit(commitment);
            assert!(myns.available("myname".to_owned()));
            register("myname".to_owned(), &accounts.eve, 100, 1);
            myns.register("myname".to_owned(), accounts.eve, 100, 1);
            assert!(!myns.available("myname".to_owned()));
        }

        #[ink::test]
        fn setlink_works() {
            // given
            let contract_balance = 100;
            let accounts = default_accounts();
            set_sender(accounts.alice);
            set_balance(contract_id(), contract_balance);
            let mut myns = Indexer::default();

            // when
            set_sender(accounts.eve);
            set_balance(accounts.eve, 100);
            let myname = "myname".to_owned();

            assert!(myns.available("myname".to_owned()));
            let commitment = myns.make_commitment("myname".to_owned(), accounts.eve, 1);
            commit(&accounts.eve, &commitment);
            myns.commit(commitment);
            assert!(myns.available("myname".to_owned()));
            register("myname".to_owned(), &accounts.eve, 100, 1);
            myns.register("myname".to_owned(), accounts.eve, 100, 1);
            assert!(!myns.available("myname".to_owned()));
            let h = myns.get_hash(myname.clone());
            myns.set_link(h.clone(), "https://subsquid.io/".to_owned());
            assert!("https://subsquid.io/".to_owned() == myns.get_link(h.clone()).unwrap());
        }      
        
        #[ink::test]
        fn setcapabilities_works() {
            // given
            let contract_balance = 100;
            let accounts = default_accounts();
            set_sender(accounts.alice);
            set_balance(contract_id(), contract_balance);
            let mut myns = Indexer::default();

            // when
            set_sender(accounts.eve);
            set_balance(accounts.eve, 100);
            let myname = "myname".to_owned();

            assert!(myns.available("myname".to_owned()));
            let commitment = myns.make_commitment("myname".to_owned(), accounts.eve, 1);
            commit(&accounts.eve, &commitment);
            myns.commit(commitment);
            assert!(myns.available("myname".to_owned()));
            register("myname".to_owned(), &accounts.eve, 100, 1);
            myns.register("myname".to_owned(), accounts.eve, 100, 1);
            assert!(!myns.available("myname".to_owned()));
            let h = myns.get_hash(myname.clone());
            let c = myns.get_hash("BTC".to_owned());
            myns.set_capability(h.clone(), c.clone(),"ALL".to_owned());
            let caps = myns.get_capabilities(h.clone());
            let value = caps.get(&c).cloned();
            assert!(value.is_some());
            assert!("ALL".to_owned() == value.unwrap());
        }           

        #[ink::test]
        fn delegate_works() {
            // given
            let contract_balance = 100;
            let accounts = default_accounts();
            set_sender(accounts.alice);
            set_balance(contract_id(), contract_balance);
            let mut myns = Indexer::default();

            // when
            set_sender(accounts.eve);
            set_balance(accounts.eve, 100);
            let myname = "myname".to_owned();

            assert!(myns.available("myname".to_owned()));
            let commitment = myns.make_commitment("myname".to_owned(), accounts.eve, 1);
            commit(&accounts.eve, &commitment);
            myns.commit(commitment);
            assert!(myns.available("myname".to_owned()));
            register("myname".to_owned(), &accounts.eve, 100, 1);
            myns.register("myname".to_owned(), accounts.eve, 100, 1);
            assert!(!myns.available("myname".to_owned()));
            let h = myns.get_hash(myname.clone());
            delegate(h.clone(), &accounts.alice);
            myns.delegate(h.clone(), accounts.alice.clone());
            assert!(true)
        }   

        fn contract_id() -> AccountId {
            ink_env::test::get_current_contract_account_id::<ink_env::DefaultEnvironment>()
                .expect("Cannot get contract id")
        }

        fn set_sender(sender: AccountId) {
            let callee =
                ink_env::account_id::<ink_env::DefaultEnvironment>().unwrap_or([0x0; 32].into());
            test::push_execution_context::<Environment>(
                sender,
                callee,
                1000000,
                1000000,
                test::CallData::new(call::Selector::new([0x00; 4])), // dummy
            );
        }

        fn default_accounts() -> ink_env::test::DefaultAccounts<ink_env::DefaultEnvironment> {
            ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
                .expect("Off-chain environment should have been initialized already")
        }

        fn set_balance(account_id: AccountId, balance: Balance) {
            ink_env::test::set_account_balance::<ink_env::DefaultEnvironment>(account_id, balance)
                .expect("Cannot set account balance");
        }

        fn commit(account: &AccountId, commitment: &Hash) {
            // 0xDEADBEEF
            let mut data = ink_env::test::CallData::new(ink_env::call::Selector::new([
                0xDE, 0xAD, 0xBE, 0xEF,
            ]));
            data.push_arg(commitment);
            let mock_transferred_balance = 100;

            // Push the new execution context which sets Eve as caller and
            // the `mock_transferred_balance` as the value which the contract
            // will see as transferred to it.
            ink_env::test::push_execution_context::<ink_env::DefaultEnvironment>(
                *account,
                contract_id(),
                1000000,
                mock_transferred_balance,
                data,
            );
        }

        fn register(name: String, from: &AccountId, duration: u32, secret: u32) {
            // 0xCAFEBABE
            let mut data = ink_env::test::CallData::new(ink_env::call::Selector::new([
                0xCA, 0xFE, 0xBA, 0xBE,
            ]));
            data.push_arg(&name);
            data.push_arg(from);
            data.push_arg(&duration);
            data.push_arg(&secret);
            let mock_transferred_balance = 600010;

            // Push the new execution context which sets 'from' as caller and
            // the `mock_transferred_balance` as the value which the contract
            // will see as transferred to it.
            ink_env::test::push_execution_context::<ink_env::DefaultEnvironment>(
                *from,
                contract_id(),
                1000000,
                mock_transferred_balance,
                data,
            );
        }

        fn delegate(name: Hash, from: &AccountId) {
            // 0xBABEBABE
            let mut data = ink_env::test::CallData::new(ink_env::call::Selector::new([
                0xBA, 0xBE, 0xBA, 0xBE,
            ]));
            data.push_arg(&name);
            data.push_arg(from);
            let mock_transferred_balance = 600010;

            // Push the new execution context which sets 'from' as caller and
            // the `mock_transferred_balance` as the value which the contract
            // will see as transferred to it.
            ink_env::test::push_execution_context::<ink_env::DefaultEnvironment>(
                *from,
                contract_id(),
                1000000,
                mock_transferred_balance,
                data,
            );
        }        
    }
}
