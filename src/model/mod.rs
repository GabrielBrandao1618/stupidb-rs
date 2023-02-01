use uuid::Uuid;
use serde::{Serialize,Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Person {
    pub name: String,
    pub age: u16,
    pub id: String,
}

impl Person {
    pub fn new(name: String, age: u16) -> Person {
        let id = Uuid::new_v4();
        Person{name, age, id: id.to_string()} 
    }
}
