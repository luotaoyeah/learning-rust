/*
  Fearless Concurrency
      Using Threads to Run Code Simultaneously
          Waiting for All Threads to Finish Using join Handles

*/

use std::thread;
use std::time::Duration;

pub fn fn_16_01_01_01() {
    println!("-------------------------------------------------- 01");
    {
        let handle = thread::spawn(|| {
            for i in 1..10 {
                println!("---------- {}", i);
                thread::sleep(Duration::from_millis(1));
            }
        });

        for i in 1..5 {
            println!("{}", i);
            thread::sleep(Duration::from_millis(1));
        }

        /*
          调用 join() 方法，会阻塞当前线程的执行或者退出，
          直到调用 join() 方法的线程执行完毕；
        */
        handle.join().unwrap();
    }
}
