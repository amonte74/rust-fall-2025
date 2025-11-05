fn main(){
    use serde::{Serialize, Deserialize};

    #[derive(Serialize,Deserialize)]
    struct Person {
        name: String,
        age: u8,
    }


    let person = Person {
        name: "John Doe".to_string(),
        age: 30,
    };

    let serialized = serde_json::to_string(&person).unwrap();// unwrap panicks

    // let actual_string = match serialized {
    //     Ok(s) => s,
    //     Err(e) => "Houston we have a Problem".to_string(),
    // };

    println!("Serialized Person = {}", serialized);

    let person:Person =serde_json::from_str::<Person>(serialized.as_str()).unwrap();

    println!("Deserialized Person = {}, {}", person.name, person.age);
}

