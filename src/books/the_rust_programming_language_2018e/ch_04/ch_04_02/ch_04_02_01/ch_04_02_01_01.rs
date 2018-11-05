/*
   Understanding Ownership
       References and Borrowing
 */

pub fn fn_04_02_01_01() {
    println!("-------------------------------------------------- 01");
    {
        /*
           rust 定义了 reference 的概念，一个 reference 变量保存的是：它所引用的变量在 stack 上的内存地址；
           通过 reference 变量访问 value，不会获取该 value 的 ownership，因此也就不存在 ownership 的 move 操作；
         */

        let s01: String = String::from("hello");

        /*
           在普通变量名称前面加上一个 & 表示获取该变量的 reference，
           该 reference 指向变量 s01 指向的数据，但是并没有获取该数据的 ownership；
         */
        let len = calculate_length(&s01);
        println!("{} {}", s01, len); // hello 5

        // 在声明 reference 类型的变量时，类型前面也要加上一个 &
        fn calculate_length(s: &String) -> usize {
            s.len()
            /*
               reference 变量在离开 scope 的时候，
               它所指向的数据不会被 drop，因为 reference 没有获取 ownership；
             */
        }
    }
}
