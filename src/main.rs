#![feature(core_intrinsics)]
use crate::books::the_rust_programming_language_2018e::ch_03_common_programming_concepts;

mod books;

/// 获取值的类型
fn type_of<T>(_: &T) {
    println!("{}", unsafe { std::intrinsics::type_name::<T>() });
}

fn main() {
    ch_03_common_programming_concepts::ch_03_02_data_types::ch_03_02_01_scalar_types::ch_03_02_01_04_numeric_operations::ch_03_02_01_04_01::fn_03_02_01_04_01();
}
