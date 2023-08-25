// Convert temperatures between Fahrenheit and Celsius.
pub fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (5.0/9.0) * (fahrenheit - 32.0)
}

pub fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    (9.0/5.0 * celsius)+ 32.0
}
pub fn temperature_example() {
    let fahrenheit = 72.0;
    let celsius = 32.0;
    println!("{fahrenheit} degrees fahrenheit is ~{:.1} degrees celsius.\n", 
        fahrenheit_to_celsius(fahrenheit));
    
    println!("{celsius} degrees celsius is ~{:.1} degrees fahrenheit.\n", 
        celsius_to_fahrenheit(celsius));
    
    let fibonacci_target = 13;
    println!("Number {fibonacci_target} of the Fibonacci Sequence is {}.\n",
        find_nth_fibonacci(fibonacci_target));
    
}

// Generate the nth Fibonacci number.
pub fn find_nth_fibonacci(target: i32) -> i32 {
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
pub fn print_twelve_days_of_christmas() {
    println!("The Twelve Days of Christmas:\n");

    let lyrics: [[&str; 2]; 12] = [
        ["first", "partridge in a pear tree 🌳\n"],
        ["second", "Two turtle doves 🕊\n"],
        ["third", "Three French hens 🐓\n"],
        ["fourth", "Four calling birds 🐦\n"],
        ["fifth", "Five goldenen rings 💍\n"],
        ["sixth", "Six geese a-laying 🦢\n"],
        ["seventh", "Seven swans a-swimming 🦢\n"],
        ["eighth", "Eight maids a-milking 🥛\n"],
        ["ninth", "Nine ladies dancing 💃\n"],
        ["tenth", "Ten lords a-leaping 🕺\n"],
        ["eleventh", "Eleven pipers piping 🎺\n"],
        ["twelfth", "Twelve drummers drumming 🥁\n"],
    ];

    let mut accumulator: String = "".to_owned();
    
    for (i, lyric) in lyrics.iter().enumerate() {
        if i == 0 {
            println!("On the {} day of christmas\nMy true love gave to me\n{}{}",
                lyric[0], "A ", lyric[1]);
            
            accumulator.insert_str(0, &format!("{}{}", "And a ", lyric[1]));
            continue
        }
        
        accumulator.insert_str(0, lyric[1]);
        
        println!("On the {} day of christmas\nMy true love gave to me\n{}",
            lyric[0], accumulator);
    };
}