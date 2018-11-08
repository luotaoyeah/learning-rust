/*
  Functional Language Features: Iterators and Closures
      Processing a Series of Items with Iterators
          Methods that Consume the Iterator
*/

pub fn fn_13_02_03_01() {
    println!("-------------------------------------------------- 01");
    {
        /*
          Iterator 里面定义了很多方法，都有默认实现，
          其中有些方法会调用 next() 方法，这些方法称之为 consuming adaptor，
          他们会获取迭代器的 ownership；
        */
        let vec = vec![1, 2, 3];
        let mut iter = vec.iter();
        let sum: i32 = iter.sum();
        println!("{}", sum);

        println!("{:?}", iter.next()); // [E0382]: use of moved value: `iter`
    }
}
