fn main() {
    let fahrenheit = 72.0;
    let celsius = 32.0;
    println!("{fahrenheit} degrees fahrenheit is ~{:.1} degrees celsius.\n", 
        ccpractice::fahrenheit_to_celsius(fahrenheit));
    
    println!("{celsius} degrees celsius is ~{:.1} degrees fahrenheit.\n", 
        ccpractice::celsius_to_fahrenheit(celsius));
    
    let fibonacci_target = 13;
    println!("Number {fibonacci_target} of the Fibonacci Sequence is {}.\n",
        ccpractice::find_nth_fibonacci(fibonacci_target));
    
    ccpractice::print_twelve_days_of_christmas()
}

mod common_concepts_practice;

use crate::common_concepts_practice as ccpractice;