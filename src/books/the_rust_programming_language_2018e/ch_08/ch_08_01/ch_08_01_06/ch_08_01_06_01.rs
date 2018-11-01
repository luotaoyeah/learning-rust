/*
   Common Collections
       Vectors
           Using an Enum to Store Multiple Types

 */

#![allow(unused_variables)]
pub fn fn_08_01_06_01() {
    println!("-------------------------------------------------- 01");
    {
        /*
           因为 Vec<T> 的元素类型必须相同，
           而 enum 的元素可以关联不同类型的数据，但是 enum 的每一个 variant 都是相同的类型，
           所以可以使用 Vec<T> 来存储 enum 的 variants；
         */

        #[derive(Debug)]
        enum Column {
            Int(i32),
            Float(f64),
            Text(String),
        }

        let vec: Vec<Column> = vec![
            Column::Int(9),
            Column::Float(3.14),
            Column::Text(String::from("foo")),
        ];

        println!("{:?}", vec); // [Int(9), Float(3.14), Text("foo")]
    }
}
