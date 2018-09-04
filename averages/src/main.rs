use std::collections::HashMap;
use std::env;

enum Average {
    Unknown,
    Mean,
    Median,
    Mode,
}

fn main() {
    let mut numbers: Vec<f32> = Vec::new();
    let mut kind: Average = Average::Unknown;

    for argument in env::args() {
        match argument.parse::<f32>() {
            Ok(i) => numbers.push(i),
            Err(_) => {}
        }

        if argument.starts_with("--") {
            match &argument[..] {
                "--mean" => kind = Average::Mean,
                "--median" => kind = Average::Median,
                "--mode" => kind = Average::Mode,
                _ => println!("Unknown argument {}", argument),
            }
        }
    }

    match kind {
        Average::Unknown => handle_unknown(),
        Average::Mean => handle_mean(numbers),
        Average::Median => handle_median(numbers),
        Average::Mode => handle_mode(numbers),
    }
}

fn handle_unknown() {
    println!("Please specify --mean --median or --mode")
}

fn handle_mean(numbers: Vec<f32>) {
    println!("You want the mean of {:?}", numbers);

    let len = numbers.len() as f32;
    let total = numbers.iter().fold(0.0, |acc, v| acc + v);
    let mean = total / len;

    println!("The mean is {}", mean);
}

fn handle_median(mut numbers: Vec<f32>) {
    println!("You want the median of {:?}", numbers);

    numbers.sort_unstable_by(|a, b| a.partial_cmp(&b).unwrap());
    let mid = numbers.len() / 2;
    let median = numbers.get(mid).unwrap();

    println!("The median is {}", median);
}

fn handle_mode(numbers: Vec<f32>) {
    println!("You want the mode of {:?}", numbers);

    let occurances = numbers.iter().fold(HashMap::new(), |mut acc, v| {
        let count = *acc.entry(v.to_string()).or_insert(0);
        acc.insert(v.to_string(), count + 1);
        acc
    });

    let mut occurances: Vec<(_, _)> = occurances.iter().collect();
    occurances.sort_by(|a, b| b.1.cmp(a.1));

    println!("The mode is {}", occurances[0].0)
}
