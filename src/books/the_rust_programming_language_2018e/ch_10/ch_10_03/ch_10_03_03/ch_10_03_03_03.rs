/*
   Generic Types, Traits，Lifetimes
       Validating References with Lifetimes
           Lifetime Annotation Syntax
           Lifetime Annotations in Function Signatures
 */

pub fn fn_10_03_03_01() {
    println!("-------------------------------------------------- 01");
    {
        let str01 = String::from("hello");
        {
            let str02 = String::from("rust");
            let str03 = longest(str01.as_str(), str02.as_str());
            println!("{}", str03);
        }
    }

    println!("-------------------------------------------------- 02");
    {
        let str01 = String::from("hello");
        let str03;
        {
            let str02 = String::from("rust");
            str03 = longest(str01.as_str(), str02.as_str());
        }
        println!("{}", str03); // [E0597]: `str02` does not live long enough
    }
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
