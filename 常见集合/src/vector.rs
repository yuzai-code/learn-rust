// pub fn new_v,vector() {
//     // 新建v,v,v,vector
//     // 方法一：使用 Vec::new()
//     let v: Vec<i32> = Vec::new();
//     // 方法二：使用 vec![]
//     let v = vec![1, 2, 3];
// }

pub fn update_vector() {
    // 更新vector，往vector中添加数据,v需要是可变
    let mut v = vec![1, 2, 3];
    v.push(4);
    v.push(6);
    v.push(8);
    for i in &v {
        println!("{}", i);
    }
    println!("v的列表内容是：{:?}", v); // {:?} 用于调试打印，可以将实现的trait类型打印出来
}

// get与[]的区别是方括号超过索取的范围值会报错
pub fn get_vector() {
    // 获取vector中的值两种方法：
    let v = vec![1, 2, 3, 4];
    // 方法一：使用[]进行索引取值
    println!("之前v的值：{:?}", v);
    let get_vec1 = &v[1];
    println!("{}", get_vec1);
    // 方法二：使用get()进行取值
    let get_vec2 = v.get(1);
    match get_vec2 {
        Some(get_vec2) => println!("get_vec2:{get_vec2}"),
        None => println!("获取到的这个值为空{:?}", get_vec2),
    }
    println!("使用v.get之后v的值为：{:?}", v);
}
