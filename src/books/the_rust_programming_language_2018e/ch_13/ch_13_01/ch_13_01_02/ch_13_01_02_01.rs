/*
  Functional Language Features: Iterators and Closures
      Closures: Anonymous Functions that Can Capture Their Environment
          Closure Type Inference and Annotation
*/

/*
  因为 closure 不会暴露给外界访问，通常只在一个很小的作用范围使用，
  因此不需要声明类型，可以由 compiler 来推断类型；
*/
pub fn fn_13_01_01_01() {
    println!("-------------------------------------------------- 01");
    {
        /*
          closure 可以显式声明类型；
        */

        let closure_01 = |x: u32| -> u32 { x + 1 };
        println!("{}", closure_01(1)); // 2
    }

    println!("-------------------------------------------------- 02");
    {
        /*
          closure 的类型一旦被推断出来，就会被锁定，
          后面调用该 closure 必须满足这个类型；
        */
        let closure_01 = |x| x;

        println!("{}", closure_01(9));
        println!("{}", closure_01("foo")); // [E0308]: mismatched types
    }
}
