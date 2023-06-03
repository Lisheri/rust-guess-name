use std::io;
pub fn variables_main() {
    // 变量默认为不可变常量, 如果需要对变量进行修改, 需要使用mut关键字, 声明可变
    // 在rust看来, 如果一个值不可变, 那么将无需对此进行跟踪, 代码会更易于推导
    // let mut x = 5;
    // println!("this value of x is {x}");
    // x = 6;
    // println!("this value of x is {x}");

    // 常量, 永远不可变, 不能使用mut关键字, 必须声明类型
    // const FUCKER_AREA: u32 = 60 * 60 * 3.14;

    // 隐藏, 前面说过, 可以通过反复声明同一个变量, 后者可以取代前者, 同时将前者隐藏, 后续所有代码使用, 均访问后者
    let a = 10;

    let a = a + 100; // 隐藏前一个a
    {
        // 块级作用域的a, 隐藏外部作用域a
        let a = a * 20;
        println!("The value of a in the inner scope is {a}");
    }
    println!("The value of x is: {a}");

    // mut和隐藏是有却别的, mut为可变变量, 隐藏是使用let声明了一个新的变量, 开辟了新的空间, 只是不在访问原有地址
    // 同时隐藏前后的变量类型也可以是不同的类型, 但是mut不行, mut必须是相同类型
}

pub fn variables_data_type() {
    // rust是静态类型语言, 每一个值都属于某一个数据类型(类型收窄), 用于告诉rust它被指定为哪种数据, 以明确数据处理方式。
    // rust数据类型分为: 标量(scalar) 和 复合(compound)
    // rust符合静态类型语言的特点, 也就是在编译时必须知道所有变量的类型。一般情况下编译器可以推断出想要的类型
    // 不过在具有多种类型时, 比如前面说的利用parse将String转换为数字, 就需要增加类型注解
    // let guess: u32 = "42".parse().expect("Not a number!"); 此处不加类型注解, 就会被抛提示, rust无法区分其类型

    /*
        标量分为: int, float, string(字符串) | char(字符), bool

            整型有两个分支: 有符号(ixxx)和无符号(uxxx)两种
            长度分为: 8 16 32 64 128 和 arch(由宿主计算机架构推断, 64位计算机为64位)
            以 int 类型为例, 32位有符号长度为 i32, 无符号长度为 u32
            有符号变体可以存储 -(2^(n - 1)) 到 2^(n - 1) - 1在内的数字
            !注: 整形超出类型上限会出现整形溢出, 发生 "整型溢出" 时, rust在生产环境下不会进行panic, 而是对溢出整型进行补码计算
            也就是(以u8为例), 255为上限, 超出回到最小值. 256变成0, 257就是1

            如果需要显示的处理溢出, 需要使用如下几种方法:
                + 使用 `wrapping_*` 方法进行 wrapping, 比如 `wrapping_add`执行求和
                + 使用`checked_*`方法, 检测溢出, 如果出现, 则返回None值
                + 使用 `overflowing_*` 方法返回值和一个bool, 表示是否出现溢出
                + 使用 `saturating_*` 方法在值的最小值或最大值处进行饱和处理
    */
    // char类型, rust的char为 4个字节, 代表了一个 unicode 标量值, 意味着可以比ASCII表示更多的内容
    let heart_eyed_cat = '😻';
    // 复合类型可以将多个值组合成一个类型。Rust有两个原生的复合类型: 元组(tuple)和数组(array)
    // 元组是一个或多个类型值的组合, 长度固定, 一旦声明, 长度和类型顺序不可修改
    let tup: (i32, f64, u8) = (599, 6.4, 1);
    // 元组`解构赋值`(这个概念和js一致), 分别获取三位并赋值到变量x, y, z上
    let (x, y, z) = tup;
    println!("The value of x is: {x}");
    println!("The value of y is: {y}");
    println!("The value of z is: {z}");
    // 也可以直接使用 tuple.index 直接访问, 序列号从0开始
    println!("The value of tup.0 is: {}", tup.0);
    println!("The value of tup.1 is: {}", tup.1);
    println!("The value of tup.2 is: {}", tup.2);
    // 不带任何值的元组叫做 单元(unit)元组: let unit:() = (); 表示空或空的返回类型
    // 如果表达式不返回任何其他值，则会隐式返回单元值, 也就是`()`。
}

pub fn variables_array_fn() {
    // 数组类型
    // rust中数组的每个元素必须相同, 同时长度固定, 因此在大多数情况下, 会选择使用 Vector 取代数组, Vector可自增
    let a = [1, 2, 3, 4];
    // rust的数组在栈(stack)上开辟空间, 而不是在堆(heap)上开辟空间
    // vector类型是标准库提供的一个允许增长和缩小长度的类似数组的集合, 一般不确定应该使用数组还是vector时, 应该使用vector
    // 一般确定元素不会改变时, 应当使用数组, 比如月份
    // 可以通过 `[类型, 长度]` 的方式约束数组长度和类型
    let months: [&str; 12] = ["January", "February", "March", "April", "May", "June", "July",
        "August", "September", "October", "November", "December"];
    // 通过 `[初始值;长度]`的方式创建一个每个元素都为初始值的数组
    // ? 初始值3, 长度为5
    let a = [3; 5];
    // 数组在栈上分配固定大小的单个内存块, 可以使用索引访问数组的元素
    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];

    // 无效数组访问
    println!("Please enter an array index");
    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");
    let index: usize = index.trim().parse().expect("Index entered was not a number");

    let element = a[index];
    println!("The value of the element at index {index} is: {element}");
}
