use std::collections::HashMap;
use std::env;

fn mode(v: &[i32]) -> i32 {
    let mut frequency = HashMap::new();
    for &value in v {
        let count = frequency.entry(value).or_insert(0);
        *count += 1;
    }

    let mut mode = v[0];
    let mut mode_count = frequency.get(&mode).copied().unwrap();

    for (&value, &count) in &frequency {
        if count > mode_count {
            mode = value;
            mode_count = count;
        }
    }

    mode
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let vec = utils::args_to_int_vec(&args);
    let mode = mode(&vec);
    println!("Mode is {mode}.")
}
