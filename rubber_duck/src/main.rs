use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    threader(
        &tx,
        vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ],
    );

    threader(
        &tx,
        vec![
            String::from("also"),
            String::from("hello"),
            String::from("from"),
            String::from("this guy"),
        ],
    );

    for received in rx {
        println!("Got: {}", received);
    }
}

fn threader(tx: &mpsc::Sender<String>, vals: Vec<String>) -> thread::JoinHandle<()> {
    let tx = mpsc::Sender::clone(&tx);
    thread::spawn(move || {
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    })
}
