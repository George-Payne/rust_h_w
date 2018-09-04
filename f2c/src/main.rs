use std::io;
use std::num;

fn main() {
    println!("Enter your temp in c or f");

    loop {
        let mut inp = String::new();

        io::stdin()
            .read_line(&mut inp)
            .expect("failed to read line :(");

        let (value, unit) = match cleave_last_char(&inp) {
            (Ok(num), Some('f')) => (convert_to_c(num), 'c'),
            (Ok(num), Some('c')) => (convert_to_f(num), 'f'),
            _ => continue,
        };

        println!("value: {}°{}", value, unit);

        break;
    }
}

fn cleave_last_char(a: &String) -> (Result<f32, num::ParseFloatError>, Option<char>) {
    let a = a.trim();
    let number: String = a.chars().take(a.len() - 1).collect();
    let number = number.parse();
    let unit = a.chars().last();
    (number, unit)
}

fn convert_to_c(f: f32) -> f32 {
    (f - 32.0) / 1.8
}

fn convert_to_f(f: f32) -> f32 {
    (f * 1.8) + 32.0
}
