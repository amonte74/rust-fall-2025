
fn check_guess(guess: i32, secret: i32) -> i32 {
    if guess == secret {
        0
    } else if guess > secret {
        1
    } else {
        -1
    }
}


fn main() {
    let secret: i32 = 7; // hardcoded secret number
    let mut guess: i32 = 0; // will simulate guesses
    let mut attempts: i32 = 0;

    loop {
        attempts += 1;
        guess += 1; // simulate guessing (1,2,3,...)

        let result = check_guess(guess, secret);

        if result == 0 {
            println!("Guess {}: {} is correct!", attempts, guess);
            break;
        } else if result == 1 {
            println!("Guess {}: {} is too high!", attempts, guess);
        } else {
            println!("Guess {}: {} is too low!", attempts, guess);
        }
    }

    println!("It took {} guesses to find the secret number!", attempts);
}
