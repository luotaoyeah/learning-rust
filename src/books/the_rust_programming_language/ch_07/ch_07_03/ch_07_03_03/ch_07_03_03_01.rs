/*
   Modules
       Referring to Names in Different Modules
           Bring All Names into Scope with a Glob

 */

/*
   use 语句中可以使用 glob operator（*），
   一次性将该 module 下面所有可访问的成员引入进来；
 */
use self::TrafficLignt::*;

#[derive(Debug)]
enum TrafficLignt {
    Yellow,
    Red,
    Green,
}

pub fn fn_07_03_03_01() {
    println!("-------------------------------------------------- 01");
    {
        println!("{:?}", Yellow);
        println!("{:?}", Red);
        println!("{:?}", Green);
    }
}
