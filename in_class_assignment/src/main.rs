// Student

struct Student {
    name: String,
    major: String,
}

impl Student {
    fn new(n:String,m:String) -> Student {
    Self {
            name: n,
            major: m,
        }
    }

    fn get_major(&self) -> &String {
        return &self.major;
    }

    fn set_major( &mut self, new_major: String) {
        self.major = new_major;
    }
}

fn main() {
    let mut my_student = Student::new("Arturo Montemayor".to_string(),"Computer Engineering".to_string());

    println!("My name is {}", my_student.name);
    println!("My major is {}", my_student.get_major());
    my_student.set_major("Computer Engineering".to_string());

    println!("My major is {}", my_student.get_major());
}