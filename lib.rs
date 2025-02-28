#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(not(feature = "std"), no_main)]

#[ink::contract]
mod calculator {
    #[ink(storage)]
    pub struct Calculator {
        last_result: i32,
    }

    impl Calculator {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self { last_result: 0 }
        }

        /// Plus 2 numbers and updates the last result and returns sum
        #[ink(message)]
        pub fn add(&mut self, a: i32, b: i32) -> i32 {
            self.last_result = a + b;
            self.last_result
        }

        /// Minus the second number from the first updates the last result, and returns mun
        #[ink(message)]
        pub fn subtract(&mut self, a: i32, b: i32) -> i32 {
            self.last_result = a - b;
            self.last_result
        }

        /// Multiplie 2 numbers, updates the last result, and returns
        #[ink(message)]
        pub fn multiply(&mut self, a: i32, b: i32) -> i32 {
            self.last_result = a * b;
            self.last_result
        }

        /// Divide  first number by the second and updates the last result and returns
        /// Returns `None` if the divisor is 0 and does not update the last result
        #[ink(message)]
        pub fn divide(&mut self, a: i32, b: i32) -> Option<i32> {
            if b == 0 {
                None
            } else {
                self.last_result = a / b;
                Some(self.last_result)
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn addition_works() {
            let mut calculator = Calculator::new();
            assert_eq!(calculator.add(5, 3), 8);
            assert_eq!(calculator.last_result, 8);
        }

        #[ink::test]
        fn subtraction_works() {
            let mut calculator = Calculator::new();
            assert_eq!(calculator.subtract(5, 3), 2);
            assert_eq!(calculator.last_result, 2);
        }

        #[ink::test]
        fn multiplication_works() {
            let mut calculator = Calculator::new();
            assert_eq!(calculator.multiply(5, 3), 15);
            assert_eq!(calculator.last_result, 15);
        }

        #[ink::test]
        fn division_works() {
            let mut calculator = Calculator::new();
            assert_eq!(calculator.divide(6, 3), Some(2));
            assert_eq!(calculator.last_result, 2);
        }

        #[ink::test]
        fn division_by_zero_does_not_update_last_result() {
            let mut calculator = Calculator::new();
            assert_eq!(calculator.divide(6, 0), None);
            assert_eq!(calculator.last_result, 0);
        }
    }
}
