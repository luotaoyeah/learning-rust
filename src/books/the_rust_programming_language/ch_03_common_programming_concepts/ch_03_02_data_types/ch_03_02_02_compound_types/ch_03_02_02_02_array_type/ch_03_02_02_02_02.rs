/*
  Common Programming Concepts
      Data Types
          Compound Types
              The Array Type
                  vector
*/

pub fn fn_03_02_02_02_02() {
    println!("-------------------------------------------------- 01");
    /*
      vector 跟 array 类似，但是 vector 大小可变
    */
    let mut vec01: Vec<char> = vec!['a', 'b', 'c'];
    vec01.insert(3, 'd');
    println!("{:?}", vec01); // ['a', 'b', 'c', 'd']

    let arr01: [char; 3] = ['a', 'b', 'c'];
    println!("{:?}", arr01); // ['a', 'b', 'c']
}
