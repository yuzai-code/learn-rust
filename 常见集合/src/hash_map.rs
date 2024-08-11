use std::collections::HashMap;

pub fn new_hashmap() {
    let mut scores = HashMap::new();
    // 往scores 中的 HashMap插入值
    scores.insert(String::from("blue"), 10);
    scores.insert(String::from("black"), 5);
    for (k, v) in &scores {
        println!("{k}: {v}");
    }
}

/// 定义scores返回值，在main.rs中进行定义返回
pub fn main_new_hashmap() -> HashMap<String, i32> {
    let mut scores = HashMap::new();
    scores.insert(String::from("yellow"), 11);
    scores.insert(String::from("gree"), 2);
    scores
}
