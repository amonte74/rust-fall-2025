// define 2 struct undegrad and grad student

// define trait show info

// grad student should have a thesis compnent
// gpa and major will be shared

// create another struct called Enrollment
// inside enrollment store undegrad and grads together
// implement show_info  for all enrolled student

// everywhere use generics and traits, no if or match statement
// program to behavior only

trait ShowInfo {
    fn show_info(&self);
}

struct Undergrad {
    name: String,
    gpa: f32,
    major: String,
}

struct Grad {
    name: String,
    gpa: f32,
    major: String,
    thesis: String,
}

impl ShowInfo for Undergrad {
    fn show_info(&self) {
        println!(
            "Undergrad Student -> Name: {}, GPA: {:.2}, Major: {}",
            self.name, self.gpa, self.major
        );
    }
}

impl ShowInfo for Grad {
    fn show_info(&self) {
        println!(
            "Grad Student -> Name: {}, GPA: {:.2}, Major: {}, Thesis: {}",
            self.name, self.gpa, self.major, self.thesis
        );
    }
}

struct Enrollment<T: ShowInfo> {
    students: Vec<T>,
}

impl<T: ShowInfo> Enrollment<T> {
    fn new() -> Self {
        Enrollment { students: Vec::new() }
    }

    fn enroll(&mut self, student: T) {
        self.students.push(student);
    }

    fn show_all(&self) {
        for s in &self.students {
            s.show_info();
        }
    }
}

fn main(){
    let u1 = Undergrad {
        name: String::from("Alice"),
        gpa: 3.8,
        major: String::from("Computer Science"),
    };

    let g1 = Grad {
        name: String::from("Bob"),
        gpa: 3.9,
        major: String::from("Electrical Engineering"),
        thesis: String::from("AI-driven Circuits"),
    };

    let mut undergrad_enrollment = Enrollment::<Undergrad>::new();
    undergrad_enrollment.enroll(u1);

    let mut grad_enrollment = Enrollment::<Grad>::new();
    grad_enrollment.enroll(g1);

    println!("--- Undergrad Students ---");
    undergrad_enrollment.show_all();

    println!("\n--- Grad Students ---");
    grad_enrollment.show_all();
}

