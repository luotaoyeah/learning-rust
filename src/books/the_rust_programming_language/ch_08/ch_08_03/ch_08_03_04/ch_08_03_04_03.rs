/*
   Common Collections
       Hash Maps
           Updating a HashMap
               Updating a Value Based on the Old Value
 */

use std::collections::HashMap;

pub fn fn_08_03_04_03() {
    println!("-------------------------------------------------- 01");
    {
        let text: &str = "hello world wonderful world";
        let mut map: HashMap<&str, i32> = HashMap::new();

        for word in text.split_whitespace() {
            let count = map.entry(word).or_insert(0);
            *count += 1;
        }

        println!("{:?}", map); // {"hello": 1, "wonderful": 1, "world": 2}
    }
}
