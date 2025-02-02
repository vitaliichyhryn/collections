use std::env;

fn median(v: &Vec<i32>) -> f64 {
    let mut sorted = v.clone();
    sorted.sort();
    let middle = sorted.len() / 2;
    if sorted.len() % 2 == 0 {
        (sorted[middle] + sorted[middle - 1]) as f64 / 2f64
    } else {
        sorted[middle] as f64
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let vec = utils::args_to_int_vec(&args);
    let median = median(&vec);
    println!("Median is {median}.")
}
