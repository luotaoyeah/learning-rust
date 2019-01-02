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
        ::ch_01_printing_on_the_terminal
        ::ch_01_04_printing_several_lines_of_text
        ::ch_01_04_01
        ::fn_01_04_01();

    /*
        the_rust_programming_language
            ::ch_03_common_programming_concepts
            ::ch_03_02_data_types
            ::ch_03_02_02_compound_types
            ::ch_03_02_02_02_array_type
            ::ch_03_02_02_02_04
            ::fn_03_02_02_02_04()
    */
}
