use std::collections::{HashMap, HashSet};
use std::io::{stdin, stdout, Write};

type EmployeeList = HashMap<String, HashSet<String>>;

fn main() {
    let mut employees: EmployeeList = HashMap::new();

    big_print(&"Employee bot");

    loop {
        let mut s = String::new();
        let _ = stdout().flush();
        stdin()
            .read_line(&mut s)
            .expect("Did not enter a correct string");

        let words: Vec<&str> = s.split_whitespace().collect();

        match words[0] {
            "add" => add_to_list(words[1..].to_vec(), &mut employees),
            "remove" => remove_from_list(words[1..].to_vec(), &mut employees),
            "list" => list_the_list(words[1..].to_vec(), &mut employees),
            _ => println!("What is {}?", words[0]),
        }
    }
}

fn find_name(words: &Vec<&str>) -> String {
    let mut name = Vec::new();
    for word in words {
        match word {
            &"to" | &"from" => return name.join(" "),
            _ => name.push(*word),
        }
    }
    name.join(" ")
}

fn find_department(words: &Vec<&str>) -> String {
    let mut name = Vec::new();
    let mut adding = false;
    for word in words {
        if adding {
            name.push(*word)
        }
        match word {
            &"to" | &"from" => adding = true,
            _ => {}
        }
    }
    name.join(" ")
}

fn add_to_list(words: Vec<&str>, employees: &mut EmployeeList) {
    let name = find_name(&words);
    let department = find_department(&words);
    let dep = employees.entry(department).or_insert(HashSet::new());
    dep.insert(name);
}

fn remove_from_list(words: Vec<&str>, employees: &mut EmployeeList) {
    let name = find_name(&words);
    let department = find_department(&words);
    let dep = employees.entry(department).or_insert(HashSet::new());
    dep.remove(&name);
}

fn list_the_list(words: Vec<&str>, employees: &mut EmployeeList) {
    let department = words.join(" ");
    let dep = employees.entry(department).or_insert(HashSet::new());
    let mut dep: Vec<&String> = dep.iter().collect();
    let mut department_name = words.join(" ");

    if dep.len() == 0 {
        department_name.push_str(" is empty.");
        big_print(&department_name)
    } else {
        dep.sort();
        big_print_list(&department_name, dep);
    }
}

fn big_print(word: &str) {
    let dashes = vec!["-"; word.len()];
    let dashes = dashes.join("");

    println!("+---{}---+", dashes);
    println!("|-- {} --|", word);
    println!("+---{}---+", dashes);
}

fn big_print_list(title: &str, list: Vec<&String>) {
    big_print(title);

    let dashes = vec!["-"; title.len()];
    let dashes = dashes.join("");

    for (i, item) in list.iter().enumerate() {
        println!("{}. {}", i + 1, item);
    }

    println!("+---{}---+", dashes);
}
