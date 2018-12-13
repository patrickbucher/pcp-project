use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();
    let mut wasted_days = 0;

    for s in 0..3 {
        let tx_copy = mpsc::Sender::clone(&tx);
        let _student = thread::spawn(move || {
            for d in 0..5 {
                tx_copy.send(1).unwrap();
                thread::sleep(Duration::from_secs(1));
                println!("student {} wasted day #{}", s + 1, d + 1);
            }
            drop(tx_copy);
        });
    }
    drop(tx);
    for received in rx {
        wasted_days += received;
    }
    println!("totally {} days wasted", wasted_days);
}
