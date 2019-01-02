/*
  Fearless Concurrency
      Using Message Passing to Transfer Data Between Threads
          Sending Multiple Values and Seeing the Receiver Waiting

*/

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub fn fn_16_02_01_03() {
    println!("-------------------------------------------------- 01");
    {
        let (tx, rx) = mpsc::channel();

        let handle = thread::spawn(move || {
            let vec = vec![
                String::from("A"),
                String::from("B"),
                String::from("C"),
                String::from("D"),
            ];

            for v in vec {
                tx.send(v).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        });

        /*
          主线程会一直阻塞等待，直到 channel 关闭；
        */
        for msg in rx {
            println!("{}", msg);
        }

        println!("END");

        handle.join().unwrap();
    }
}
