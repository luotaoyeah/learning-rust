/*
  Fearless Concurrency
      Shared-State Concurrency
          Using Mutexes to Allow Access to Data from One Thread at a Time
              The API of Mutex<T>
*/

use std::sync::Mutex;

pub fn fn_16_02_01_04() {
    println!("-------------------------------------------------- 01");
    {
        /*
          mutex（mutual exclusion）同一时间只允许一个线程访问数据；
          通过 lock 来记录当前可以访问数据的线程；
        */

        let mutex = Mutex::new(5);
        {
            /*
              通过 lock() 方法获取 lock，返回的是一个 MutexGuard 对象，
              MutexGuard 实现了 Deref 和 Drop 特性，
              因此在离开 scope 时，lock 会自动释放；
            */
            let mut guard = mutex.lock().unwrap();
            *guard = 6;
        }

        println!("{:?}", mutex); // Mutex { data: 6 }
    }
}
