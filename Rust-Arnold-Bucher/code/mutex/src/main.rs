use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    let wasted_days = Arc::new(Mutex::new(0));
    let mut students = vec![];

    for s in 0..3 {
        let wasted_days = Arc::clone(&wasted_days);
        let student = thread::spawn(move || {
            for d in 0..5 {
                {
                    let mut c = wasted_days.lock().unwrap();
                    *c += 1;
                }
                thread::sleep(Duration::from_secs(1));
                println!("student {} wasted day #{}", s + 1, d + 1);
            }
        });
        students.push(student);
    }
    for student in students {
        student.join().unwrap();
    }
    println!("totally {} days wasted", *wasted_days.lock().unwrap());
}
