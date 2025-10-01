
// Car

// Struct responsible for data
struct Car {
    seats: u8,
    model: String,
}



impl Car {
    fn new(s:u8,m:String) -> Self { // static method
        Self {
            seats: s,
            model: m,
        }
    }

    fn get_model( &self ) -> &String {
        return &self.model
    }

    fn set_model(&mut self, new_model: String){
        self.model = new_model;

    }
}

fn main() {
    let mut my_car = Car::new(4,"Tacoma".to_string());

    println!("Number of seats {}",my_car.seats);
    println!("Number of seats {}",my_car.get_model());
    my_car.set_model("Corolla".to_string());

    println!("Number of seats {}",my_car.get_model());



}
