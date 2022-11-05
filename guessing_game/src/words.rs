pub mod words {
    use lazy_static::lazy_static;
    use std::collections::HashMap;

    lazy_static! {
        pub static ref WORDS: HashMap<&'static str, NumberedWord> = {
            let mut map = HashMap::new();
            map.insert(
                "try",
                NumberedWord {
                    singular: "try".to_string(),
                    plural: "tries".to_string(),
                }
                );
            map.insert(
                "win",
                NumberedWord {
                    singular: "win".to_string(),
                    plural: "wins".to_string(),
                }
                );
            map
        }; 
    }

    pub struct NumberedWord {
        singular: String,
        plural: String,
    }

    impl NumberedWord {
        pub fn get_correct_form(&self, number: u32) -> &str {
            return if number == 1 { 
                self.singular.as_str()
            } else {
                self.plural.as_str()
            }
        }
    }
}
