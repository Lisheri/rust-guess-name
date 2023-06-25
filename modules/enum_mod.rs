// 声明 unstable 方法
#![feature(is_some_and)]
#![feature(result_option_inspect)]
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
    let some_number_defer: Option<&i32> = Some(&100);
    println!("this number is: {:?}", &some_number);
    println!("some_number is Some?: {:?}", &some_number.is_some());
    println!("is some_number > 10 ?: {:?}", &some_number.is_some_and(|x| { x > 10 }));
    // 是否为 None
    println!("is some_number is None?: {:?}", &some_number.is_none());
    println!("is absent_number is None?: {:?}", &absent_number.is_none());
    // unwrap, 如果是Some(v), 则返回v, 如果是None, 则直接panic, 其实就是处理是否为空指针, 遇到空指针直接报错
    println!("some_number is: {:?}", &some_number.unwrap());
    // None使用unwrap会直接panic
    // println!("None {:?} must panic", absent_number.unwrap());

    // expect, 和 unwrap 类似, 但是遇到None, 在panic的同时, 还会打印出 msg 信息
    // println!("current None msg is: {:?}", &absent_number.expect("your fucking garbage code!"));

    // unwrap_or Some(v) -> v, None -> default
    println!("------------------------------------------------------------------------------------------------");
    println!("some_number unwrap_or: {:?}", &some_number.unwrap_or(0));
    println!("None unwrap_or: {:?}", &absent_number.unwrap_or(1)); // None被转换为了1

    // unwrap_or_else, Some(v) -> v, None返回回调函数返回值
    println!("------------------------------------------------------------------------------------------------");
    println!("None unwrap_or_else: {:?}", &absent_number.unwrap_or_else(|| { 123123123 }));

    // unwrap_or_default, 如果是None, 则返回T::default() 的返回值, 这里是i32::default(), 也就是0
    println!("------------------------------------------------------------------------------------------------");
    println!("None unwrap_or_default: {:?}", &absent_number.unwrap_or_default());

    // map, 将Options<T> 通过回调函数, 转换为 Options<U>
    println!("------------------------------------------------------------------------------------------------");
    println!("some_number after map is: {:?}", &some_number.map(|v| { v.to_string() + &"wocao".to_string() }));

    // map_or, 和map不同, 他返回的是U, 而不是Options<U>, 同时会将None转换为default
    println!("------------------------------------------------------------------------------------------------");
    println!("some_number after map_or is: {:?}", &some_number.map_or("fucker".to_string(), |v| { v.to_string() + &"fucker".to_string() }));
    println!("None after map_or is: {:?}", &absent_number.map_or("fucker".to_string(), |v| { v.to_string() + &"fucker".to_string() }));

    // map_or_else 默认值变成了default回调, 而不是一个value, 其他的和 map_or 一样
    println!("------------------------------------------------------------------------------------------------");
    println!("some_number after map_or_else is: {:?}", &some_number.map_or_else(|| {"fucker map_or_else".to_string()}, |v| {v.to_string()}));
    println!("None after map_or_else is: {:?}", &absent_number.map_or_else(|| {"fucker map_or_else".to_string()}, |v| {v.to_string()}));

    println!("------------------------------------------------------------------------------------------------");

    // inspect 遇到Some(v)调用 f(&v), 返回枚举成员本身
    // ? feature API, 声明了也会报错, 目前无解, 无法使用
    // println!("inspect: {:?}", &absent_number.inspect(|v| {println!("正在调用回调函数inspect")}));
    println!("------------------------------------------------------------------------------------------------");

    // ok_or 将Options<T> 转换为 Result<T, E>
    // None 返回 Err(err)
    println!("some_number ok_or is: {:?}", &some_number.ok_or("fuck").unwrap());

    // ok_or_else, 与ok_or类似, 只不过参数变为回调函数, 回调函数返回错误信息的值
    println!("------------------------------------------------------------------------------------------------");
    
    // 解引用, 将Some(&v) 解引用, 返回Some(v)
    println!("some_number_defer as_deref is: {:?}", &some_number_defer.as_deref());
    // 和as_defer类似的还有 as_deref_mut, 只不过后者是解除可变引用 
    println!("------------------------------------------------------------------------------------------------");
    // 其实就是 &, 只不过是Some中的内容进行 &, 有一个是None, 就返回None
    // 与之类似的还有 or, 也就是 | 
    println!("some_number add is: {:?}", &some_number.and(Some(10000)));
    println!("------------------------------------------------------------------------------------------------");

    println!("None or None: {:?}", None::<i32>.or(None));
    println!("Some or None: {:?}", None::<i32>.or(Some(1000)));
    println!("------------------------------------------------------------------------------------------------");
    

    
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