use std::collections::HashMap;
use crate::modules::reference::reference_safe::*;
use crate::modules::reference::reference_shared_edit::*;
// 到目前为止, 前面看到的所有指针类型, 包括Box<T>堆指针以及String、Vec值内部的指针, 都是所有型指针
// 这意味着当所有者被清除时, 引用的资源也会随之清除。
// Rust 也有一种非所有型指针类型, 叫做: 引用(reference), 这种指针对它所引用资源的生命周期没有影响
// ? 引用有一条非常重要的规则: 引用的生命周期不能超过其引用的资源(也就是引用的生存期不能超过它指向的值, 避免出现悬垂引用)
// ! 为了强调上述规则, Rust把创建对某个值的引用的这个动作称为 `借用` 这个值(借的东西, 迟早要还给所有者)
// 说到底, 引用就是地址, 只是Rust在保证引用安全的规则比较新奇, 纵观同类语言, 可以说是史无前例

// 假设我们想做一个包含文艺复兴时期艺术家及其知名作品的表。Rust标准库有一个散列表HashMap, 基于HashMap可以创建我们的自定义类型, 如下
type Table = HashMap<String, Vec<String>>; // 键为String 值为 Vec<String> 的对象
// 可以使用for循环迭代HashMap, 为了调试, 我们写一个函数来打印内容
fn show(table: &Table) {
    // 这里不能直接接收 table 值, 应该使用其 共享引用
    for (artist, works) in table {
        // 在迭代HashMap共享引用的过程中, 会产生对其中每一项 键 和 值 的共享引用
        // artist 从 String -> &String
        // works 也从 Vec<String> -> &Vec<String>
        println!("works by {}:", artist);
        for work in works {
            println!(" {}", work);
        }
    }
}

// show方法可以使用共享引用, 但如果此时要对 HashMap进行排序, 就不行了， 因为涉及到了对HashMap的修改
// 这里需要使用 Table 的 可修改引用
fn sort_works(table: &mut Table) {
    for (_artist, works) in table {
        works.sort();
    }
}

// * 引用作为值
fn reference_for_value() {
    // 在Rust中, 引用是通过 & 操作符显示创建的, 而解引用也要显示使用 * 操作符
    let x = 10;
    let r = &x; // 创建x的引用 r
    let res = *r == 10; // true
    println!("x == r: {}", res); // 解引用并对比, *&x == x

   // 可修改引用
    let mut y = 32;
    let m = &mut y; // 创建y 的可变引用
    *m += 32; // m解引用后修改值, 其实就是在修改y的值
    // ! 注意, 这里不能使用y, 因为创建可变引用后, 不能同时拥有对该值的任何其他引用, 否则可能会导致出现悬垂引用
    assert_eq!(*m, 64); // 它们是相等的

    // ? 不过对于前面的 show 函数，在传递引用而不是值之后， 并没有使用 * 操作符来解引用
    // ? 这个主要是由于 .操作符的特性, .操作符会在必要时对其左操作数进行隐式解引用
    struct Anime { name: &'static str, bechdel_pass: bool };
    let aria = Anime { name: "Aria: The Animation", bechdel_pass: true };
    let anime_ref = &aria;
    // .操作符会自动解引用, 因此我们可以通过 &Anime.name 访问到 name真实的类型, 也就是 &str, 同时还包含了他的生命周期
    assert_eq!(anime_ref.name, "Aria: The Animation");
    // 上述代码等价于: assert_eq!((*anime_ref).name, "Aria: The Animation")；

    // 除了自动解引用外, .操作符 在访问一个结构体方法时, 还可以隐式的对左操作数进行借用
    let mut v = vec![1873, 1939];
    v.sort(); // 这里会自动产生了对 v 的可修改借用, 因此在调用完成sort() 方法后， 变量v还可以继续使用
    // 这个也是结构体方法第一个默认参数是 &Self的原因
    // 上述代码等价于 (&mut v).sort();
}

// 
fn reference_change_address(b: bool) {
    // 给Rust引用赋值会导致它指向新的地址
    // 这一点和C系语法完全不同， 所有C系语言, 引用一经赋值后, 不能将这个引用指向初始值之外的任何其他地址
    let x = 10;
    let y = 20;
    let mut r = &x;
    // 如果条件成立, r的地址将变为 &y, 此时引用地址已发生变化
    if b { r = &y };
    assert!(*r == 10 || *r == 20);
}

fn infinite_reference() {
    // Rust允许引用的引用， 并且无论多少层引用， .操作符都能找到最终值
    struct Point { x: i32, y: i32 };
    let point = Point { x: 100, y: 200 };
    let r: &Point = &point;
    let rr: &&Point = &r;
    let rrr: &&&Point = &rr;
    // 无论堆叠多少层引用, 都可以通过.解引用 并找到原始值
    assert!(rrr.x == 100);

    // 除了 .操作符以外， Rust的比较操作符也能够看穿无限层级, 抵达最原始的值
    let x = 10;
    let y = 10;
    let rx = &x;
    let ry = &y;
    let rrx = &rx;
    let rry = &ry;
    assert!(rrx == rry);

    // 如果一定要对比两个指针的地址是否相等, 可以使用 std::ptr::eq
    println!("利用 std::ptr::eq 对比指针地址： {}", std::ptr::eq(*rrx, rx));

    let x: &i32 = &0;
    println!("x现在是： {}", x);

    // Rust的引用永远不会为空, 引用没有默认初始值, 在初始化之前任何值都不能使用。
    // 不过在机器级别, Rust将 Options<&T>::None作为空指针使用, 将 Some(&T)作为非空指针, 因此Option<&T>完全可以像C系列的空指针一样有效使用
    // 只是在使用之前要求开发人员必须检测它是否为 None
}


// 借用对任意表达式的引用
// 和C与C++只允许对某些类型的表达式应用 &操作符 不同, Rust允许借用对任何类型表达式的值的引用
fn factorial(n: usize) -> usize {
    (1..n + 1).fold(1, |a, b| a * b)
}
fn get_all_reference() {
    // 类似这种情况, Rust会创建一个匿名变量来保存表达式的值, 然后生成一个指向该值的引用
    // 这个变量的生命周期取决于开发人员要对这个变量做什么处理
    // - 如果在let语句中立即把这个引用赋给一个变量(或将其变成立即被赋值的结构体或数组的一部分), 那Rust会让这个匿名变量具有与let初始化的变量一样长的生命期。
    // - 比如下面的r, 会一直引用对应的匿名变量
    // - 否则匿名变量会存活至闭合语句的末尾。
    // ? 如果有超出匿名变量生命期且永远不会用到的引用存在, Rust一定会在编译时发现并报告它。只要把匿名变量的值保存到一个命名变量中并给他一个适当的生命周期即可
    let r = &factorial(6);
    // ? 比如这里的用于保存 1009 的匿名变量会持续到 assert_eq 语句结尾
    assert_eq!(r + &1009, 1729);

    // ? Rust有两种 "胖指针", 也就是包含某个值的地址以及与使用该值相关的必要信息的一个两个字的值
    // * 对切片的引用是一个胖指针, 包含切片地址及其长度信息
    // * Rust的另一种胖指针是特型对象(trait object), 也就是对 Trait 的引用, 包含一个值的地址和一个指向该值匹配的Trait实现的指针, 以便于调用 Trait 的方法
    // * 除此之外, 切片和Trait的引用与其他的引用没有什么不同, 它们都不拥有自己指向的值, 他们的生命周期都不能超出目标值, 它们可以是可修改的或可共享的等
}

pub fn reference_fn() {
    // 建表
    let mut table: HashMap<String, Vec<String>> = Table::new();
    table.insert("Gesualdo".to_string(), vec!["many madrigals".to_string(), String::from("Tenebrae Responsoria")]);
    table.insert("Garavaggio".to_string(), vec!["Musicians".to_string(), String::from("The Calling of St. Matthew")]);
    table.insert("Cellini".to_string(), vec!["Perseus with the head of Medusa".to_string(), String::from("a salt cellar")]);
    // show(table);
    // 如果直接將table传入show, 那么到此处就不能使用table了, 因为table的所有权通过参数传入了函数show
    // 同时在show执行完毕后, 指针table也随着栈帧的回收而被Drop了
    // 如果想要继续使用table, 那么show函数应该接收table的引用, 而不是table本身
    show(&table);
    assert_eq!(table["Gesualdo"][0], "many madrigals");

    // 正确的做法就是如上所示使用引用。通过引用可以访问值, 又不会影响其所有权。
    // ? 引用分两种情况:
    // + 共享引用
    // -   可以读取引用的值，但不能修改。不过可以拥有对某个特定值的任意多个共享引用。
    // -   表达式 &e 会产生一个对 e 的值的共享引用。 
    // -   如果e的类型是T, 那么 &e 的类型就是 &T。 
    // -   共享引用是 Copy

    // + 可修改引用
    // -   可以读取和修改引用的值
    // -   不能同时拥有对该值的任何其他引用。
    // -   表达式 &mut e 会产生一个对 e 的可修改应用, 类型为 &mut T
    // -   可修改引用不是 Copy

    // ? 可以将共享引用和可修改引用的区别理解为它们会在编译时分别执行 多读(multiple readers) 和 单写(single writer)的检查规则
    // ? 上述规则不仅仅适用于引用, 也适用于被借用值的所有者
    // ? 只要存在对某个值的共享引用, 即便是该值的所有者也不能修改它。
    // ? 此时，这个值已经被锁定了
    // ? 比如在 上述show 执行期间, 没有人可以修改table(直接避免多线程场景下的数据竞争和上锁问题)
    
    // ? 类似的, 对于可修改引用, 该引用对这个值拥有 "排他读写权"。 
    // ? 在这个可修改引用存续期间， 连它的所有者都不能使用它

    println!("------------------------------------------------------------------------------------");
    // 排序
    sort_works(&mut table);
    show(&table);

    println!("------------------------------------------------------------------------------------");
    reference_for_value();

    println!("------------------------------------------------------------------------------------");

    reference_change_address(true);
    reference_change_address(false);

    println!("------------------------------------------------------------------------------------");
    infinite_reference();

    get_all_reference();

    println!("------------------------------------------------------------------------------------");
    // println!("this is reference");

    reference_safe();

    println!("------------------------------------------引用作为参数传递------------------------------------------");
    reference_to_params();
    println!("------------------------------------------引用作为参数传递------------------------------------------");

    println!("------------------------------------------引用作为返回值返回------------------------------------------");
    return_reference();
    println!("------------------------------------------引用作为返回值返回------------------------------------------");

    println!("------------------------------------------结构体包含引用------------------------------------------");
    struct_with_reference();
    println!("------------------------------------------结构体包含引用------------------------------------------");

    println!("------------------------------------------不同lifetime参数------------------------------------------");
    lifetime_params();
    println!("------------------------------------------不同lifetime参数------------------------------------------");

    println!("------------------------------------------省略lifetime参数------------------------------------------");
    to_omit_lifetime_params();
    println!("------------------------------------------省略lifetime参数------------------------------------------");

    println!("------------------------------------------共享与修改------------------------------------------");
    share_and_edit();
    println!("------------------------------------------共享与修改------------------------------------------");
    // 引用作为参数传递
}