// 所有权
// rust核心功能之一, 就是所有权(ownership)
// 该 "特性" 对rust编程有着深刻的影响

/**
* 程序管理内存有多种方式:
* + 一些语言具有垃圾回收机制, 在程序运行时有规律的寻找不在使用的内存.(js, python等)
* + 在另一些语言中, 需要开发人员手动释放和分配内存(C语言)
* ! Rust选择了另外一条路: 通过所有权管理内存
*   - 编译器在编译时会根据一系列规则进行检查, 违反任何规则程序都不能通过编译. 在运行时, 所有权系统的任何功能都不会减慢系统运行
* ! 对所有权规则的掌握, 是编写Rust高效和安全代码的基础
*/

/**
 * Rust堆和栈
 * 堆和栈都是代码运行时可供使用的内存, 但是结构不同.
 * 栈中所有数据都必须占据已知且固定的大小. 在编译时大小未知或大小可能变化的数据, 需要更改为存储在堆上.
 *
 * 堆是缺乏组织的: 当向堆中增加数据时, 需要请求一定大小的空间.
 * 内存分配器(memory allocator)在堆的某处找到一块足够大的空位, 并将其标记为已使用, 并返回一个表示该位置地址的 指针(Pointer).
 * 上述过程被称为: 在堆上分配内存(allocating on the heap), 也叫做 "分配(allocating)"
 * ! 注: 将数据推入栈中并不被认为是"分配"
 *
 * 因为指向放入堆中数据的指针是已知的并且大小是固定的, 可以将该指针存储在栈上, 不过需要实际数据时, 必须访问指针.
 *
 * 入栈比堆上分配内存更快, 因为入栈时分配器无需为存储数据搜索内存空间, 位置也总是在栈顶. 相比之前, 在堆上分配内存需要更多的操作
 * 不仅是要找到一块足够的空间存放数据, 还需要做一些记录为下一次分配做准备.
 *
 * 访问堆上数据也比访问栈中数据更慢, 因为需要通过指针进行访问. 现代处理器在内存中跳转越少就越快(缓存)
 *
 * 当代码调用一个函数时，传递给函数的值（包括可能指向堆上数据的指针）和函数的局部变量被压入栈中。当函数结束时，这些值被移出栈。(和C++一样)
 *
 * 跟踪哪部分代码正在使用堆上的哪些数据, 最大限度的减少堆上的重复数据数量, 清理堆上不再使用的数据确保不会耗尽空间, 这些问题正是所有权系统要处理的.
 *
 *
 */

/**
 * 所有权规则:
 * + Rust中每个值都有一个所有者
 * + 值在任一时刻有且只有一个所有者
 * + 当所有者(变量)离开作用域, 这个值将被丢弃
 */

// 变量作用域
pub fn variables_scope() {
    // 作用域是一个项（item）在程序中有效的范围
    // 假设如下变量
    let s = "hello";
    // 变量s绑定了一个字符串的值, 这个字符串值是通过硬编码进程序代码中的. 这个变量从声明的点开始直到当前`作用域`结束时都是有效的
    // 比如下面的声明方式
    {
        // s在当前块级作用域中暂时无效, 因为还没有被声明
        let s = "hello2"; // 从这里开始, s已经有效
                          // 使用s
        println!("s是这个东西: {s}");
    } // 作用域结束, s不再生效(和c++类似)
      // 有两个重要的时间点:
      // + 当 s 进入作用域 时, 它是有效的
      // + 这一直持续到它 离开作用域 为止

    // 到上面为止, 变量和作用域的关系, 和其他语言是类似的
}

// String类型 变量作用域
pub fn variables_scope_string() {
    // 前面所有的类型都过于简单, 都是存储在栈中的数据, 离开作用域则出栈
    // 如果代码的另一部分需要在不同的作用域中使用相同的值, 可以快速简单的复制来创建一个新的独立实例
    // 因此我们需要找到一个存储在堆上的数据来探索Rust如何直到该在何时清理数据.

    // 前面说过字符串字面值, 也就是被硬编码进入程序的字符串值
    // 字符串字面值很方便, 不过他们并不适合使用文本的每一种场景
    // 主要原因在于: 1. 他们都是不可变的 2. 并非所有字符串的值都能够在编写代码时就知道, 比如获取用户输入的值

    // 为此, Rust出现了第二种字符串类型: String
    // 这个类型用于管理被分配到 堆 上的数据, 能够存储在编译时未知大小的文本
    // 可以使用 from 函数基于字符串字面值来创建 String, 如下:
    let mut s = String::from("草拟吗"); // ? 这里的String是一个模块命名空间, from是其中实现的一个方法
    s.push_str(", 狗东西!"); // push_str() 在字符串后追加字面值
    println!("{}", s); // 打印结果: 草拟吗, 狗东西!
}
/**
 * 为什么String可变, 但是 &str 不行?
 * 主要区别在于对内存的处理上
 * 
 * 对于字符串字面值(&str), 在编译时就知道其内容, 所以文本被直接硬编码进最终的可执行文件中. 这使得字符串字面值快速且高效.
 * 不过这些都是因为字符串字面值是不可变的.
 * 我们并不能为了每一个在编译时大小未知的文本而将一块内存放入二进制文件中, 并且它的大小还可能随着程序运行而改变
 * 
 * 对于 String 类型, 为了支持一个可变, 可增长的文本片段, 需要在堆上分配一块在编译时未知大小的内存来存放内容, 意味着:
 * + 必须在运行时向内存分配器(memory allocator)请求内存
 * + 需要一个当我们处理完 String 时将内存返回给分配器的方法
 * 
 * 第一步就是调用 `from` 方法
 * 第二步在有GC的语言中, GC检查变量是否不再使用, 进一步对其进行回收. 而对于C这类语言, 则是需要开发人员主动完成析构函数, 对变量进行回收
 * 但如果忘记回收, 会造成内存浪费, 过早回收, 又会出现无效变量, 重复回收, 会产生bug
 * 我们需要精确的为一个 `allocator` 配对一个 `free`
 * 
 * Rust的内存策略: 内存在拥有它的变量离开作用域后就被自动释放.
 * ? 比如上述的例子, 在块级作用域结束后, 内部s也就被回收了, 后续也无法继续使用
 * ? 其实在离开作用域时, rust会自动调用一个 `drop` 函数, 类似c++的析构函数, 将内部变量完全回收
 * ? 这个模式和C++的 RAII(Resource Acquisition Is Initialization) 资源获取即初始化 类似
 */

// 上述资源离开作用域自动 drop 的模式虽然看起来很好, 但是在更复杂的场景下代码的行为会变得难以预测, 比如有多个变量使用在堆上分配内存时


pub fn variables_move_and_clone_on_heap() {
    // + 变量与数据交互的方式(一): 移动
    // 在 Rust 中, 多个变量可以采取不同的方式与同一数据进行交互

    // 栈存储, 没啥好说的, x和y都是栈上独立空间, 只不过他们的值都是5
    let x = 5;
    let y = x;
    
    // 堆存储
    // 看起来和上述行为非常类似, 第二行生成s1的拷贝并绑定到s2上
    // 不过, 事实并不完全是这样的
    // String由三个部分组成: 栈上指针s1(ptr), 长度(len), 容量(capacity), 堆上存放字符串的内容
    // 长度: 表示 String 的内容当前使用了多少字节的内存
    // 容量: 表示 String 从分配器总共获取了多少字节的内存
    // ! 注意: 长度和容量是不同的, 比如Vector, 可能申请了30的字节, 但实际只使用了15的字节
    let s1 = String::from("hello");
    // 在这里将s1赋值给s2时, String 的数据被复制了, 意味着我们从栈上拷贝了它的指针, 长度和容量
    // 我们并没有复制指针指向的堆上数据(类似js的对象直接拷贝)
    // rust并不会拷贝堆上数据, 否则在堆上数据较大时, 会出现极大的内存隐患(这一点, c, js等均如此)
    let s2 = s1;
    // 这里, s1和s2两个指针都指向同一个地址
    // 前面说过当变量离开作用域时, rust会自动调用drop函数, 来清理变量的内存.
    // 此时两个指针指向了同一个地址, 当s1和s2离开作用域时, 他们都会尝试释放相同的内存.
    // 因此, 这里会产生 `二次释放`的错误, 这也是之前提过的内存安全性bug之一
    // 两次释放相同内存会导致内存污染, 可能会导致潜在的安全漏洞
    
    // 解决方案: Rust解决此问题很粗暴
    // 为了保证内存安全, 当执行 let s2 = s1; 以后, Rust会认为s1不在有效, 因此Rust不需要在 s1 离开作用域后清理任何东西
    // 所以在这个位置如果使用s1, 会直接panic, 提示无效变量

    // 其实s2 = s1, 类似浅拷贝, 只不过rust还让 s1 无效了
    // 这个做法, 其实叫做 `移动(move)`, 而不是浅拷贝
    // 其实是s1指针, 整体移动到了s2
    // 这样目的就达成了, 因为只有 s2 是有效的
    // ! 这里还有一个隐藏设计, Rust永远不会自动创建数据的 "深拷贝", 因此, 任何自动的复制可以被认为对运行时性能影响较小
    
    // + 变量与数据交互的方式(二): 克隆
    // 如果不想执行移动, 也可以使用 clone, 直接进行克隆, 但是克隆类似深拷贝, 相当消耗资源
    let s3 = s2.clone();
}

// Copy trait(拷贝)
pub fn variables_copy_on_heap() {
    // 对于只在栈上的数据, rust会进行拷贝.
    // 像整型这样的在编译时已知大小的类型被整个存储在栈上, 所以拷贝其实际的值是快速的.
    // 意味着没有理由在创建y后使x无效
    // 和其他语言一样, 栈上的数据, 是没有深浅拷贝区别的, 这里调用 clone 和直接拷贝, 并不会产生任何区别, 因此不必理会
    let x = 5;
    let y = x;

    // Rust有一个叫做 Copy trait 的特殊注解, 可以用在类似整型这样的存储在栈上的类型上.
    // 如果一个类型实现了 Copy trait, 那么一个旧的变量在将其赋值给其他变量后仍然可以使用
    // ! 注: Rust并不允许自身或其任何部分实现了 `Drop` trait 的类型使用 `Copy` trait .
    // ! 如果强行对其值离开作用域时需要特殊处理的类型使用 Copy 注解, 将会编译错误
    // ! Copy 与 Drop 不能共存
    // ! copy 的行为是隐式行为，开发者不能重载 Copy 行为，它永远都是一个简单的位复制。

    // 任何一组简单标量值的组合都可以实现 Copy, 任何不需要分配内存或某种形式资源的类型都也可以实现 Copy, 如下:
    // + 所有整数类型
    // + bool类型
    // + 所有浮点数类型
    // + 字符类型
    // + 元组(不过需要内部所包含的所有类型都实现了Copy) 
    // + struct 也可以实现Copy trait


}

// 函数所有权与Copy trait
pub fn ownership_in_func() {
    // 变量所有权进入函数会发生移动或者复制, 和直接赋值一样, 对实现了 Copy trait的变量会发生复制
    // 存储在堆地址上的变量则是所有权移动
    // * 理解 Copy trait
    // s在栈上, 是一个指针, hello在堆上, 是对应的值
    let s = String::from("hello");
    println!("addr of data 'hello' = {:p} on heap", s.as_ptr()); // 指向堆中存储值 hello 的地址
    println!("addr of 's' = {:p} on stack", &s); // 栈上的指针s的地址
    // s移动到函数func1中
    takes_ownership(s);
    // 此时s已经失效
    println!("------------------------------------------------------------------------------
        ----------");
    let x = 10;
    println!("x={x}");
    println!("addr of x = {:p}", &x);
    makes_copy(x);
    // x 默认实现了 Copy trait, 后续可以继续使用, 不会发生所有权的转移
    // 本质上 Copy trait 是重新分配了一块内存, 然后将数据直接 copy 进了那块栈内存中
    println!("after func2 x={x}");
    // 可以看到, 函数func2中的地址, 和外部x的地址是不同的
    // 说明x入参后, 调用了 copy trait中的实现, 开辟了新的地址, 传入函数内部
    // 在函数内部, drop掉的也是新的地址, 而不会影响外部x
    println!("after func2 addr of x = {:p}", &x);
} // 这里，x 先移出了作用域，然后是 s。但因为 s 的值已被移动到函数 takes_ownership 中

fn takes_ownership(some_string: String) {
    println!("in func1, some_thing={some_string}");
    println!("in func1, addr of some_string={:p} on stack", &some_string);
    // 堆上对应同一个地址, 但是栈上不是, 这里发生了所有权移动, 栈上开辟了新的空间
    println!("in func1, addr of some_string={:p} on heap", some_string.as_ptr());
} // some_string 移出作用域并调用 drop方法

fn makes_copy(some_integer: i32) {
    println!("in func2, some_integer={some_integer}");
    println!("in func2, addr of some_integer={:p}", &some_integer);
} // some_integer 移出作用域, 出栈

// 返回值与作用域
pub fn scope_return_val() {
    let s1 = gives_ownership(); // gives_ownership 将返回值的所有权move给s1

    let s2 = String::from("hello"); // s2进入作用域
    println!("s2堆地址: {:p}", s2.as_ptr());
    println!("s2栈地址: {:p}", &s2);

    let s3 = takes_and_gives_back(s2); // s2 所有权被移动到了 takes_and_gives_back 中
    println!("s3堆地址: {:p}", s3.as_ptr()); // s3和s2堆地址相同
    println!("s3栈地址: {:p}", &s3); // 栈地址不同, 所有权发生移动, 指针内存空间往栈顶移动
    // 同时将返回值move到s3
} // s3被移出作用域同时被丢弃, s2移出作用域, 但s2已转移, 不会drop
// s1离开作用域同时被丢弃

fn gives_ownership() -> String {
    let some_string = String::from("yours"); // some_string 进入作用域
    some_string // 返回 some_string, 同时所有权被移动给调用它的函数
}

/**
 * 变量的所有权总是遵循相同的模式: 将值赋值给另一个变量时移动它。当持有堆中数据值的变量离开作用域时, 其值将通过drop被清理掉,
 * 除非数据被移动为另一个变量所有。
 * 在每个函数中都获取所有权并接着返回所有权有些多余, 应该使用更简单的方式
 * 这个方式就是 `引用(references)`
 */

// 传入字符串, 同时返回该字符串
fn takes_and_gives_back(a_string: String) -> String {
    a_string // 返回a_string, 并将所有权移动给调用的函数
}
