/*
  Smart Pointers
      Using Box<T> to Point to Data on the Heap
          Using a Box<T> to Store Data on the Heap
*/

pub fn fn_15_01_01_01() {
    println!("-------------------------------------------------- 01");
    {
        /*
          pointer（指针）是一种特殊的变量，存的是数据的内存地址，而不是数据本身；
          reference 是一种 pointer，使用 & 标识；
          smart pointer 是一种 pointer，但是比起 reference，它具有其他的特性；
        
          reference 只能 borrow 数据，而 smart pointer 可以 own 数据；
          String 和 Vec<T> 是 smart pointer；
        
          通常使用 struct 来实现 smart pointer，此时 struct 需要实现 Deref 和 Drop 特性；
        */


        let x = Box::new(9);
        println!("{}", x); // 9
    }
}
