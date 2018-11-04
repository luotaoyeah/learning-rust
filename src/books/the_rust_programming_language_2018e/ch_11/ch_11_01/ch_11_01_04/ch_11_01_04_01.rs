/*
   Writing Automated Tests
       How to Write Tests
           Adding Custom Failure Messages
 */

pub fn fn_11_01_04_01() {
    println!("-------------------------------------------------- 01");
    {}
}

fn greeting(name: &str) -> String {
    format!("HELLO {}", name)
}

#[test]
fn without_msg() {
    let name = greeting("tom");
    assert!(name.contains("tom"));
}

#[test]
fn with_msg() {
    let name = greeting("tom");
    assert!(name.contains("cat"), "DID NOT CONTAINS NAME: `{}`", name);
}
