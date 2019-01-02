/*
   Common Collections
       Hash Maps
           Updating a HashMap
               Only Inserting a Value If the Key Has No Value
 */

use std::collections::HashMap;

pub fn fn_08_03_04_02() {
    println!("-------------------------------------------------- 01");
    {
        let mut map: HashMap<i32, String> = HashMap::new();
        map.insert(0, String::from("foo"));

        /*
           entry() 方法返回指定 key 在 map 中的 entry，
           or_insert() 方法，返回 entry 对应的 key 指向的 value 的 mutable reference，
               如果对应的 key 不存在，则将新的 value 插入 map，然后返回 value 的 mutable reference；
         */
        map.entry(0).or_insert(String::from("bar"));
        map.entry(1).or_insert(String::from("baz"));

        println!("{:?}", map); // {1: "baz", 0: "foo"}
    }
}
