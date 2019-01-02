/*
   Enums and Pattern Matching
       Enum Values
 */

pub fn fn_06_01_02_03() {
    println!("-------------------------------------------------- 01");
    {
        /*
            enum 的每个 variant 可以分别定义类型，并在实例化的时候设置值；
         */

        #[derive(Debug)]
        enum IpAddrKind {
            V4(String),
            V6(String),
        }

        let v4 = IpAddrKind::V4(String::from("127.0.0.1"));
        let v6 = IpAddrKind::V6(String::from("::1"));

        println!("{:?}", v4); // V4("127.0.0.1")
        println!("{:?}", v6); // V6("::1")
    }

    println!("-------------------------------------------------- 02");
    {
        /*
           各个 variants 可以定义不同的类型；
         */

        #[derive(Debug)]
        enum IpAddrKind {
            V4(u8, u8, u8, u8),
            V6(String),
        }

        let v4 = IpAddrKind::V4(127, 0, 0, 1);
        let v6 = IpAddrKind::V6(String::from("::1"));

        println!("{:?}", v4); // V4(127, 0, 0, 1)
        println!("{:?}", v6); // V6("::1")
    }
}
