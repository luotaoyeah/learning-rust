/*
  Fearless Concurrency
      Using Threads to Run Code Simultaneously
          Using move Closures with Threads

*/

use std::thread;
use std::time::Duration;

pub fn fn_16_01_02_02() {
    println!("-------------------------------------------------- 01");
    {
        let v = vec![1, 2, 3];

        /*
          可以使用 move closure，显式声明 closure 捕获变量的方式；
        */
        let handle = thread::spawn(move || {
            println!("{:?}", v); // [1, 2, 3]
        });

        handle.join().unwrap();
    }
}
