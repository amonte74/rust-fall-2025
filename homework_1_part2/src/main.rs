
fn is_even(n: i32) -> bool {
    n % 2 == 0
}

fn main() {
    let numbers: [i32; 10] = [3, 5, 15, 8, 12, 7, 20, 1, 9, 25];

    // Check each number with for loop
    for &num in numbers.iter() {
        if num % 3 == 0 && num % 5 == 0 {
            println!("{} -> FizzBuzz", num);
        } else if num % 3 == 0 {
            println!("{} -> Fizz", num);
        } else if num % 5 == 0 {
            println!("{} -> Buzz", num);
        } else if is_even(num) {
            println!("{} -> Even", num);
        } else {
            println!("{} -> Odd", num);
        }
    }

    // Find sum using while loop
    let mut i = 0;
    let mut sum = 0;
    while i < numbers.len() {
        sum += numbers[i];
        i += 1;
    }
    println!("Sum of numbers: {}", sum);

    // Find largest number using loop
    let mut largest = numbers[0];
    let mut j = 1;
    loop {
        if j >= numbers.len() {
            break;
        }
        if numbers[j] > largest {
            largest = numbers[j];
        }
        j += 1;
    }
    println!("Largest number: {}", largest);
}
