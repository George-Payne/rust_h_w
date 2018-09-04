use std::collections::HashMap;
use std::thread;
use std::time::Duration;

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);
}

// fn simulated_expensive_calculation(intensity: u32) -> u32 {
//     println!("calculating slowly...");
//     thread::sleep(Duration::from_secs(2));
//     intensity
// }

struct Cacher<T, R>
where
    T: Fn(R) -> R,
    R: std::cmp::Eq,
    R: std::hash::Hash,
    R: Copy,
{
    calculation: T,
    value: HashMap<R, R>,
}

impl<T, R> Cacher<T, R>
where
    T: Fn(R) -> R,
    R: std::cmp::Eq,
    R: std::hash::Hash,
    R: Copy,
{
    fn new(calculation: T) -> Cacher<T, R> {
        Cacher {
            calculation,
            value: HashMap::new(),
        }
    }

    fn value(&mut self, arg: R) -> R {
        if !self.value.contains_key(&arg) {
            self.value.insert(arg, (self.calculation)(arg));
        }

        match self.value.get(&arg) {
            Some(v) => *v,
            None => {
                panic!("impossiburu!");
            }
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn call_with_different_values() {
        let mut c = Cacher::new(|a| a);

        let _v1 = c.value(1);
        let v2 = c.value(2);

        assert_eq!(v2, 2);
    }

    #[test]
    fn call_with_different_type() {
        let mut c = Cacher::new(|a| a);

        let _v1 = c.value("hello");
        let v2 = c.value("hi");

        assert_eq!(v2, "hi");
    }
}
