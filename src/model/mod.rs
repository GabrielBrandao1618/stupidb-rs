use serde::{Serialize,Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Person {
    pub name: String,
    pub age: u16,
    pub id: String,
}
