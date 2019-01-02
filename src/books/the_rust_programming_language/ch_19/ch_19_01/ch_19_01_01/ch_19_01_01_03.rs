/*
  Advanced Features
      Unsafe Rust
          Creating a Safe Abstraction over Unsafe Code
*/

use std::slice;

pub fn fn_19_01_01_03() {
    println!("-------------------------------------------------- 01");
    {
        let mut vec = vec![1, 2, 3, 4, 5, 6];

        let m = &mut vec[..];
        let (a, b) = m.split_at_mut(3);

        assert_eq!(a, &mut [1, 2, 3]);
        assert_eq!(b, &mut [4, 5, 6]);
    }

    println!("-------------------------------------------------- 02");
    {
        /*
        
        */

        /*
                fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
                    let len = slice.len();
        
                    assert!(mid <= len);
        
                    (&mut slice[..mid], &mut slice[mid..]) // [E0499]: cannot borrow `*slice` as mutable more than once at a time
                }
        */

        fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
            let len = slice.len();
            let ptr = slice.as_mut_ptr();

            assert!(mid <= len);

            unsafe {
                (
                    slice::from_raw_parts_mut(ptr, mid),
                    slice::from_raw_parts_mut(ptr.offset(mid as isize), len - mid),
                )
            }
        }
    }
}
