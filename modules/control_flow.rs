use num::bigint::BigUint;
use num::traits::{One};
use std::mem::replace;
// 控制流
pub fn control_flow(number: i32) {
    // 和其他语言不同的是, 条件语句的判断条件必须是一个 bool, rust并不会主动转换类型
    // 因此如下语句会直接panic
    // if number {...}
    if number < 3 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
    
    // let 语句中使用if
    let condition = true;
    // rust没有?:三元运算符, 和golang一样, rust认为一个语言中只需要一个条件控制流表达式
    // 同时, rust的条件不是语句, 而是表达式, 可以返回值, 因此可以直接使用如下方式, 但是多个分支的类型必须相同, 否则会报错
    // 这一点和C不一样, C只会告警, 但是允许转换, 但是rust需要准确知道开发人员的意图
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {number}");
}

// 循环
pub fn loop_flow() {
    // rust有三种循环: loop, while, for
    // loop会反复执行, 直到明确要求停止
    let mut a = 0;
    let res = loop {
        a += 1;
        if a == 20 {
            println!("now a = {a}, stop!");
            // 使用 break 告诉程序停止循环
            // break后接语句可以将该值作为循环的值返回
            break a * 10;
        } else {
            println!("now a = {a}, again!");
        }
    };
    println!("loop value is: {res}");
}

// 嵌套循环, 以及停止循环歧义
pub fn nested_loop() {
    // 如果存在嵌套循环, 可以选择在一个循环上指定一个 `循环标签`, 来指定关键字作用的循环
    let mut count: isize = 0;
    // 外层循环标签, 'counting_up, 循环标签以 `'`开头
    // ! 注意和 char 区分, 只有一个单引号
    'counting_up: loop {
        println!(" count = {count}");
        let mut remaining: isize = 10;
        loop {
            println!("remaining = {remaining}");
            if (remaining == 9) {
                // 停止当前循环
                break;
            }
            if count == 2 {
                // 停止外层循环 counting_up
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");
}

// while循环
pub fn while_fn() {
    let mut number = 10;
    while number > 1 {
        println!("number is: {number}");
        number -= 1;
    }
}

pub fn for_fn() {
    let a = [1, 2, 3, 4, 5, 6];
    for element in a {
        // for in 中 element 代表的是数组当前遍历成员
        println!("the value is: {element}");
    }
    // 利用 x..y, 直接生成一个 x到y的Range, 本质上是一个数组
    // let a: Range<i32> = 1..4;
    // 利用rev反转
    for num in (1..4).rev() {
        println!("the value is: {num}");
    }

}

// 返回大数
pub fn fibonacci(n: isize) -> BigUint {
    let mut pre1: BigUint = One::one();
    let mut pre2: BigUint = One::one();
    let mut cur: BigUint = One::one();
    if n < 2 {
        return cur;
    }
    for _ in 2..n {
        // 利用 replace 方法, 将 pre2的地址替换为 cur
        // 返回pre2所有权转移给pre1

        // replace 作用: 读取 pre2 堆地址对应的值, 同时将 cur 的值写入到 pre2 对应的堆地址中
        // 然后返回前面读取的堆地址对应值
        // 所以这一步相当于将 cur的值 -> pre2, pre2的值 -> pre1, 避开了所有权的转移
        pre1 = replace(&mut pre2, cur);
        cur = &pre1 + &pre2;
    }
    cur
}

// ? 保留方案, 过于粗暴, 会栈溢出, 导致计算不出结果
pub fn fibonacci_over_flow(n: isize) -> isize {
    if n == 1 { return 1; }
    if n == 0 { return 0; }
    return fibonacci_over_flow(n - 1) + fibonacci_over_flow(n - 2);
}
