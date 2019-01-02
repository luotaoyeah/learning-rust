/*
  Smart Pointers
      Treating Smart Pointers Like Regular References with the Deref Trait
          How Deref Coercion Interacts with Mutability
*/

use std::ops::Deref;

pub fn fn_15_02_02_04() {
    println!("-------------------------------------------------- 01");
    {
        /*
          Deref 特性描述了 immutable reference，
          对应的，DerefMut 特性描述了 mutabl reference；
        
          rust 会在下面三种情况下进行 deref coercion：
              如果 T: Deref<Target=U>，则由 &T 转换为 &U；
              如果 T: DerefMut<Target=U>，则由 &mut T 转换为 &mut U；
              如果 T：Deref<Target=U>，则由 &mut T 转换为 &U；
        */

    }
}
