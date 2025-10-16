#[derive(Debug)]
enum Fruit {
    Apple(String),
    Banana(String),
    Tomato(String),
}

struct Inventory {
    fruit:Vec<Fruit>,
}

impl Inventory {
    fn available_fruits(&self){
        for f in &self.fruit{
            println!("{:?} :",f);
            Self::tell_me_joke(f);
        }
    }

    fn tell_me_joke(fruit:&Fruit){
        match fruit {
            Fruit::Apple(msg) => println!("{}",msg),
            Fruit::Banana(msg) => println!("{}",msg),
            Fruit::Tomato(msg) => println!("{}",msg),

        }
    }
}

fn main(){
    let a = "Why was the apple so sad? Because he lost his core values.".to_string();
    let b = "Why do bananas never feel lonely? They all hang out in bunches.".to_string();
    let t = "Why don’t tomatoes ever win arguments? They always end up stewing in their own juices.".to_string();
    let fruits = vec![Fruit::Banana(b),Fruit::Apple(a),Fruit::Tomato(t)];
    
    let grocery_store = Inventory {
        fruit:fruits,
    };

    grocery_store.available_fruits();
}






