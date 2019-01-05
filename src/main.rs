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
            ::ch_01_printing_on_the_terminal
            ::ch_01_07_comments
            ::ch_01_07_01
            ::fn_01_07_01();
    */

    the_rust_programming_language
            ::ch_03_common_programming_concepts
            ::ch_03_03_functions
            ::ch_03_03_02_functions_parameters
            ::ch_03_03_02_01
            ::fn_03_03_02_01()
}
