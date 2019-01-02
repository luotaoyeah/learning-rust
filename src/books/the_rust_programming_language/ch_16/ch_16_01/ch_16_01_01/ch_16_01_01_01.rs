/*
  Fearless Concurrency
      Using Threads to Run Code Simultaneously
          Creating a New Thread with spawn

*/

use std::thread;
use std::time::Duration;

pub fn fn_16_01_01_01() {
    println!("-------------------------------------------------- 01");
    {
        /*
          rust 的多线程使用的是 1:1 模式，即一个应用程序线程对应一个操作系统线程；
        */

        /*
          使用 thread::spawn() 创建一个新的线程，
          这种方式的缺陷：当主线程结束时，该线程也会结束，无论它是否执行完毕；
        */
        thread::spawn(|| {
            for i in 1..10 {
                println!("---------- spawn thread: {}", i);
                thread::sleep(Duration::from_millis(1));
            }
        });

        for i in 1..5 {
            println!("main thread: {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    }
}
