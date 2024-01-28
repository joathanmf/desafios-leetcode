// 387. First Unique Character in a String
// https://leetcode.com/problems/first-unique-character-in-a-string/

use std::collections::HashMap;

fn first_uniq_char(s: String) -> i32 {
    let mut hash = HashMap::<char, usize>::new();

    for ch in s.chars() {
        hash.entry(ch).and_modify(|c| *c += 1).or_insert(1);
    }

    for (i, ch) in s.chars().enumerate() {
        let value = hash.get(&ch);

        match value {
            Some(v) => {
                if *v as i32 == 1 {
                    return i as i32;
                }
            }
            None => {}
        }
    }

    return -1;
}

fn main() {
    let s = "bbbbbbbbbbbbcd".to_string();
    let res = first_uniq_char(s);

    println!("Resposta: {}", res);
}

