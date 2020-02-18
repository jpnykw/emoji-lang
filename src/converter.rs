use std::collections::HashMap;

pub fn run(
    map: HashMap<char, char>,
    code: &String
) -> String {
    let mut new_code = Vec::new();

    for c in code.chars() {
        let opt = map.get(&c);
        if opt != None { new_code.push(opt.unwrap()); }
    }

    new_code.iter().cloned().collect::<String>()
}

