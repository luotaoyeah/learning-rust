/*
   Modules
       Referring to Names in Different Modules
           Bring Names into Scope with the `use` Keyword
               enums are namespaces

 */

/*
   把 enum 作为 namespace 引入；
   通过 {} 同时引入多个成员；
 */
use self::TrafficLight::{Green, Yellow};

#[derive(Debug)]
enum TrafficLight {
    Yellow,
    Red,
    Green,
}

pub fn fn_07_03_02_02() {
    println!("-------------------------------------------------- 01");
    {
        println!("{:?}", Green);
        println!("{:?}", Yellow);
        println!("{:?}", TrafficLight::Red);
    }
}
