#![feature(core_intrinsics)]

use crate::books::beginning_rust;
use crate::books::the_rust_programming_language;

mod books;

/// 打印类型
fn print_type_of<T>(_: &T) {
    println!("{}", unsafe { std::intrinsics::type_name::<T>() });
}

fn main() {
    beginning_rust
        ::ch_03_naming_objects
        ::ch_03_09_change_of_type_and_of_mutability
        ::ch_03_09_01
        ::fn_03_09_01();

    /*
        the_rust_programming_language
                ::ch_03_common_programming_concepts
                ::ch_03_05_control_flow
                ::ch_03_05_05_looping_through_a_collection_with_for
                ::ch_03_05_05_04
                ::fn_03_05_05_04()
    */
}
