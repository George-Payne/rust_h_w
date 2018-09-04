use std::env;

fn main() {
    let mut sentence: Vec<String> = Vec::new();

    for argument in env::args() {
        match argument.chars().next().unwrap() {
            'a' | 'e' | 'i' | 'o' | 'u' => sentence.push(handle_vowel(argument)),
            _ => sentence.push(handle_consonant(argument)),
        }
        sentence.push(" ".to_string());
    }

    let pig_speak: String = sentence.into_iter().collect();
    println!("{}", pig_speak);
}

fn handle_vowel(mut word: String) -> String {
    word.push_str(&"-hay");
    word
}

fn handle_consonant(word: String) -> String {
    let mut pig: Vec<char> = Vec::new();
    pig.extend(word[1..].chars());
    pig.push('-');
    pig.extend(word[..1].chars());
    pig.push('a');
    pig.push('y');
    pig.into_iter().collect()
}
