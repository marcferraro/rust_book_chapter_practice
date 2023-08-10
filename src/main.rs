fn main() {
    let fahrenheit = 72.0;
    let celsius = 32.0;
    println!("{fahrenheit} degrees fahrenheit is ~{:.1} degrees celsius.", 
        fahrenheit_to_celsius(fahrenheit));
    
    println!("{celsius} degrees celsius is ~{:.1} degrees fahrenheit.", 
        celsius_to_fahrenheit(celsius));
}

fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (5.0/9.0) * (fahrenheit - 32.0)
}

fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    (9.0/5.0 * celsius)+ 32.0
}