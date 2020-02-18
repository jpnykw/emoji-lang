extern crate num;

use num::One;
use std::ops::AddAssign;
use std::ops::SubAssign;

pub fn run<T: std::string::ToString>(
    code: String,
    mut memories: Vec<T>
) -> Vec<T>
    where T: Ord + Clone + One + AddAssign + SubAssign,
{
    let mut index: usize = 0;
    let mut pointer: usize = 0;
    let mut output: Vec<char> = Vec::new();

    loop {
        let string = memories[pointer].to_string();
        let c = code.chars().nth(index).unwrap();

        match c {
            '>' => pointer += 1,

            '<' => pointer -= 1,

            '+' => memories[pointer] += T::one(),

            '-' => memories[pointer] -= T::one(),

            '.' => {
                let num: u8 = string.parse().unwrap();
                output.push(num as char);
            },

            '[' => {
                let num: i64 = string.parse().unwrap();
                if num == 0 {
                    let mut count = 0;
                    index += 1;

                    while count != 0 || code.chars().nth(index).unwrap() != ']' {
                        let tok = code.chars().nth(index).unwrap();
                        if tok == '[' { count += 1; }
                        if tok == ']' && count > 0 { count -= 1; }

                        if index == code.chars().count() { break; }
                        index += 1;
                    }
                }
            },

            ']' => {
                let num: i64 = string.parse().unwrap();
                if num != 0 {
                    let mut count = 0;
                    index -= 1;

                    while count != 0 || code.chars().nth(index).unwrap() != '[' {
                        let tok = code.chars().nth(index).unwrap();
                        if tok == ']' { count += 1; }
                        if tok == '[' && count > 0 { count -= 1; }

                        if index == 0 { break; }
                        index -= 1;
                    }
                }
            },

            _ => {}
        }

        index += 1;
        if index >= code.chars().count() { break; }
    }

    println!("\n{}", output.iter().cloned().collect::<String>());
    memories
}

