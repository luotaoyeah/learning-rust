/*
   Understanding Ownership
       References and Borrowing
           Mutable References
 */

pub fn fn_04_02_02_02() {
    println!("-------------------------------------------------- 01");
    {
        /*
            在某个 scope 中，对某个 variable 的 mutable reference 最多只能有一个；
            这种限制的好处是，在 compile 时期就能够阻止资源竞争；
         */
        let mut s01: String = String::from("hello");
        let r01 = &mut s01;
        let r02 = &mut s01; // [E0499]: cannot borrow `s01` as mutable more than once at a time
        println!("{} {}", r01, r02);
    }

    println!("-------------------------------------------------- 02");
    {
        /*
           通过花括号创建新的 scope，在新的 scope 中可以创建另外的 mutable inference；
         */

        let mut s01: String = String::from("hello");
        {
            let r01 = &mut s01;
            println!("{}", r01);
        }
        let r02 = &mut s01;
        println!("{}", r02);
    }
}
