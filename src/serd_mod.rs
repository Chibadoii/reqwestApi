


pub mod serd
{
    use crate::serd_mod::serd::{Serialize, Deserialize};
    #[derive(Serialize, Deserialize)]
    struct Person {
        name: String,
        age: u32,
    }
    fn create_serd_info() {
        let person = Person {
            name: String::from("Alice"),
            age: 30,
        };

        let serialized = serde_json::to_string(&person).unwrap();
        println!("{}", serialized);
    }
}