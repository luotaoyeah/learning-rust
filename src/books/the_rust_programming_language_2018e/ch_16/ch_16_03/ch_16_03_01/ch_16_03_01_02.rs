/*
  Fearless Concurrency
      Shared-State Concurrency
          Using Mutexes to Allow Access to Data from One Thread at a Time
              Sharing a Mutex<T> Between Multiple Threads
              Multiple Ownership with Multiple Threads
              Atomic Reference Counting with Arc<T>
*/

use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

pub fn fn_16_03_01_02() {
    println!("-------------------------------------------------- 01");
    {
        /*
          Arc（Atomic Reference Counting）原子引用计数，跟 Rc（Reference Counting）的区别是：
          它是线程安全的，但是有一定的性能开销；
        */
        let counter = Arc::new(Mutex::new(0));

        let mut handles = vec![];

        for i in 0..10 {
            let counter = Arc::clone(&counter);
            let handle = thread::spawn(move || {
                let mut guard = counter.lock().unwrap();
                *guard += 1;
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        println!("{:?}", counter); // Mutex { data: 10 }
    }
}
