// Given a list of integers, use a vector and return the 
// median (when sorted, the value in the middle position) and 
// mode (the value that occurs most often; a hash map will be helpful here) of the list.

use std::collections::HashMap;
use std::io;

pub fn vector_median(v: &mut Vec<i32>) -> f64 {
    v.sort_unstable();
    let length = v.len();
    
    if let 0 = length % 2 {
        (v[(length / 2) - 1] as f64 + v[length / 2] as f64) as f64 / 2.0
    } else {
        (v[(length - 1) / 2]) as f64
    }
}

pub fn vector_mode(v: &Vec<i32>) -> i32 {
    let mut tally = HashMap::new();
    
    // possible to keep track of highest as we go?
    for int in v.iter() {
        let count = tally.entry(*int).or_insert(0);
        *count += 1;
    };

    let mut highest_key: i32 = v[0];
    for (key, value) in &tally {
        if value > &tally.get(&highest_key).copied().unwrap() {
            highest_key = *key;
        }
    }
    highest_key
}

// Convert strings to pig latin. The first consonant of each word is moved to the end of the word
// and “ay” is added, so “first” becomes “irst-fay.” Words that start with a vowel have “hay” added
// to the end instead (“apple” becomes “apple-hay”). Keep in mind the details about UTF-8 encoding!

pub fn pig_latin(s: String) -> String {
    let mut pig_latin = String::new();
    
    for word in s.to_lowercase().split_whitespace() {
        let mut chars = word.chars();

        let first_letter = match chars.next() {
            Some(letter) => letter,
            None => {
                println!("String needs at least one letter.");
                break
            },
        };

        match first_letter {
            'a' | 'e' | 'i' | 'o' | 'u' => {
                pig_latin = pig_latin + &format!("{}{}-hay", first_letter.to_string(), chars.as_str().to_owned()) + " ";
            },
            _ => {
                pig_latin = pig_latin + &format!("{}-{}ay", chars.as_str().to_owned(), first_letter.to_string()) + " ";
            }
        }
    };
    pig_latin
}

//  Using a hash map and vectors, create a text interface to allow a user to add employee names to a department
//  in a company. For example, “Add Sally to Engineering” or “Add Amir to Sales.” Then let the user retrieve a
//  list of all people in a department or all people in the company by department, sorted alphabetically.

pub fn add_employees() {
    let mut departments: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        println!("Add employees to departments? y/n");
        let mut confirm = String::new();
        
        io::stdin()
            .read_line(&mut confirm)
            .expect("Failed to read user input.");

        match confirm.trim() {
            "n" => {
                println!("Have a good day!");
                break
            },
            "y" => println!("Please enter a department:"),
            &_ => {
                println!("Please enter y or n.");
                continue
            },
        }

        let mut department = String::new();

        io::stdin()
            .read_line(&mut department)
            .expect("Failed to read user input.");

        if department.len() == 0 {
            println!("Department must have at least one character.");
            continue
        }

        let mut employee = String::new();

        println!("Please enter an employee name:");

        io::stdin()
            .read_line(&mut employee)
            .expect("Failed to read user input.");

        if employee.len() == 0 {
            println!("Employee must have at least one character.");
            continue
        }

        let dept_vec = departments.entry(String::from(department.trim())).or_insert(vec![]);
        dept_vec.push(String::from(employee.trim()))
    }
    if departments.len() > 0 {println!("{:?}", departments)}
}