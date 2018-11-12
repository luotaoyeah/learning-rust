/*
  Object Oriented Programming Features of Rust
      Using Trait Objects that Allow for Values of Different Types
          Trait Objects Perform Dynamic Dispatch
          Object Safety Is Required for Trait Objects
*/

pub fn fn_17_02_01_03() {
    println!("-------------------------------------------------- 01");
    {
        /*
          trait objects 中的 trait 必须是 object-safe 的，即必须满足两个条件：
              1. trait 中所有的方法都不能有 generic type parameter
              2. trait 中所有的方法都不能返回 Self
        
          Self 表示当前调用该方法的实际类型；
        */
    }

    /*
      Clone 不是一个 object-safe 的 trait，
      因为 clone() 方法返回了 Self；
    */
    trait Clone {
        fn clone(&self) -> Self;
    }

    struct Screen {
        components: Vec<Box<dyn Clone>>, // [E0038]: the trait `Clone` cannot be made into an object
    }
}
