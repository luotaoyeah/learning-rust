/*
   Common Collections
       Hash Maps
           HashMap and OwnerShip

 */

use std::collections::HashMap;

pub fn fn_08_03_02_01() {
    println!("-------------------------------------------------- 01");
    {
        /*
           对于实现了 Copy 特性（trait）的类型（如：i32），当被放入 hashmap 中时，会发生复制；
           否则（如：String），当被放入 hashmap 中时，会发生 move，hashmap 会成为它的 owner；
         */

        let key: String = String::from("foo");
        let value: String = String::from("bar");
        let mut map: HashMap<String, String> = HashMap::new();
        map.insert(key, value);

        println!("{}", key); // [E0382]: use of moved value: `key`
    }
}
