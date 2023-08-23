mod common_concepts_practice;
mod common_collections_practice;

fn main() {
    let fahrenheit = 72.0;
    let celsius = 32.0;
    println!("{fahrenheit} degrees fahrenheit is ~{:.1} degrees celsius.\n", 
        common_concepts_practice::fahrenheit_to_celsius(fahrenheit));
    
    println!("{celsius} degrees celsius is ~{:.1} degrees fahrenheit.\n", 
        common_concepts_practice::celsius_to_fahrenheit(celsius));
    
    let fibonacci_target = 13;
    println!("Number {fibonacci_target} of the Fibonacci Sequence is {}.\n",
        common_concepts_practice::find_nth_fibonacci(fibonacci_target));
    
    common_concepts_practice::print_twelve_days_of_christmas();

    let unsorted_ints_even_length: Vec<i32> = vec![6, 3, 2, 7, 8, 9, 5, 10, 4, 1];
    let median_even = common_collections_practice::vector_median(unsorted_ints_even_length);
    println!("The median of the even array {}.", median_even);

    let unsorted_ints_odd_length: Vec<i32> = vec![6, 3, 2, 7, 8, 9, 5, 10, 4, 1, 0];
    let median_odd = common_collections_practice::vector_median(unsorted_ints_odd_length);
    println!("The median of the odd array is {}.", median_odd);
}