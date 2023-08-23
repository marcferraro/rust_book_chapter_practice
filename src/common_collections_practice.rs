// Given a list of integers, use a vector and return the 
// median (when sorted, the value in the middle position) and 
// mode (the value that occurs most often; a hash map will be helpful here) of the list.

pub fn vector_median(mut v: Vec<i32>) -> f64 {
    v.sort_unstable();
    let length = v.len();
    
    if let 0 = length % 2 {
        (v[(length / 2) - 1] as f64 + v[length / 2] as f64) as f64 / 2.0
    } else {
        (v[(length - 1) / 2]) as f64
    }


    // sort array by value
    // check length of array
        // if even, average middle two numbers
        // if odd, reutrn middle num
}

pub fn vector_mode() -> i32 {
    todo!()
}