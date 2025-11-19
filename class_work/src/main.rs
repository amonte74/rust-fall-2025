fn process_vector<F>(vec: Vec<i32>, f: F) -> Vec<i32>
where
    F: Fn(i32) -> i32,
{
    // Your implementation here
    vec.into_iter().map(f).collect()
}

fn main() {
    let numbers = vec![1, 2, 3];

    let doubled = process_vector(numbers.clone(), |x| {
        // Implement: multiply each number by 2
        x
    });

    let replaced = process_vector(numbers, |x| {
        // Implement: if number > 2, replace with 0, else keep number
        x
    });

    println!("Doubled: {:?}", doubled);
    println!("Replaced: {:?}", replaced);
}
