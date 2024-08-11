mod hash_map;
mod vector;
/// rust编写文档
fn main() {
    println!("Hello, world!");
    hash_map::new_hashmap(); // 调用模块
    let scores = hash_map::main_new_hashmap();
    for (key, value) in &scores {
        println!("{},{}", key, value);
    }
    vector::update_vector(); // 更新vector中的值
    vector::get_vector(); // 获取vector中的值
}
