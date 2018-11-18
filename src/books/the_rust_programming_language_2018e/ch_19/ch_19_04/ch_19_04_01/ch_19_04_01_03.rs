/*
  Advanced Features
      Advanced Types
          Dynamically Sized Types and the Sized Trait
*/

pub fn fn_19_04_01_03() {
    println!("-------------------------------------------------- 01");
    {
        /*
          DST（Dynamic Sized Types）是指在 runtime 时期才能知道大小的类型；
          因为 rust 必须在 compile 时期知道所有类型的大小，
          因此要使用 DST，必须使用某种 pointer 来包装一下 DST 再使用；
        
          如 str 是一种 DST，而 &str 是一个 string slice，
          string slice 中不仅包含数据的地址，还包含数据的长度；
        
          trait 也是一种 DST，因此在使用 trait object 的时候，
          必须使用 &dyn Trait 或者 Box<dyn Trait> 的形式；
        */

        /*
          rust 定义了一个特殊的 trait：Sized，用来判断某个类型是否是 DST，
          即如果某个类型实现了 Sized，则它不是 DST；
          所有在 compile 时期大小可知的类型，都会自动实现这个 trait；
        */

        /*
          rust 为所有 generic function 隐式地添加了 Sized 这个 trait bound，
          即默认情况下，generic function 不同使用 DST；
        */

        /*
          generic function 隐式地添加了 trait bounds：Sized
        */
        fn fn_01<T>(t: T) {}
        fn fn_02<T: Sized>(t: T) {}

        /*
          如果为 generic function 显式声明 ?Sized，
          则表示该类型可以是 DST，也可以不是 DST；
          此时，因为该类型可能是 DST，因此必须使用指针的形式，比如 reference；
        */

        fn fn_03<T: ?Sized>(t: &T) {}
    }
}
