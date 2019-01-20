#![feature(core_intrinsics)]

use crate::books::beginning_rust;
use crate::books::the_rust_programming_language;

mod books;

/// 获取值的类型
fn type_of<T>(_: &T) {
    println!("{}", unsafe { std::intrinsics::type_name::<T>() });
}

fn main() {
    /*
            beginning_rust
                ::ch_02_doing_arithmetic
                ::ch_02_05_breaking_literal_strings
                ::ch_02_05_03
                ::fn_02_05_03();
    */

    the_rust_programming_language
            ::ch_03_common_programming_concepts
            ::ch_03_05_control_flow
            ::ch_03_05_02_repetition_with_loops
            ::ch_03_05_02_01
            ::fn_03_05_02_01()
}
