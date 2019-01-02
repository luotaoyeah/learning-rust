/*
  Fearless Concurrency
      Using Message Passing to Transfer Data Between Threads
          Creating Multiple Producers by Cloning the Transmitter
*/

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub fn fn_16_02_01_04() {
    println!("-------------------------------------------------- 01");
    {
        /*
          mpsc（multiple producer single consumer）可以允许多个 transmitter；
        */

        let (tx, rx) = mpsc::channel();

        /*
          使用 mpsc::Sender::clone() 方法复制一个新的 transmitter；
        */
        let tx02 = mpsc::Sender::clone(&tx);

        thread::spawn(move || {
            let msgs = vec![
                String::from("A"),
                String::from("B"),
                String::from("C"),
                String::from("D"),
            ];

            for m in msgs {
                tx.send(m).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        });

        thread::spawn(move || {
            let msgs = vec![
                String::from("1"),
                String::from("2"),
                String::from("3"),
                String::from("4"),
            ];

            for m in msgs {
                tx02.send(m).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        });

        for r in rx {
            println!("{}", r);
        }
    }
}
