/*
   Common Collections
       Hash Maps
           Accessing Values in a HashMap

 */

use std::collections::HashMap;

pub fn fn_08_03_03_02() {
    println!("-------------------------------------------------- 01");
    {
        /*
           使用 for/in 遍历；
         */

        let mut map: HashMap<String, i32> = HashMap::new();
        map.insert(String::from("foo"), 9);
        map.insert(String::from("bar"), 99);

        for (k, v) in &map {
            println!("k: {}, v: {}", k, v);
        }

        println!("{:?}", map); // {"foo": 9, "bar": 99}
    }
}
