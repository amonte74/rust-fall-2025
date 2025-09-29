
// Car

// Struct responsible for data
struct Car {
    seats: u8,
    model: String,
}

// Student

// Struct for student
struct Student {
    name: String,
    major: String,
}

impl Student {
    fn new(n:String,m:String) -> Studnet {
        Student {
            name: n,
            major: m,
        }
    }
}

// Methods are added by MPL statement

impl Car {
    fn new(s:u8,m:String) -> Car { // static method
        Car {
            seats: s,
            model: m,
        }
    }
}

fn main() {
    let my_car = Car::new(4,"Tacoma".to_string());

    println!("Number of seats {}",my_car.seats);
    println!("Number of seats {}",my_car.model);



}
