// 函数

// Rust中函数和变量名推荐使用 snake case 风格
// 定义函数使用`fn`关键字
// 声明函数形参时, 必须声明参数类型, 这样编译器就不需要开发人员在其他地方注明类型来指出意图
pub fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {}{}", value, unit_label);
    // rust的`语句` 不返回值, 因此不能把let语句赋值给另一个变量, 如下语句会产生错误
    // let x = (let y = 5);
    // 会产生panic, 这个和其他语言不通, C和Ruby的赋值语句会返回所赋的值, 在这些语言中, 可以使用 x = y = 5 这样的方式声明, 但是Rust不行
    
    // 语句不会返回值, 但是表达式会计算出一个值
    // 并且编写的大部分Rust代码是由表达式组成的
    // 比如 ley y = 5; 这个 `5` 就是一个表达式, 计算出的值是 `5`
    // 函数调用, 宏调用, 大括号创建的块级作用域等都是一个表达式, 比如
    let y = {
        let x = 3;
        x + 1
    };
    // 这里会输出y的值, 也就是 x + 1, 为4, 代码块返回的值就是4, 这个4会作为代码块的值绑定到y上
    // ? 这里 x + 1没有分号, 说明他是表达式, 如果有分号, 那么它就是语句, 语句不会返回值
    println!("the value of y is: {y}");
}

// 具有返回值的函数
// 在 rust 中, 函数返回值等同于函数最后一个`表达式`的值, 当然, 也可以显式使用 return
pub fn five() -> i32 {
    5
}

pub fn plus_one(x: i32) -> i32 {
    x + 100
}
