pub fn args_to_int_vec(args: &Vec<String>) -> Vec<i32> {
    let mut vec = Vec::new();
    for arg in &args[1..] {
        if let Ok(value) = arg.parse() {
            vec.push(value)
        }
    }
    vec
}
