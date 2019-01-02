/*
  Functional Language Features: Iterators and Closures
      Processing a Series of Items with Iterators
          Using Closures that Capture Their Environment
*/

pub fn fn_13_02_05_01() {
    println!("-------------------------------------------------- 01");
    {
        /*
        
        */
    }
}

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_my_size(shoes: Vec<Shoe>, my_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|x| x.size == my_size).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let shoes: Vec<Shoe> = vec![
            Shoe {
                size: 10,
                style: String::from("red"),
            },
            Shoe {
                size: 13,
                style: String::from("yellow"),
            },
            Shoe {
                size: 10,
                style: String::from("green"),
            },
        ];

        let my_shoes = shoes_in_my_size(shoes, 10);
        assert_eq!(
            my_shoes,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("red"),
                },
                Shoe {
                    size: 10,
                    style: String::from("green"),
                },
            ]
        )
    }
}
