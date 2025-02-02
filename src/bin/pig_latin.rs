use std::env;

fn is_vowel(c: char) -> bool {
    let vowels = "aeiou";
    vowels.contains(c)
}

fn to_pig_latin(s: &str) -> String {
    if is_vowel(s.chars().next().unwrap()) {
        String::new()
            .chars()
            .chain(s.chars())
            .chain("hay".chars())
            .collect()
    } else {
        String::new()
            .chars()
            .chain(s.chars().skip(1))
            .chain(s.chars().next())
            .chain("ay".chars())
            .collect()
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let word = &args[1];
    println!("{word} in Pig Latin is {}", to_pig_latin(word))
}
