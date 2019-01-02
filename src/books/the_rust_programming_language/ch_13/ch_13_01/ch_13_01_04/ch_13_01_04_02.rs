/*
  Functional Language Features: Iterators and Closures
      Closures: Anonymous Functions that Can Capture Their Environment
          Capturing the Environment with Closures
*/

pub fn fn_13_01_04_02() {
    println!("-------------------------------------------------- 01");
    {
        /*
          closure 捕获变量有三种方式，分别对应三个 traits：
              FnOnce：获取 ownership，该 closure 只能被调用一次；
              FnMut：mutable borrowing；
              Fn：immutable borrowing；
        
          当创建 closure 时，rust 根据该 closure 使用捕获变量的方式，
          决定实现哪个 trait；
        */

        let x = vec![1, 2, 3];
        let equal_to_x = |a| a == x;
        println!("x: {:?}", x);
        let y = vec![1, 2, 3];
        println!("{}", equal_to_x(y)); // true
    }

    println!("-------------------------------------------------- 02");
    {
        /*
          可以通过关键字 move 显式声明实现 FnOnce；
        */

        let x = vec![1, 2, 3];
        let equal_to_x = move |a| a == x;
        println!("x: {:?}", x); // [E0382]: use of moved value: `x`
        let y = vec![1, 2, 3];
        println!("{}", equal_to_x(y));
    }
}
