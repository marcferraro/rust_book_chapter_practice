fn main() {
    let fahrenheit = 72.0;
    let celsius = 32.0;
    println!("{fahrenheit} degrees fahrenheit is ~{:.1} degrees celsius.", 
        fahrenheit_to_celsius(fahrenheit));
    
    println!("{celsius} degrees celsius is ~{:.1} degrees fahrenheit.", 
        celsius_to_fahrenheit(celsius));
    
    let fibonacci_target = 13;
    println!("Number {fibonacci_target} of the Fibonacci Sequence is {}.",
        find_nth_fibonacci(fibonacci_target));
}

// Convert temperatures between Fahrenheit and Celsius.
fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (5.0/9.0) * (fahrenheit - 32.0)
}

fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    (9.0/5.0 * celsius)+ 32.0
}

// Generate the nth Fibonacci number.
fn find_nth_fibonacci(target: i32) -> i32 {
    if target == 1 || target == 2 {
        return target - 1
    }

    let mut last_two_nums: [i32; 2] = [0, 1];

    for _n in  3..=target {
        let sum = last_two_nums[0] + last_two_nums[1];
        last_two_nums[0] = last_two_nums[1];
        last_two_nums[1] = sum;
    }

    last_two_nums[1]
}

// Print the lyrics to the Christmas carol “The Twelve Days of Christmas,” 
// taking advantage of the repetition in the song.
// fn print_twelve_days_of_christmas() {
    
// }