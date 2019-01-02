/*
  Functional Language Features: Iterators and Closures
      Processing a Series of Items with Iterators
          The Iterator Trait and the next Method
*/

pub fn fn_13_02_02_01() {
    println!("-------------------------------------------------- 01");
    {
        /*
          标准库定义了一个 trait：Iterator，所有迭代器都实现了这个 trait；
        */

        let vec = vec!["a", "b", "c"];

        /*
          调用 next() 方法时，需要将迭代器设为 mutable，
          因为 next() 方法会改变迭代器的内部状态；
        */
        let mut iter = vec.iter();

        /*
          调用 next() 方法获取下一个元素；如果遍历结束，则返回 None；
        */
        println!("{:?}", iter.next()); // Some("a")
        println!("{:?}", iter.next()); // Some("b")
        println!("{:?}", iter.next()); // Some("c")
        println!("{:?}", iter.next()); // None
    }

    println!("-------------------------------------------------- 02");
    {
        /*
          如果需要返回元素的 immutable reference，使用 iter() 方法，
          如果需要返回元素的 mutable reference，使用 iter_mut() 方法
          如果需要返回元素的 ownership，使用 into_iter() 方法；
        */
        let mut vec = vec!["a", "b", "c"];

        let iter1 = vec.iter();
        let iter2 = vec.into_iter();
        let iter3 = vec.iter_mut();
    }
}
