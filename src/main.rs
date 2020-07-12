use lazy_static::lazy_static;
use std::collections::HashMap;
use std::env;
use std::process;

lazy_static! {
    // https://en.wikipedia.org/wiki/NATO_phonetic_alphabet
    // Lima is replaced with London
    static ref PHONETICS_ALPHABETS: HashMap<char, &'static str> = {
        let phonetic_alphabets = [
            "Alfa", "Bravo", "Charlie", "Delta", "Echo", "Foxtrot", "Golf", "Hotel", "India",
            "Juliet", "Kilo", "London", "Mike", "November", "Oscar", "Papa", "Quebec", "Romeo",
            "Sierra", "Tango", "Uniform", "Victor", "Whiskey", "X-ray", "Yankee", "Zulu",
        ];
        (b'A'..=b'Z')
            .map(char::from)
            .zip(phonetic_alphabets.iter().cloned())
            .collect()
    };
}

fn phonetic_alphabet_of(text: &String) -> Vec<Option<&'static str>> {
    text.chars()
        .map(|c| c.to_ascii_uppercase())
        .map(|c| PHONETICS_ALPHABETS.get(&c).map(|a| *a))
        .collect()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let text = args.get(1).unwrap_or_else(|| {
        eprintln!("Require 1 string argument. e.g. 'Hello world!'");
        process::exit(1);
    });

    let phonetics = phonetic_alphabet_of(&text);
    let characters = text.chars().zip(phonetics);
    for character in characters {
        let code = character.1.unwrap_or("");
        println!("{} -> {}", character.0, code);
    }
}