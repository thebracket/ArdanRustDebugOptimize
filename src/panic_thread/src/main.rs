use std::thread;

fn main() {
    let data = [
        Some(5),
        Some(3),
        None,
        Some(2),
        None,
    ];

    let mut threads = Vec::new();
    for i in 0..data.len() {
        let n = data[i]; // Take ownership of my copy
        let t = thread::spawn(move || {
            println!("Thread {i} is processing {}", n.unwrap());
        });
        threads.push(t);
    }
    threads.drain(0..).for_each(|thread| {
        let _ = thread.join(); // This pattern ignores errors
    });
    println!("Program Complete");
}
