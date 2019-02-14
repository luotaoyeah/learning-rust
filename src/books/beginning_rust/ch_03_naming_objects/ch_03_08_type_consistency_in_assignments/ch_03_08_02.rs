/*
  Naming Objects
      3.8. Type Consistency in Assignments
          type inference
*/

pub fn fn_03_08_02() {
    println!("-------------------------------------------------- 01");
    // 如果声明变量的时候，没有标识类型，也没有设置初始值，那么编译报错
    // 因为变量的类型必须在编译时期明确，要么显式标识类型，要么通过赋值进行推断
    /*
        let n; // [E0282]: type annotations needed
    */

    println!("-------------------------------------------------- 02");
    // 下面的 n01 和 n02 都可以推断出类型
    let n01 = 9;
    let n02 = n01;
    crate::print_type(&n02); // i32
}
