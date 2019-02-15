//
// Understanding Ownership
//     Ownership and Functions
//

pub fn fn_04_01_05() {
    println!("-------------------------------------------------- 01");
    {
        // 函数传参跟变量赋值类似，也会发生 move 或者 copy 操作

        let s01: String = String::from("FOO");
        fn_01(s01); // FOO

        /*
                println!("{}", s01); // [E0382]: borrow of moved value: `s01`
        */
    }

    println!("-------------------------------------------------- 02");
    {
        let i01: i32 = 9;
        let i02: i32 = fn_02(i01);
        println!("{}", i01); // 9
        println!("{}", i01 == i02); // true
    }

    println!("-------------------------------------------------- 03");
    {
        let a01: A = A { x: 9 };
        fn_03(a01);
        println!("{:?}", a01);
    }
}

///
fn fn_01(s: String) {
    println!("{}", s);
}

///
fn fn_02(i: i32) -> i32 {
    println!("{}", i);
    i
}

///
fn fn_03(a: A) {
    println!("{:?}", a);
}

#[derive(Debug, Clone, Copy)]
struct A {
    x: i32,
}
