/*
  Fearless Concurrency
      Using Message Passing to Transfer Data Between Threads

*/

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub fn fn_16_02_01_01() {
    println!("-------------------------------------------------- 01");
    {
        /*
          使用 mpsc::channel() 创建一个 channel，
          channel 是一个 tuple，包含两个部分：发送者，接收者；
        */
        let (tx, rx) = mpsc::channel();

        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(2000));

            let msg = String::from("HI");

            /*
              使用 send() 方法往 channel 发送一个消息；
            */
            tx.send(msg).unwrap();
        });

        /*
          使用 recv() 方法从 channel 接受一个消息，
          recv() 方法会阻塞当前线程，知道接收到一个消息；
        */
        let msg = rx.recv().unwrap();
        println!("{}", msg);

        handle.join().unwrap();
    }
}
