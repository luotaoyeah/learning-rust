#![feature(core_intrinsics)]

use crate::books::beginning_rust;
use crate::books::the_rust_programming_language;

mod books;

/// 获取值的类型
fn type_of<T>(_: &T) {
    println!("{}", unsafe { std::intrinsics::type_name::<T>() });
}

fn main() {
        beginning_rust
            ::ch_02_doing_arithmetic
            ::ch_02_03_floating_point_arithmetic
            ::ch_02_03_01
            ::fn_02_03_01();

/*
    the_rust_programming_language
            ::ch_03_common_programming_concepts
            ::ch_03_05_control_flow
            ::ch_03_05_02_repetition_with_loops
            ::ch_03_05_02_01
            ::fn_03_05_02_01()
*/
}
