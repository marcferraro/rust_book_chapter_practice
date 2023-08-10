fn main() {
    let fahrenheit = 72.0;
    println!("{fahrenheit} degrees fahrenheit is ~{} degrees celsius.", 
        fahrenheit_to_celsius(fahrenheit).floor())
}

fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (5.0/9.0) * (fahrenheit - 32.0)
}