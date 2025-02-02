use std::env;

fn is_vowel(c: char) -> bool {
    matches!(c, 'a' | 'e' | 'i' | 'o' | 'u')
}

fn to_pig_latin(s: &str) -> String {
    let mut chars = s.chars();

    match chars.next() {
        Some(first) if is_vowel(first) => format!("{s}hay"),
        Some(first) => format!("{}{}ay", chars.as_str(), first),
        None => String::new(),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let word = &args[1];
    println!("{word} in Pig Latin is {}", to_pig_latin(word))
}
