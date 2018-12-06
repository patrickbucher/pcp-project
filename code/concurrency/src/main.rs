use std::thread;
use std::time::Duration;
use std::string::String;
use std::io::{stdout, Write};

fn main() {
    let long_lasting_handle: thread::JoinHandle<u64> = thread::spawn(|| {
        let millis: u64 = 3000;
        sleep(millis);
        print!("{}", millis);
        return millis;
    });
    let even_longer_lasting_handle: thread::JoinHandle<u64> = thread::spawn(|| {
        let millis: u64 = 6000;
        sleep(millis);
        print!("{}", millis);
        return millis;
    });
    let last_handle: thread::JoinHandle<String> = thread::spawn(move || {
        let millis: u64 = 2000;
        let many_millis = match long_lasting_handle.join() {
            Ok(millis) => millis,
            Err(error) => {
                panic!("error joining long lasting thread: {:?}", error)
            },
        };
        let even_more_millis = match even_longer_lasting_handle.join() {
            Ok(millis) => millis,
            Err(error) => {
                panic!("error joining even longer lasting thread: {:?}", error)
            },
        };
        sleep(millis);
        return format!("was waiting for {} ms",
                       (many_millis + even_more_millis + millis));
    });
    println!("-> Now waiting for things to happen!");
    for _ in  0..16 {
        sleep(500);
        print!(".");
        stdout().flush().unwrap();
    }
    let last_result = last_handle.join();
    match last_result {
        Ok(message) => print!("{}", message),
        Err(_) => panic!("Oh, noes!"),
    }
    println!("-> Done.");
}

fn sleep(millis: u64) {
    thread::sleep(Duration::from_millis(millis));
}
