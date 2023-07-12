use std::collections::HashMap;
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

pub fn reference_fn() {
    // 建表
    let mut table = Table::new();
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
    // println!("this is reference");
}