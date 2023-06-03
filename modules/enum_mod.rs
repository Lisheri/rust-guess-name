// 声明 unstable 方法
#![feature(is_some_and)]

use num::abs;

#[derive(Debug)]
enum IpAddKind {
    // 可以使用枚举直接定义类型
    // ipv4的成员总是包含四个0-255之间的数字
    V4(u8, u8, u8, u8),
    V6(String),
}

fn route(ip_kind: &IpAddKind) {
    println!("this ip address is {:?}", ip_kind);
}

#[derive(Debug)]
enum Message {
    Quit,
    Move{ x: i32, y: i32 }, // struct类型枚举字段
    Write(String), // 元组类型枚举字段
    ChangeColor(i32, i32, i32) // 元组类型枚举字段
}

// impl不仅可以为Struct定义方法, 也可以为enum定义方法
// 所有枚举成员均可访问枚举方法
impl Message {
    fn call(&self) {
        // 这里的 &self 指向调用当前方法的枚举成员
        println!("current is: {:?}", self);
    }
}

fn tst_message_fn() {
    let message = Message::Move{x: 12, y: 18};
    message.call();
}

// ! 特殊枚举: Options枚举
// Options编码了一个非常常见的场景, 也就是 有值 和 空值
// Rust中没有空值功能, 也就是(Null)空指针, 在其他语言中, 变量有两种情况, 一种是正常变量, 另一种就是空指针
// 引入空指针以后, 很容易产生空指针异常, 因为空指针无处不在
// Rust没有空指针, 但是有一个表示有值和没有值的枚举, 定义在标准库中, 也就是 Options
/**
enum Option<T> {
    None,
    Some(T),
}
*/

fn use_options_enum() {
    let some_number = Some(100);
    // let some_char = Some('a');
    let absent_number: Option<i32> = None;
    println!("this number is: {:?}", &some_number);
    println!("some_number is Some?: {:?}", &some_number.is_some());
    println!("is some_number > 10 ?: {:?}", &some_number.is_some_and(|x| { x > 10 }));
    // 是否为 None
    println!("is some_number is None?: {:?}", &some_number.is_none());
    println!("is absent_number is None?: {:?}", &absent_number.is_none());
    // println!("this char is: {:?}", &some_char);
    // println!("this null is: {:?}", &absent_number);
    // Option<T> 和 T, 是两个类型, 避开了空指针异常的情况, 编译阶段会报错
    // 但如果引入空指针null, 编译器会将null视为一个正常类型的值, 可以与其他类型进行操作, 但在运行时, 会导致程序崩溃
    // 如果要对两个类型的值进行操作, 比如说 Option<i8> + i8, 那么需要将Option<i8>转换为i8
    // 这个转换过程中, 就会出现一个情况, 假设某值存在, 但某值并不存在, 这样编译器介入后, 可以将对弈的错误抛出

}



pub fn enum_mod_main() {
    // let four = IpAddKind::V4(127, 0, 0, 1);
    // let six = IpAddKind::V6(String::from("::1"));
    // route(&four);
    // println!("this ip address is {:?}", four);
    // tst_message_fn();
    use_options_enum();
}