/*
   Common Collections
       Hash Maps
           Accessing Values in a HashMap

 */

use std::collections::HashMap;

pub fn fn_08_03_03_01() {
    println!("-------------------------------------------------- 01");
    {
        /*
           使用 get() 方法根据 key 获取 value;
         */

        let mut map: HashMap<String, i32> = HashMap::new();
        map.insert(String::from("foo"), 10);

        /*
           get() 方法返回的是一个 Option<&V> 类型；
         */
        match map.get("foo") {
            None => {
                println!("not found");
            }
            Some(v) => {
                println!("{}", v); // 10
            }
        }

        match map.get("bar") {
            None => {
                println!("not found"); // not found
            }
            Some(v) => {
                println!("{}", v);
            }
        }
    }
}
