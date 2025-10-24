use std::fs::File;
use std::io::{Write, BufReader, BufRead};
use std::error::Error;

struct Book {
    title: String,
    author: String,
    year: u16,
}

fn save_books(books: &Vec<Book>, filename: &str) {
    // Create or overwrite the file
    let mut file = File::create(filename).expect("Unable to create file");

    for book in books {
        // Save each book on its own line, separated by commas
        writeln!(file, "{},{},{}", book.title, book.author, book.year)
            .expect("Unable to write to file");
    }
}

fn load_books(filename: &str) -> Vec<Book> {
    let file = File::open(filename).expect("Unable to open file");
    let reader = BufReader::new(file);

    let mut books = Vec::new();

    for line in reader.lines() {
        if let Ok(line) = line {
            let parts: Vec<&str> = line.split(',').collect();
            if parts.len() == 3 {
                if let Ok(year) = parts[2].trim().parse::<u16>() {
                    books.push(Book {
                        title: parts[0].to_string(),
                        author: parts[1].to_string(),
                        year,
                    });
                }
            }
        }
    }

    books
}

fn main() -> Result<(), Box<dyn Error>> {
    let books = vec![
        Book {
            title: "1984".to_string(),
            author: "George Orwell".to_string(),
            year: 1949,
        },
        Book {
            title: "To Kill a Mockingbird".to_string(),
            author: "Harper Lee".to_string(),
            year: 1960,
        },
        Book {
            title: "Brave New World".to_string(),
            author: "Aldous Huxley".to_string(),
            year: 1932,
        },
    ];

    save_books(&books, "books.txt");
    println!("✅ Books saved to file.");

    let loaded_books = load_books("books.txt");
    println!("Loaded books:");
    for book in loaded_books {
        println!("{} by {}, published in {}", book.title, book.author, book.year);
    }

    Ok(())
}
