use std::process::Command;
use std::io::{self, Write};
use std::fs::File;

fn create_and_write_to_file(filename: &String) {
    let mut file = File::create(&filename).unwrap();
    writeln!(file, "Hello, Rust file operations!").unwrap();
    writeln!(file, "This is a new line.").unwrap();
}

fn reading_from_console(msg:&String) -> String{
    let mut buffer = String::new();

    print!("{}",msg);
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buffer).unwrap();
    let name = buffer.trim().to_string();
    buffer.clear();
    return name;
}

fn read_file_linux(filename:String) {
    let output = Command::new("cat")
        .arg(&filename)
        .output()
        .expect("Failed to execute command");

    println!("Command output: {}", String::from_utf8_lossy(&output.stdout));
}

fn main() {

    while true {
        
        let message = "What do you want to do today? Type 1 to create file, 2 to read from file, 3 to finish the program.".to_string();
        println!("{}",message);
        let choice = reading_from_console(&"Type the number: ".to_string()).parse().unwrap();

        match choice {
            1 => {
                let message = "What is the name of your new file? ".to_string();
                let filename = reading_from_console(&message);
                create_and_write_to_file(&filename);

            },
            2 => {
                let message = "What file to open?".to_string();
                let filename = reading_from_console(&"What file to open? ".to_string()); //accept from the user
                read_file_linux(filename); 
            }
            3 => break,
            _ => {}
        }
    }
    // ask the user if he wants to create a file and write
    // => create a file and write inside of if
    // accept everything from the console
    //filename he wants to create and actual content
    //refer to 03rust-file-operations.md
    
    // or if he wants to read from existent file

    // accept from the user file he wants to open
    // let message = "What file to open?"
    // let filename = reading_from_console(&"What file to open? ".to_string()); //accept from the user
    // read_file_linux(filename);
}
