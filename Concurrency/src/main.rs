use std::thread::sleep;
use  std::sync::{Arc, Mutex};
const NUM_THREADS: usize = 20; //local machine data type
fn main() {
    // THREADING:
    // let mut threads = vec![];
    // for i in 0..10 {
    //     let th = std::thread::spawn(move || {
    //         sleep(std::time::Duration::from_millis(i * 1000));
    //         println!("new thread {}", i); 
    //     });
    //     threads.push(th);
    // }
    // for t in threads{
    //     t.join();
    // }
    // println!("Main Thread");

    // MSPC: Multiple Producer Single Reveiver (thread communication)
    //1
    // let (tx, rx) = std::sync::mpsc::channel();
    // std::thread::spawn(move || {tx.send(42).unwrap()});
    // println!("recieved {}", rx.recv().unwrap());
    //2
    // let (tx, rx) = std::sync::mpsc::channel();
    // for i in 0..NUM_THREADS{
    //     start_thread(i, tx.clone());
    // } 
    // for j in rx.iter().take(NUM_THREADS){
    //     println!("Recieved {}", j);
    // }

    //MUTEX: Only one threadcan access the data at any one time
    // ARC: atomic ref. counted type, allows converting data to primitive type
    let c = Arc::new(Mutex::new(1));
    let mut threads = vec![];
    for i in 0..10{
        let c = Arc::clone(&c);
        let t = std::thread::spawn(move || {
            let mut num = c.lock().unwrap();
            *num +=1; //deref. num with *
        });
        threads.push(t);
    }
    for th in threads{
        th.join().unwrap();
    }
    println!("Results {}", c.lock().unwrap());

}

// MSPC: Multiple Producer Single Reveiver (thread communication)
fn start_thread(d:usize, tx : std::sync::mpsc::Sender<usize>) {
    std::thread::spawn(move || {
        println!("Setting Time {}", d);
        sleep(std::time::Duration::from_secs(d as u64));
        println!("Sending {}", d);
        tx.send(d).unwrap();
    });
}