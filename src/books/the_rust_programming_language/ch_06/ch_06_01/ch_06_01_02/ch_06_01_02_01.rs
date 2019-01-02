/*
   Enums and Pattern Matching
       Enum Values
 */

pub fn fn_06_01_02_01() {
    #[derive(Debug)]
    enum IpAddrKind {
        V4,
        V6,
    }

    println!("-------------------------------------------------- 01");
    {
        /*
           访问 enum 的 variant 需要使用 :: 的方式；
         */

        println!("{:?}", IpAddrKind::V4); // V4
    }

    println!("-------------------------------------------------- 02");
    {
        /*
           enum 的所有 variants 拥有相同的类型，即所属的 enum；
         */

        fn route(ip_kind: IpAddrKind) {
            println!("{:?}", ip_kind);
        }

        /*
           IpAddrKind::V4 和 IpAddrKind::V6 的类型都是 IpAddrKind；
         */
        route(IpAddrKind::V4); // V4
        route(IpAddrKind::V6); // V6
    }
}
