// // Problem 1
// fn concat_strings(s1: &String, s2: &String) -> String {
//     // Concatenate the two strings by borrowing them
//     let result = s1.clone() + s2;  // Clone the first string and concatenate with the second
//     result
// }

// fn main() {
//     let s1 = String::from("Hello, ");
//     let s2 = String::from("World!");
//     let result = concat_strings(&s1, &s2);
//     println!("{}", result); // Should print: "Hello, World!"
// }

// // Problem 2
// fn clone_and_modify(s: &String) -> String {
//     // Your code here
//     let mut cloned = s.clone();
//     cloned.push_str("World!"); // Append to the cloned string
//     cloned
// }

// fn main() {
//     let s = String::from("Hello, ");
//     let modified = clone_and_modify(&s);
//     println!("Original: {}", s); // Should print: "Original: Hello, "
//     println!("Modified: {}", modified); // Should print: "Modified: Hello, World!"
// }

// Problem 3
#[allow(unused_variables, unused_mut)]
fn sum(total: &mut i32, low: i32, high: i32) {
    // Write your code here!
    for i in low..=high {
        *total += i;  // Dereference the mutable reference to add values to total
    }
}

fn main() {
    // create necessary variables and test your function for low 0 high 100
    // total should be 5050
    let mut total = 0;
    let low = 0;
    let high = 100;
    sum(&mut total, low, high);
    println!("Total sum from {} to {}: {}", low, high, total); 
}
