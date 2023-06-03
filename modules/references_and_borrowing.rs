/**
* * 借用与引用入口函数
*/
pub fn references_and_borrowing() {
    // 在之前的例子中, 必须将 入参 String 返回给调用函数, 以便于在函数调用后, 依然可以使用参数
    // 因为参数被移动到了函数内。
    // 相反, 可以提供一个 String 值的引用(reference)
    // `引用(reference)` 像一个指针, 因为它是一个地址。可以由此访问存储于该地址的属于其他变量的数据
    /**
    * ! 和指针不同, 引用确保指向某个特定类型的有效值
    */
    // references_calculator();

    // 可变引用案例
    change_reference();

    // 悬垂引用
    // dangling_references();
}

fn references_calculator() {
    // 下面展示一个新的 calculate_length 函数, 以接受一个 `引用` 作为参数, 而不是直接获取值的所有权
    let s1 = String::from("fuck");
    println!("addr of s1 = {:p}", s1.as_ptr()); // s1 在堆上的地址
    println!("addr of s1 = {:p} on stack", &s1); // s1 在栈上的地址
    println!("------------------------------------------------------------");
    let len = calculate_length(&s1);
    // 由于进入函数 calculate_length 的参数是s1的引用地址, 因此并没有发生所有权的转移
    // 同时返回的是一个新的值, 其所有权绑定了len上, 因此在调用完毕后, s1 和 len 均不会被Drop
    println!("The length of '{s1}' is {len}");
    // 在上述逻辑中, &s1 创建了一个指向值 s1 的引用, 但是并不拥有它。
    // 因为并不拥有这个值, 所以当引用停止使用时, 它所指向的值也不会被丢弃。
}

/**
* * 可以发现, s和s1在堆上都指向的 String fuck
* * 但是在栈上确不是, 参数进入 calculate_length 后, 就会产生新的地址, 这个地址指向了 s1在栈上的地址
* `&` 就是引用, 允许使用值, 但是不获取所有权(`引用`只读不能改)
* 有时候需要对值进行修改, 则需要使用 `解引用运算符 *`, 操作和引用相反
 */
fn calculate_length(s: &String) -> usize { // 这里的 s是String的引用
    // 注意: 这里的 s, 本身就是 s1的引用地址, 指向的就是 s1的栈地址
    // 而存储在栈上的 s1, 是一个指向String fuck堆空间的指针
    // 因此这里本质上是通过 s(s1) -> s1 -> String
    println!("addr of s = {:p}", s.as_ptr()); // s 在堆上的地址
    println!("addr of s1 = {:p} on stack", s); // s1 在栈上的地址
    println!("addr of s = {:p} on stack", &s); // s 在栈上的地址
    s.len()
} // s离开作用域, 但是它并不拥有值的所有权, 所以这里不会发生任何改变

// 这里有点绕, 变量s的作用域和函数参数的作用域是一样的, 只不过当 s 停止使用时并不会丢弃引用所指向的内存空间
// 因为 s 并不拥有对其指向空间的所有权。
// 当函数使用引用而不是实际值作为参数, 无需返回值来交还所有权, 因为就不曾拥有过所有权

// ? 将创建一个引用的行为称为 `借用`。
// 正如现实中, 如果一个人拥有某样东西, 可以从他那里借来, 但是使用完了要还回去
// 同时我们也没有对这个东西的所有权

/**
* ? 如果要修改引用对应的值, 需要使用 `可变引用(mutable reference)`
 */

fn change_reference() {
    // 只有可变的值才能够被 可变借用
    let mut s1 = String::from("hello");
    println!("s1此时是: {s1}");
    change(&mut s1);
    println!("s1此时是: {s1}");
    // ! 注意: 不能在同一时间多次借用可变变量, 会直接 panic, 主要好处是避免 `数据竞争`
    // 不同作用域内可以存在对同一个值的可变借用, 因为多个作用域一般不会同时执行(多线程场景除外)
    // 同时, 同一个值的可变和不可变借用同时存在也会导致一样的问题
    // 因为这个可变引用的存在可能会导致其他地方读取值的时候会出现意外的数据竞争, 导致值发生变化

    // 一个 `引用的作用域` 从声明的地方开始一直持续到最后一次使用为止
    // 因此在 一个引用的作用域 结束后, 就可以继续对同一个值进行可变借用了
    // 如下
    let mut s = String::from("你大爷");
    let r1 = &s;
    let r2 = &s;
    println!("r1&r2: {r1} and {r2}");
    let r3 = &mut s;
    println!("r3: {r3}")
}

fn change(some_string: &mut String) {
    some_string.push_str(", world!");
}

/**
* 在具有指针的语言中, 很容易通过释放内存时保留指向它的指针而错误的生成一个 `悬垂指针(dangling pointer)`
* 所谓悬垂指针是指其指向的内存可能已经被分配给了其他持有者。
* 相比之下, 在Rust中编译器确保引用永远不会变成悬垂状态
* 当拥有一些数据的引用, 编译器确保数据不会在其引用之前离开作用域
*/
fn dangling_references() {
    // 比如通过如下方式创建一个 悬垂引用, 编译器会直接报错
    let reference_to_nothing = dangle();
}

// fn dangle() -> &'static String {
//     let s = String::from("dangling_references");
//     // 返回s的引用, 但是s已被回收, 编译器会直接报错, 因为这是一个悬垂引用
//     &s
// }

fn dangle() -> String {
    let s = String::from("dangling_references");
    // 返回s的引用, 但是s已被回收, 编译器会直接报错, 因为这是一个悬垂引用
    s
}
