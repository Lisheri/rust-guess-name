// 学习rust, 需要忘记以往的范式
// 库crate可以包含任意能被其他程序使用的代码, 但是不能自执行
use std::io; // 获取输入输出的标准库
// 引入Rng, 本质上是一个Trait
// 该Trait 定义了随机数生成器应该使用的方法, 如果要使用这些方法, 必须保证此Trait在作用域中
use rand::Rng;
// 枚举, 用于比较数字大小
use std::cmp::Ordering;
use num::BigInt;
use num::bigint::BigUint;

#[path = "../modules/mod.rs"]
mod modules;

fn guess_number() {
    println!("Guess the number!");
    // ? 调用 rand::thread_rng() 函数, 该函数返回随机数生成器
    // ? 接着调用生成器的 gen_range方法, 该方法由rand::Rng引入到作用域的Rng Trait 定义
    // ? gen_range入参为一个 range expression, 表示一个范围, 同时生成一个范围内的随机数(int32)
    // ? range expression也就是 start..=end 这样的形式, 标识开始和结束
    // ? 由于锁定了 guess 为u32类型, 同时这两个数字正在进行比较, rust会根据context自动推断secret_num也是u32
    let secret_num = rand::thread_rng().gen_range(1..=100);
    // println!("The secret number is : {secret_num}");
    loop {
        println!("Please input your guess.");
        let mut guess = String::new();
        // 输入监听
        io::stdin()
            .read_line(&mut guess)
            // read_line 返回一个 Result 枚举, 包含 Ok和Err
            // Ok包含成功的值
            // Err包含失败的原因
            .expect("Failed to read line");
        // guess应该转换为u32, 通过如下方式将 String 转换为 u32
        // ? rust规则允许重名变量 隐藏(Shadowing) 原变量的值, 常用于类型转换
        // ? read_ling 中, 获取到的内容, 包含了用户键入回车时, 留下的回车符和换行符, 也就是\r\n, trim用于消除这两个符号
        // ? parse 只有在逻辑上能够正确转换时, 才会转换成功, 如果失败了很容易导致程序panic, 因此返回一个 Result枚举, 需要开发人员进行处理
        // ? 这里不使用expect, 而是匹配Result枚举
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            // 输错了继续输
            Err(_) => continue,
        };
        match guess.cmp(&secret_num) {
            Ordering::Less => println!("小了"),
            Ordering::Greater => println!("大了"),
            Ordering::Equal => {
                println!("猜对了");
                // 猜对了才结束, 否则一直输入
                break;
            },
        }
    }
}
// 处理一次猜测
#[warn(unused_assignments)]
fn main() {
    // guess_number();
    // modules::variables::variables_data_type();
    // modules::variables::variables_array_fn();
    // modules::func::print_labeled_measurement(10, 'a');
    // let x = modules::func::five();
    // let y: i32 = modules::func::plus_one(10);
    // println!("The value of func::x is: {x}");
    // println!("The value of func::y is: {y}");
    // modules::control_flow::control_flow(4);
    // modules::control_flow::loop_flow();
    // modules::control_flow::nested_loop();
    // modules::control_flow::while_fn();
    // modules::control_flow::for_fn();
    // let x = modules::control_flow::fibonacci(20);
    // let y = modules::control_flow::fibonacci_over_flow(20);
    // println!("result is: {x}");
    // println!("result is: {y}");
    // modules::ownership::variables_scope();
    // modules::ownership::variables_scope_string();
    // modules::ownership::variables_move_and_clone_on_heap();
    // modules::ownership::variables_copy_on_heap();
    // modules::ownership::ownership_in_func();
    // modules::ownership::scope_return_val();
    // modules::references_and_borrowing::references_and_borrowing();
    // modules::slice::slice_references();
    // 结构体
    // modules::structs::structs_main();
    // modules::enum_mod::enum_mod_main();
    // modules::mem_replace::use_replace();
    // let result: BigUint = modules::mem_replace::fibonacci(10);
    // println!("计算结果为: {:?}", result);
    // modules::ownership::main_ownership();
    // println!("计算结果为: {:?}", modules::climb_stairs::climb_stairs(2));
    modules::reference::reference_fn();
}






