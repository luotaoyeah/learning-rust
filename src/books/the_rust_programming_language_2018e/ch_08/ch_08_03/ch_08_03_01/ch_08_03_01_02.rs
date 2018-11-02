/*
   Common Collections
       Hash Maps
           Creating a New HashMap

 */

#![allow(unused_variables)]

use std::collections::HashMap;

pub fn fn_08_03_01_02() {
    println!("-------------------------------------------------- 01");
    {
        let vec01: Vec<String> = vec![String::from("Blue"), String::from("Yellow")];
        let vec02: Vec<i32> = vec![10, 50];

        /*
           使用 collect() 方法构造一个 hashmap；
           因为 collect() 方法可以构造多种数据结构，所以这儿需要显式声明类型 HashMap，
           因为 HashMap 的两个泛型参数必须提供，但是这儿我们希望由编译器推断类型，
               因此使用两个占位符 _
         */
        let map: HashMap<_, _> = vec01.iter().zip(vec02.iter()).collect();
        println!("{:?}", map); // {"Blue": 10, "Yellow": 50}
    }
}
