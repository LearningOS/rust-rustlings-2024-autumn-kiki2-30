// macros3.rs
//
// Make me compile, without taking the macro out of the module!
//
// Execute `rustlings hint macros3` or use the `hint` watch subcommand for a
// hint.

mod macros {
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }

    // 使用这个 `pub` 关键字将宏暴露到模块外部
    pub(crate) use my_macro; // 将宏导出
}

fn main() {
    macros::my_macro!(); // 使用模块路径调用宏
}