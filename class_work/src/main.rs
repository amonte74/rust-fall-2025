
fn append_region(mut word:String) {
    word.push_str("RGV");
    
}

fn main() {
    
    let mut x = "UT".to_string();
    append_region(&mut x);
    println!("{}",x);

}

// fn main() {
//     let mut word = "UT".to_string();

//     {
//     let mutate_word = &mut word;
//     mutate_word.push_str("RGV")
//     }

//     println!("{}", word);
// }

// fn main() {
//     let mut word = "UT".to_string(); 
//     fn update(word: &mut String) {
//         word.push_str("RGV");
//     }
//     update(&mut word);
//     println!("{word}")
// }
