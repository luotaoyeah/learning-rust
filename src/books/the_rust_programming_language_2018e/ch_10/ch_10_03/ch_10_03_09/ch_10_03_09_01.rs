/*
   Generic Types, Traits，Lifetimes
       Validating References with Lifetimes
           Generic Type Parameters, Trait Bounds, and Lifetimes Together
 */

use std::fmt::Display;

pub fn fn_10_03_09_01() {
    println!("-------------------------------------------------- 01");
    {
        println!(
            "{}",
            longest_with_announcement("hello", "rust", "ANNOUNCEMENT")
        );
    }
}

fn longest_with_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("{}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
