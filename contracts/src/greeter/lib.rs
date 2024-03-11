#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod salary {
    use ink::prelude::string::String;

    #[ink(event)]
    pub struct Salary {
        from: Option<AccountId>,
        message: String,
    }

    #[ink(storage)]
    pub struct Salary {
        message: String,
    }

    impl Salary {
        
        #[ink(constructor)]
        pub fn new(init_value: String) -> Self {
            Self {
                message: init_value,
            }
        }

        #[ink(constructor)]
        pub fn default() -> Self {
            let default_message = String::from("Hello ink!");
            Self::new(default_message)
        }

        #[ink(message)]
        pub fn slr(&self) -> String {
            self.message.clone()
        }

        #[ink(message)]
        pub fn set_message(&mut self, new_value: String) {
            self.message = new_value.clone();

            let from = self.env().caller();
            self.env().emit_event(slry {
                from: Some(from),
                message: new_value,
            });
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn new_works() {
            let message = "Hello ink! v4".to_string();
            let salary = Salary::new(message.clone());
            assert_eq!(salary.slr(), message);
        }

        #[ink::test]
        fn default_new_works() {
            let salary = Salary::default();
            let default_message = String::from("Hello ink!");
            assert_eq!(salary.slr(), default_message);
        }

        #[ink::test]
        fn set_message_works() {
            let message_1 = String::from("gm ink!");
            let mut salary = Salary::new(message_1.clone());
            assert_eq!(salary.slr(), message_1);
            let message_2 = String::from("gn");
            salary.set_message(message_2.clone());
            assert_eq!(salary.slr(), message_2);
        }
    }
}
