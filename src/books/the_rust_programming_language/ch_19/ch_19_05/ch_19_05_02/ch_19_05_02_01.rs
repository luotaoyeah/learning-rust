/*
  Advanced Features
      Advanced Functions and Closures
          Returning Closures
*/

pub fn fn_19_05_02_01() {
    println!("-------------------------------------------------- 01");
    {
        /*
          函数的返回值不能直接声明为一个 trait，
          因此，函数不能直接返回一个 closure，因为 closure 的类型是使用 trait 来标识的；
          此时，可以通过返回 trait objects 的形式，来返回一个 closure；
        */

        fn fn_01() -> Box<dyn Fn(i32) -> i32> {
            Box::new(|x| x + 1)
        }
    }
}
