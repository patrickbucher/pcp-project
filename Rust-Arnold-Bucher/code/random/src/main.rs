extern crate rand;

use rand::distributions::Distribution;
use rand::distributions::Uniform;
use rand::thread_rng;

fn main() {
    generate_sort_and_print_random_ints(10, 100)
}

fn generate_sort_and_print_random_ints(amount: usize, max: i32) {
    let mut numbers = generate_random_numbers(max, amount);

    numbers.sort();
    numbers.reverse();

    let result = numbers_to_strings(numbers).join(" > ");
    println!("reverse ordered list = {{ {} }}", result)
}

fn generate_random_numbers(max: i32, amount: usize) -> Vec<i32> {
    let mut rng = thread_rng();

    return Uniform::new(0, max)
        .sample_iter(&mut rng)
        .take(amount)
        .collect();
}

fn numbers_to_strings(numbers: Vec<i32>) -> Vec<String> {
    return numbers.iter().map(|n| n.to_string()).collect();
}
