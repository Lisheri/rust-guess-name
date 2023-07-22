// + 1. 表达式语言
use super::system_info::{get_cpu_info, Cpu};

#[derive(Debug)]
enum CpuStatus {
    Idle,
    Busy,
}

// + 2. 块与分号
fn bound() {
    // 代码块同样也是 "表达式". 块产生值, 可以用于任何需要值的地方
    /* let display_name = match post.author() {
        Some(author) => author.name(),
        None => {
            let network_info = post.get_network_metadata()?;
            let ip = network_info.client_address();
            ip.to_string()
        }
    } */
    /*
        上述代码中, Some(author) => 之后的代码是一个简单的表达式 author.name()
        而 None => 之后的代码块也是一个表达式
        在Rust中它们没有区别, 这个代码块的值就是最后一个表达式 ip.to_string() 的值
        ! 注意, 他没有分号
        如果一个块看起来像C代码, 分号出现在了熟悉的位置, 那么它也会像C的块一样运行, 其值为()
        但是在块的最后一行省略分号, 那么就会让这个块产生一个值, 即最后一个表达式的值
    */

    // 在Rust中, 分号一定是有意义的, 不能像Js一样省略
    let msg = {
        // let声明: 分号是必须的
        // let dandelion_control = puffabll.open();
        // 表达式+分号: 方法调用, 返回值被清除
        // dandelion_control.release_all_seeds(launch_codes);
        // 表达式不带分号: 方法被调用, 返回值保存在 msg 中
        // dandelion_control.get_status()
        1.to_string()
    };

    // Rust的块很精妙, 既可以包含声明, 又可以在末尾产生值

    // 空语句 可以出现在块中。 所谓空语句其实就是末尾的一个空的 分号(;)
    if msg == "1".to_string() { // 空语句, 什么也不会发生, 只是传达淡淡的惆怅
    }
}

// + 3. statement 声明
fn statement_fn() {
    // let 声明局部变量(声明变量时, 类型和初始值是可选的, 但是分号是必须的)
    let name: char = '1';
    // 可以只声明不初始化
    let name1: char;
    name1 = '2'; // 通过赋值进行初始化, 但是它并非可变变量, 因此初始化过后不能进行修改
                 // name1 = '3'; // 不可修改

    // 块内也可以包含 "特性项" 声明
    // 所谓  "特性项"(item), 指的是任何可以在程序或模块的全局中出现的声明, 比如fn, struct 或 use

    // 任何块中都可以包含上述的一些 特性项, 比如声明函数的fn, 声明结构体的 struct
    fn tst() {}

    struct User {
        uid: String,
        username: String,
    }

    use super::system_info::Cpu;
}

// + 4. if与match
fn if_and_match() -> String {
    // if 表达式已经很熟悉了
    // ! 只不过Rust中, if的每一个 condition 都必须是 bool 类型, 不会自动做bool类型隐式转换
    // * 甚至不需要给圆括号(给了会被rustc报警告)
    if (true) {
        println!("111")
    }

    // match表达式和 switch case类似(但其实更灵活, 因为块可以返回值)
    let code: i32 = 2;
    match code {
        0 => println!("Ok"),
        1 => println!("Wires Tangled"),
        2 => println!("User Asleep"),
        _ => println!("Unrecognized Error {}", code), // 默认值
    }

    // 编译器可以使用 跳转表(jump table) 来优化这种 match 表达式
    // 如果 match 的每个分支都产生一个常量值, 那么也可以应用同样的优化
    // 此时, 编辑器会构建一个这些值的数组, 而 match 会编译为对数组的访问。除了边界检查, 编译后的代码中根本没有分支

    // 功能丰富的 match 源于对各种 "模式" 的支持。这些模式可以用在每个分支 => 的左侧
    let a = Some("111");
    // 支持Options枚举
    match a {
        Some(name) => println!("Hello, {}", name),
        None => println!("fuck off"),
    }

    // 所有模式都必须至少有一个匹配, Rust会对无法涵盖所有可能值的match表达式拒绝编译

    // 同时, 如果使用 if 表达式赋值, 那么if 表达式的所有分支块都必须返回相同的类型
    // 类似的, match表达式的分支也必须返回相同的类型

    // * if let 表达式
    /*
        if let pattern = expr {
            block1
        } else {
            block2
        }

        上面的 expr要么匹配 pattern, 走 block1, 要么不匹配, 走 block2
        有时候这是从 Option 或者 Result中获取数据的一种便捷形式
        ! 核心在于当前只想匹配某一种模式, 其他的都可以忽略, 此时可以使用if let 代替 match。否则需要在 match 中写出 _ => { block } 这样的代码
    */
    let cookie = "1111";
    let cookie2 = Some("1111");

    if Some(cookie) == Some("1111") {
        println!("对了");
    }

    if let Some(cookie) = cookie2 {
        // 这个return, 是作为函数返回值, 而不是 if let块的返回值
        // if let 本质上只是 match 的语法糖
        println!("aaa: {}", cookie);
        return cookie2.unwrap().to_string();
    } else if let Some(cookie) = Some("123123") {
        return 1.to_string();
    } else {
        return 2.to_string();
    }
}

// + 5. 循环
fn loop_fn() {
    // 循环lifetime标签
    'search: for room in [[1], [2], [3], [4]] {
        for spot in room {
            if spot == 1 {
                println!("Your keys are {} in this {:?}", spot, room);
                // 通过lifetime参数直接结束外层循环
                break 'search;
            }
        }
    }
    let cookie: &str = "111";
    // while let pattern = expr 语法, 和 if let 类似, 也是满足后面的 pattern = expr后循环代码块
    'a: while let Some(cookie) = Some("111") {
        println!("cookie: {}", cookie);
        break 'a;
    }

    // 标准循环
    for i in 0..20 {
        // .. 操作符会产生一个 Range, 和 std::ops::Range{ start: 0, end: 20 } 一样
        // Range也是一个迭代类型, 因此可以用于循环
        // ? 所谓的迭代类型, 就是实现了 std::iter::IntoIterator trait
        println!("current i is: {}", i);
    }

    // 在循环过程中, 被遍历的值的所有权也会转移到循环中, 因此每一次循环后, 就会消耗一个所有权
    // 最简单的补救方法是使用 &(引用)
    // 如果是迭代的一个 &mut, 那么循环体获取到的每一个参数都是&mut
    let mut strings: Vec<String> = vec!["111".to_string(), "222".to_string()];
    for rs in &mut strings {
        // 给每一个加一个换行符
        if *rs == "222".to_string() {
            // 直接修改
            *rs = "我日你妈".to_string();
        } else {
            rs.push('\n');
        }
    }
    println!("aaaa: {strings:?}");
}


// + 6. return表达式
fn return_expression() {
    // return表达式表示退出当前函数, 向调用者返回值
    // 无值的return默认返回一个 零元组(所谓的"基元类型")
    // ? return; === return(); 这个括号就是 基元类型

    // ! "?"操作符中其实也带有一段return
    // 本质上 ? 操作符是对 match表达式的缩写:
    /* 
        
        let output = File::create(filename)?;
        上述语句其实就是:(快速包裹了一段 Result枚举)
        let output = match File::create(filename) {
            Ok(f) => f,
            Err(err) => return Err(err);
        }

    */
}

// + 7. 函数与方法调用
/* 
    跟在其他许多语言中一样 调用函数与方法的语法在Rust中是一样的
    let x = gcd(1302, 462); // * 1. 函数调用
    let room = player.location(); // * 2. 方法调用
    其中 player 是自定义的结构体Player的实例或者 trait
    在Player中有一个方法 location

    player可以是一个 Player, 一个对类型Player的引用 &Player
    甚至可以是一个智能指针Box<Player>或者Rc<Player>

    * 3. 静态方法调用
    let mut numbers = Vec::new(); // 这个new就是Vec的静态方法
    和面向对象语言一样, 静态方法通过类(结构体)直接调用, 实例方法通过类的实例(结构体的值)调用

    * 4.方法也可以链式调用
    Iron::new(router).http("localhost:3000").unwrap();

    * Rust语法有一个怪癖: 通常用于函数调用或方法调用的语法不能用于泛型 Vec<T>
    return Vec<i32>::with_capacity(3000); // 报错
    let ramp = (0..20).collect<Vec<i32>>(); // 同样报错

    其实上面的问题在于: "<" 是一个"小于操作符", Rust编译器对这样的情况会建议使用 ::<T>而不是<T>
    如下:
    return Vec::<i32>::with_capacity(3000); // 可以
*/

/* 
    + 8. 字段与元素
    * 结构体字段:
    game.black_pawns;

    *元组字段
    coords.1;

    * 数组元素
    arr[i]

    * 左值, 因为它们会出现在赋值操作的左侧
    game.black_pawns = 0x00ff0000_00000000_u64
    coords.1 = 0;
    arr[2] = Some(Piece::new(Black, Knight, coords));

    * 从数组或者Vec中提取切片
    * 其中 game_moves可以是数组、Vec或者Slice
    let second_half = &game_moves[midpoint..end];

    * 范围操作符 ..
    * 允许省略左右两侧的操作数
    * 根据操作数的个数, 可能产生 4种 不同类型的对象:
    .. // 全部范围
    a.. // 从a开始到结尾的范围
    ..b // 从头到b - 1的范围
    a..b // 从a到b的范围
    * Rust的范围和js一样, 是半开口的, 包含开始位置, 但是不包含结束为止

    ! 在迭代过程中, 必须同时包含 开始位置 和 结束位置
    ! 不过在数组(包含Vec)切片中, 上面四种形式都可以使用, 会自动补充开始和结束位置
*/

// 下面是一个经典分治法实现的快速排序
// * 快速排序核心
fn quicksort(slice: &mut Vec<usize>) {
    if slice.len() < 1 {
        return; // 没有长度不需要排序
    }
    fn partition(slice: &mut Vec<usize>, begin: usize, end: usize) -> usize {
        let (mut i, v) = (begin, slice[end - 1]);
        for j in begin..end - 1 {
            if slice[j] <= v {
                slice.swap(i, j);
                i += 1;
            }
        }
        slice.swap(i, end - 1);
        i
    }

    fn quick_sort(slice: &mut Vec<usize>, begin: usize, end: usize) {
        if begin + 1 < end {
            let mid = partition(slice, begin, end);
            quick_sort(slice, begin, mid);
            quick_sort(slice, mid, end);
        }
    }

    quick_sort(slice, 0, slice.len())
}


// 类型转换
fn type_transform() {
    let x = 17;
    // 类型转换使用 as 关键字
    let index = x as usize;

    // 解引用强制转换
    // &String -> &str
    // &Vec<i32> -> &[i32]
    // &Box<Chessboard> -> &Chessboard
}

pub async fn expression_language() {
    // Rust虽然看起来和C家族的语言很像, 但是这只是它的一个策略。
    // 在C中, 表达式和语句有着明显的区别
    // 比如下面是表达式:
    // 5 * (fahr - 32) / 9;
    // 下面是语句
    /*
        for (;begin != end; begin++) {
            if (*begin == target) {
                break;
            }
        }
    */

    // * 表达式有值, 语句确没有
    // Rust 是所谓的 "表达式语言"。
    // 它遵循一种比较早期的传统, 可以追溯到lisp, 当时表达式包打天下。
    // 在C家族中, if和switch不产生值, 也不能用在表达式中间。而在Rust中, if和match可以 "产生" 值

    // 产生值的match表达式
    /*
        pixels[r * bounds.0 + c] = match escapes(Complex { re: point.0, im: point.1 }, 255) {
            None => 0,
            Some(count) => 255 - count as u8,
        }
    */
    // Rust 的 if表达式可以用来初始化变量, 并且配合match产生值
    println!("start");
    let cpu: Cpu = match get_cpu_info(1000).await {
        Ok(cpu) => cpu,
        Err(_) => Cpu {
            count: 0,
            usage: 0.0,
        },
    };
    const IDLE_USEAGE_PERCENT: f32 = 10.0;
    // 利用if初始化变量
    let status = if cpu.usage <= IDLE_USEAGE_PERCENT {
        CpuStatus::Idle
    } else {
        CpuStatus::Busy
    };
    // * 同时 match 表达式可以作为参数传递给宏或者函数
    // 在Rust中, 大多数控制流都是表达式, 几乎没有语句~
    println!("current cpu status is: {:?}", status);
    println!("current cpu count is: {:?}", cpu.count);
    println!("current cpu useage is: {:?}", cpu.usage);

    println!("--------------------------------循环----------------------------------------");
    loop_fn();
    println!("--------------------------------循环----------------------------------------");

    println!("--------------------------------return表达式----------------------------------------");
    return_expression();
    println!("--------------------------------return表达式----------------------------------------");

    println!("--------------------------------if和模式匹配----------------------------------------");
    if_and_match();
    println!("--------------------------------if和模式匹配----------------------------------------");
}
