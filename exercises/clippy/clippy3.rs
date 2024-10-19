// clippy3.rs
// 
// Here's a couple more easy Clippy fixes, so you can see its utility.
//
// Execute `rustlings hint clippy3` or use the `hint` watch subcommand for a hint.



#[allow(unused_variables, unused_assignments)]
fn main() {
    let my_option: Option<()> = None;
    
    // 修复：使用 match 而不是 unwrap()，因为 my_option 是 None
    if my_option.is_none() {
        println!("my_option is None");
    }

    // 修复：数组的元素之间需要用逗号分隔
    let my_arr = &[
        -1, -2, -3, // 加上逗号
        -4, -5, -6,
    ];
    println!("My array! Here it is: {:?}", my_arr);

    // 修复：使用正确的方法来创建一个空 Vec
    let my_empty_vec: Vec<i32> = vec![1, 2, 3, 4, 5].into_iter().take(0).collect();
    println!("This Vec is empty, see? {:?}", my_empty_vec);

    let mut value_a = 45;
    let mut value_b = 66;
    // 修复：交换两个值的方法
    std::mem::swap(&mut value_a, &mut value_b);
    println!("value a: {}; value b: {}", value_a, value_b);
}