use std::collections::HashMap;
use std::env;

fn mode(v: &Vec<i32>) -> i32 {
    let mut map = HashMap::new();
    for value in v {
        let count = map.entry(*value).or_insert(0);
        *count += 1;
    }
    let mut mode = v[0];
    for (key, value) in &map {
        let mode_count = map.get(&mode).copied().unwrap_or(0);
        if *value > mode_count {
            mode = *key;
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
