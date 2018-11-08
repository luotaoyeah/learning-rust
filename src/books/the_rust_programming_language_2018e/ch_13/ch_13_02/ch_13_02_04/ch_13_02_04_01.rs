/*
  Functional Language Features: Iterators and Closures
      Processing a Series of Items with Iterators
          Methods that Produce Other Iterators
*/

pub fn fn_13_02_04_01() {
    println!("-------------------------------------------------- 01");
    {
        /*
          Iterator 中，
          调用 next() 方法的方法称之为：consuming adaptor，
          其他的方法称之为：iterator adaptor；
        
          iterator adaptor 的返回值为新的 iterator，
          因此它们可以链式调用；
        
          iterator adaptor 是延迟计算（lazy）的，
          因此在最后需要调用一个 consuming adaptor 才能生效；
        */

        let vec = vec![1, 2, 3];

        // warning: unused `std::iter::Map` that must be used
        // note: iterator adaptors are lazy and do nothing unless consumed
        vec.iter().map(|x| x + 1);

        let vec02 = vec.iter().map(|x| x + 1).collect::<Vec<i32>>();
        assert_eq!(vec02, vec![2, 3, 4]);
    }
}
