use std::io;
use std::num;

fn main() {
    loop {
        let a = match input_num("Seed a") {
            Ok(num) => num,
            Err(err) => {
                println!("oops: {}", err);
                continue;
            }
        };

        let b = match input_num("Seed b") {
            Ok(num) => num,
            Err(err) => {
                println!("oops: {}", err);
                continue;
            }
        };

        let n = match input_num("What nth would you enjoy") {
            Ok(num) => num,
            Err(err) => {
                println!("oops: {}", err);
                continue;
            }
        };

        let res = find_n(a, b, n);

        print_result(n, res);
        break;
    }
}

fn input_num(req: &str) -> Result<u32, num::ParseIntError> {
    println!("{}", req);

    let mut n = String::new();
    io::stdin()
        .read_line(&mut n)
        .expect("failed to read line :(");

    n.trim().parse()
}

fn find_n(a: u32, b: u32, n: u32) -> u32 {
    if n == 0 {
        return a;
    }

    println!("{}", a);

    find_n(b, a + b, n - 1)
}

fn print_result(n: u32, res: u32) {
    println!("The {}th number is {}", n, res);
}
