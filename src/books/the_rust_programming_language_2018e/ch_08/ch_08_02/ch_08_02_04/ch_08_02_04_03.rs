/*
   Common Collections
       Strings
           Indexing into Strings
               byte/char/grapheme cluster

 */

#![allow(unused_variables)]

pub fn fn_08_02_04_03() {
    println!("-------------------------------------------------- 01");
    {
        /*
           可以从三个视角来看待 rust 中的 Sting：
               grapheme cluster/letter
               char
               byte
         */
        let string01: String = String::from("नमस्ते");
        println!("{}", string01.len()); // 18

        // 从 grapheme cluster 的视角来看
        // नमस्ते
        println!("{}", string01);
        // 从 byte 的视角来看
        // [224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164, 224, 165, 135]
        println!("{:?}", string01.as_bytes());
        // 从 char 的视角来看
        // "नमस\u{94d}त\u{947}"
        println!("{:?}", string01.chars().as_str());
    }
}
