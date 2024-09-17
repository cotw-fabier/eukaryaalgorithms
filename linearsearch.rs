use std::io;
use std::time::Instant;

fn linear_search(arr: &[u64], target: u64) -> (Option<usize>, usize) {
    let mut tries = 0;
    for (index, &value) in arr.iter().enumerate() {
        tries += 1;
        if value == target {
            return (Some(index), tries);
        }
    }
    (None, tries)
}

fn main() {
    // Get maximum value for the series from user
    let mut input = String::new();
    println!("Enter the maximum value for the series (1-X): ");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input.");
    let max_value: u64 = input.trim().parse().expect("Invalid number.");

    // Generate a vector of numbers from 1 to max_value
    let numbers: Vec<u64> = (1..=max_value).collect();

    // Ask the user for the number to search for
    let mut input = String::new();
    println!("Enter a number between 1 and {} to search for: ", max_value);
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input.");
    let target: u64 = input.trim().parse().expect("Invalid number.");

    // Start timing
    let start = Instant::now();

    // Perform linear search
    let (result, tries) = linear_search(&numbers, target);

    // End timing
    let duration = start.elapsed();

    // Display results
    match result {
        Some(index) => println!(
            "Number {} found at index {} after {} tries.",
            target, index, tries
        ),
        None => println!("Number {} not found after {} tries.", target, tries),
    }

    println!(
        "Time taken to search: {:.6?} seconds.",
        duration.as_secs_f64()
    );
}
