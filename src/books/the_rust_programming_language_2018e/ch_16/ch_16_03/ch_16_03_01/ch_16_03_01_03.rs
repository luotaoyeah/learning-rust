/*
  Fearless Concurrency
      Shared-State Concurrency
          Using Mutexes to Allow Access to Data from One Thread at a Time
              Similarities Between RefCell<T>/Rc<T> and Mutex<T>/Arc<T>
*/

use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

pub fn fn_16_03_01_03() {
    println!("-------------------------------------------------- 01");
    {
        /*
          Mutex<T> 类似 RefCell<T>，实现了 interior mutability，
              Rc<T> 存在 recursive reference 的风险，
              Mutex<T> 存在 deadlock 的风险；
          Arc<T> 类似 Rc<T>，实现了 reference counting（share ownership）；
        */
    }
}
