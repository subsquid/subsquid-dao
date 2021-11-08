#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

pub use self::epoch::Epoch;

#[ink::contract]
mod epoch {

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct Epoch {
        offset: BlockNumber,
        period: BlockNumber,
    }

    impl Epoch {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(offset: BlockNumber, period: BlockNumber) -> Self {
            Self { offset, period }
        }

        /// set the offset from genesis where period begin.
        #[ink(message)]
        pub fn set_offset(&mut self, offset: BlockNumber) {
            self.offset = offset;
        }

        /// set period for each epoch.
        #[ink(message)]
        pub fn set_period(&mut self, period: BlockNumber) {
            self.period = period;
        }

        /// Simply returns the current value of offset.
        #[ink(message)]
        pub fn get_offset(&self) -> BlockNumber {
            self.offset
        }

        /// Simply returns the current value of offset.
        #[ink(message)]
        pub fn get_period_length(&self) -> BlockNumber {
            self.period
        }

        /// Simply returns the current value of epoch.
        #[ink(message)]
        pub fn get_current_epoch(&self) -> u32 {
            let off = self.env().block_number() - self.offset;
            off / self.period
        }

        /// Simply returns the value of epoch since param.
        #[ink(message)]
        pub fn get_current_epoch_since(&self, since: BlockNumber) -> u32 {
            let s = (since - self.offset) / self.period;
            let off = (self.env().block_number() - self.offset) / self.period;
            off - s
        }

        /// Simply returns the current value of block inside epoch.
        #[ink(message)]
        pub fn get_current_block(&self) -> u32 {
            let off = self.env().block_number() - self.offset;
            off % self.period
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
            let epoch = Epoch::default();
            assert_eq!(epoch.get(), false);
        }

        /// We test a simple use case of our contract.
        #[ink::test]
        fn it_works() {
            let mut epoch = Epoch::new(false);
            assert_eq!(epoch.get(), false);
            epoch.flip();
            assert_eq!(epoch.get(), true);
        }
    }
}
