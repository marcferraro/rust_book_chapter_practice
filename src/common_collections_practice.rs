// Given a list of integers, use a vector and return the 
// median (when sorted, the value in the middle position) and 
// mode (the value that occurs most often; a hash map will be helpful here) of the list.

use std::collections::HashMap;

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