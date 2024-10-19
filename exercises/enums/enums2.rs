// enums2.rs
//
// Execute `rustlings hint enums2` or use the `hint` watch subcommand for a
// hint.

#[derive(Debug)]
enum Message {
    // TODO: define the different variants used below
        Quit,                             // 表示退出的消息
        Echo(String),                     // 表示回声消息，携带一个字符串
        Move { x: i32, y: i32 },         // 表示移动消息，包含 x 和 y 坐标
        ChangeColor(u32, u32, u32),      // 表示改变颜色的消息，携带 RGB 颜色值
}

impl Message {
    fn call(&self) {
        println!("{:?}", self);
    }
}

fn main() {
    let messages = [
        Message::Move { x: 10, y: 30 },
        Message::Echo(String::from("hello world")),
        Message::ChangeColor(200, 255, 255),
        Message::Quit,
    ];

    for message in &messages {
        message.call();
    }
}
