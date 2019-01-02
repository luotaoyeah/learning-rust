/*
   Common Collections
       Hash Maps
           Creating a New HashMap

 */

#![allow(unused_variables)]

use std::collections::HashMap;

pub fn fn_08_03_01_01() {
    println!("-------------------------------------------------- 01");
    {
        /*
           hashmap 的类型是 HashMap<K, V>，由标准库定义，
           跟 Vec<T> 的区别在于，
               Vec<T> 的 key 的值是整数，即只能通过 index 访问元素；
               HashMap<K, V> 的 key 的值可以是任意类型；
           HashMap<K, V> 的所有 key 的类型必须相同，所有 value 的类型必须相同；
         */

        // 使用 HashMap::new() 创建一个空的 hashmap
        let mut map: HashMap<String, i32> = HashMap::new();
        // 使用 insert() 方法插入新的元素
        map.insert(String::from("Blue"), 10);
        map.insert(String::from("Yellow"), 50);

        println!("{:?}", map); // {"Yellow": 50, "Blue": 10}
    }
}
