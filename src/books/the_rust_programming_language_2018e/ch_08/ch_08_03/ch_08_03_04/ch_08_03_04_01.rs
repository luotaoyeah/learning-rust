/*
   Common Collections
       Hash Maps
           Updating a HashMap
               overwriting a value

 */

use std::collections::HashMap;

pub fn fn_08_03_04_01() {
    println!("-------------------------------------------------- 01");
    {
        /*
           使用 insert() 方法加入数据，如果 key 已经存在，原来的数据会被覆盖；
         */

        let mut map: HashMap<i32, String> = HashMap::new();
        map.insert(0, String::from("foo"));
        println!("{:?}", map); // {0: "foo"}

        map.insert(0, String::from("bar"));
        println!("{:?}", map); // {0: "bar"}
    }
}
