#![feature(core_intrinsics)]

use crate::books::beginning_rust;
use crate::books::the_rust_programming_language;

mod books;

/// 打印类型
fn print_type_of<T>(_: &T) {
    println!("{}", unsafe { std::intrinsics::type_name::<T>() });
}

fn main() {
/*
    beginning_rust
        ::ch_03_naming_objects
        ::ch_03_11_using_the_functions_of_the_standard_library
        ::ch_03_11_02
        ::fn_03_11_02();
*/

    the_rust_programming_language
            ::ch_04_understanding_ownership
            ::ch_04_01_what_is_ownership
            ::ch_04_01_03
            ::fn_04_01_03();
}
