/*
  Fearless Concurrency
      Using Message Passing to Transfer Data Between Threads
          Channels and Ownership Transference

*/

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub fn fn_16_02_01_02() {
    println!("-------------------------------------------------- 01");
    {
        /*
           std::sync::mpsc.send() 方法的参数，会发生 move 操作；
        */

        let (tx, rx) = mpsc::channel();

        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(2000));
            let val = String::from("HI");

            tx.send(val).unwrap();
            println!("val: {}", val); // [E0382]: use of moved value: `val`
        });

        let msg = rx.recv().unwrap();
        println!("{}", msg);

        handle.join().unwrap();
    }
}
