
use std::thread;
use std::time::Duration;
fn share_data_between_threads_through_channel() {
    // multiple producer, single consumer
    use std::sync::mpsc;

    println!("Concurrency");
    let (sender,receiver) = mpsc::channel(); // notice tuple unpacking

    let thread = {
        std::thread::spawn(move ||{
            let steps = receiver.recv().unwrap();
            for step in 1..=steps {
                std::thread::sleep(std::time::Duration::from_secs(1));
                println!("Thread step {}",step);
            }
            "Goodbye!" // important thread could return values
        })
    };

    println!("Spawned a thread!");
    sender.send(5).unwrap();// if message did not reach reciever, you get err
    std::thread::sleep(std::time::Duration::from_secs(3));
    println!("Main thread slept for 3 seconds");
    let result = thread.join().unwrap();
    println!("Thread returned: {:?}", result);
    
}

fn main() {
    share_data_between_threads_through_channel();
}
