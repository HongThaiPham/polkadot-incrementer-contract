#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod incrementer {
    #[ink(storage)]
    pub struct Incrementer {
        value: i32,
    }

    impl Incrementer {
        #[ink(constructor)]
        pub fn new(init_value: i32) -> Self {
            // Contract Constructor
            Self{value: init_value}
        }

        pub fn default() -> Self {
            Self{value: 0}
        }

        #[ink(message)]
        pub fn get(&self) -> i32 {
            // Contract Message
            self.value
        }

        #[ink(message)]
        pub fn inc(&mut self, by: i32) {
            // ACTION: Simply increment `value` by `by`
            self.value = self.value + by
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use ink_lang as ink;

        #[ink::test]
        fn default_works() {
            // Test Your Contract
            let contract = Incrementer::default();
            assert_eq!(contract.get(), 0);
        }

        #[ink::test]
        fn it_works() {
            let mut contract = Incrementer::new(42);
            assert_eq!(contract.get(), 42);
            contract.inc(5);
            assert_eq!(contract.get(), 47);
            contract.inc(-50);
            assert_eq!(contract.get(), -3);
        }
    }
}
