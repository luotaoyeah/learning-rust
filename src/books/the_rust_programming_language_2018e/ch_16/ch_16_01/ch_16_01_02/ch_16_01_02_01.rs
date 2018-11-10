/*
  Fearless Concurrency
      Using Threads to Run Code Simultaneously
          Using move Closures with Threads

*/

use std::thread;
use std::time::Duration;

pub fn fn_16_01_02_01() {
    println!("-------------------------------------------------- 01");
    {
        let v = vec![1, 2, 3];

        /*
          因为 rust 会推断 closure 捕获环境变量的方式，
          在这里，因为只是通过 println! 打印 v 的值，因此被推断为 Fn 类型的 closure，
          即对 v 是 borrowing；
        
          而新的线程可能会比 v 活得久，因此会引发错误，所以此处编译失败；
        */
        let handle = thread::spawn(|| {
            println!("{:?}", v);
        });

        handle.join().unwrap();
    }
}
