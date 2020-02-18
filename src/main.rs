use std::collections::HashMap;
use std::io::prelude::*;
use std::fs::File;
use std::env;

mod interpreter;
mod converter;

fn main() {
    // You can customize instructions!
    let mut map = HashMap::new();
    map.insert('🤢', '>');
    map.insert('🤡', '<');
    map.insert('🤖', '+');
    map.insert('🧠', '-');
    map.insert('🥶', '.');
    map.insert('🤯', ',');
    map.insert('👹', '[');
    map.insert('👿', ']');

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("\x1b[31m\nYou need enter *.ebf file.\n\x1b[m");
        return;
    }

    let mut file = File::open(&args[1]).unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    let size: usize = 10000;
    let code = converter::run(map, &data);
    interpreter::run::<i32>(code, vec![0i32; size]);
}

