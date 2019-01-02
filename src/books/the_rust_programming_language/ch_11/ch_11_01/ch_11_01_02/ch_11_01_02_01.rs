/*
   Writing Automated Tests
       How to Write Tests
           Checking Results with the assert! Macro
 */

pub fn fn_11_01_02_01() {
    println!("-------------------------------------------------- 01");
    {}
}

struct Rect {
    width: u32,
    height: u32,
}

impl Rect {
    fn can_hold(&self, other: &Rect) -> bool {
        self.width > other.width && self.height > other.height
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_larger_hold_smaller() {
        let rect01 = Rect {
            width: 9,
            height: 9,
        };
        let rect02 = Rect {
            width: 8,
            height: 8,
        };

        assert!(rect01.can_hold(&rect02));
    }

    #[test]
    fn can_smaller_hold_larger() {
        let rect01 = Rect {
            width: 9,
            height: 9,
        };
        let rect02 = Rect {
            width: 8,
            height: 8,
        };
        /*
           assert!() 的条件为 false 时，会调用 panic!()
         */
        assert!(rect02.can_hold(&rect01));
    }

}
