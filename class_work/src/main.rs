

fn pattern_match_simple(num:u32) -> String {
 
    match (num%3==0,num%5==0) {
        (true,true) => "FizzBuzz".to_string(),
        (true,false) => "Fizz".to_string(),
        (false,true) => "Buzz".to_string(),
        (false,false) => num.to_string(),
    }

}

fn main() {
    for num in 1..20 {
        println!("{}",pattern_match_simple(num));
    }
    
}
