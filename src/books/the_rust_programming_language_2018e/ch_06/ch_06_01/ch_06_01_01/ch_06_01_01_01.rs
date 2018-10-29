/*
   Enums and Pattern Matching
       Define an Enum
 */

pub fn fn_06_01_01() {
    println!("-------------------------------------------------- 01");
    {
        /*
           enum 用来描述一组确定的值；
         */

        #[derive(Debug)]
        enum IpAddrKind {
            V4,
            V6,
        }

        println!("{:?}", IpAddrKind::V4); // V4
    }
}
