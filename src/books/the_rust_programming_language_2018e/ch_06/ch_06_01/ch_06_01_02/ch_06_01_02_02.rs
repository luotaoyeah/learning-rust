/*
   Enums and Pattern Matching
       Enum Values
           Listing 6-1: Storing the data and IpAddrKind variant of an IP address using a struct
 */

pub fn fn_06_01_02_02() {
    println!("-------------------------------------------------- 01");
    {
        #[derive(Debug)]
        enum IpAddrKind {
            V4,
            V6,
        }

        #[derive(Debug)]
        struct IpAddr {
            kind: IpAddrKind,
            address: String,
        }

        let home = IpAddr {
            kind: IpAddrKind::V4,
            address: String::from("127.0.0.1"),
        };

        let loopback = IpAddr {
            kind: IpAddrKind::V6,
            address: String::from("::1"),
        };

        println!("{:?}", home); // IpAddr { kind: V4, address: "127.0.0.1" }
        println!("{:?}", loopback); // IpAddr { kind: V6, address: "::1" }
    }
}
