/*
   Summary
 */

pub fn fn_03_05_04_06() {
    println!("-------------------------------------------------- 01");
    {
        print!("{}, ", fibonacci(1));
        print!("{}, ", fibonacci(2));
        print!("{}, ", fibonacci(3));
        print!("{}, ", fibonacci(4));
        print!("{}, ", fibonacci(5));
        print!("{}, ", fibonacci(6));
        print!("{}, ", fibonacci(7));
        print!("{}, ", fibonacci(8));
        print!("{}, ", fibonacci(9));
    }
}

/*
   斐波那契数列（fibonacci）
 */
fn fibonacci(n: u32) -> u32 {
    if n < 3 {
        1
    } else {
        fibonacci(n - 2) + fibonacci(n - 1)
    }
}
