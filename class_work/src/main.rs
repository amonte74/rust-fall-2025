use std::io::{self, Read, Write};

struct Student {
    name: String,
    major: String,
}

fn reading_from_console() {
    let mut buffer = String::new();

    print!("What's your name? ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buffer).unwrap();
    let name = buffer.trim().to_string();
    buffer.clear();

    print!("What's your Major? ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buffer).unwrap();
    let major = buffer.trim().to_string();
    buffer.clear();

    let student = Student { name, major };
    println!("Hi {}, your major is {}!", student.name, student.major);
}

fn main() {
    reading_from_console();
}