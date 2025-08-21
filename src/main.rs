use std::collections::HashMap;

fn main() {
    trait Person {
        fn get_name(&self) -> String;
        fn get_age(&self) -> i32;
        fn say(&self) {
            println!("Hello, {}", self.get_name());
        }
    }
}
