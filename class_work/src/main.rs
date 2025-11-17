
fn box_polymorphism() {
    use core::fmt::Debug;
    
    trait Animal: Debug {
        fn sound(&self) -> String;
    }
    // inside of the struct create a field called name
    // and beside sounds your animal should print "hey my name is {}",name
    
    #[derive(Debug)]
    struct Dog{
        name:String,
    };
    
    impl Animal for Dog {
        fn sound(&self) -> String {
            "Woof woof. My name is {}".to_string()
        }
    }
    
    #[derive(Debug)]
    struct Cat;
    
    impl Animal for Cat {
        fn sound(&self) -> String {
            "Meow meow".to_string()
        }
    }

    
    let mut zoo: Vec<Box<dyn Animal>> = Vec::new();
    
    zoo.push(Box::new(Dog{name:"Lucy"}));
    zoo.push(Box::new(Cat{}));
    
    for animal in zoo {
        println!("{}", animal, animal.sound());
    }
}

fn main() {
    box_polymorphism();
}
