/**
* Slice 类型
* slice 允许引用集合中一段连续的元素序列, 而不用引用整个集合。
* slice 是一类引用, 因此它没有所有权
*/
pub fn slice_references() {
    // 为什么需要 字符串slice
    // slice_first();
    // 字符串slice
    // string_slice_what();
    // 字符串字面值
    string_literal();
}


fn slice_first() {
    let mut s1 = String::from("wo cao");
    let idx = first_word(&s1);
    println!("空格的位置是: {idx}");
    // 这里通过 first_word 返回了一个独立的 usize
    // 但是这个 usize 只在 &String 上下文中才是一个有意义的数字
    // 因为它是一个与String相分离的值, 无法保证将来它依然有效
    // 比如现在清除s1
    s1.clear();
    // 前面计算的idx是一个全新的栈空间, 他并不是和 s1 相关联的, 此时就会出问题, idx还是2, 但是s1已经被清空了
    // 虽然通过了编译, 但是产生了bug, 我们需要的应该是idx跟随s1进行响应式变化
    // rust的解决方案是使用 `字符串slice`
    println!("{s1}中空格的位置是: {idx}");
}

/**
* @return usize 参数s的一个字节索引值
*/
fn first_word(s: &String) -> usize {
    // 这里有一个 first_word 函数
    // 只有一个参数 &String, 因为不需要所有权, 所以这里没有问题
    // 但是返回值需要考虑, 因为缺少一种获取 部分 字符串的办法
    // 不过, 我们可以返回单词结尾的索引, 结尾由一个空格表示

    // 由于需要逐个元素的检查 String 中的值是否为空格, 需要用 as_bytes 方法将 String 转化为字节数组
    let bytes = s.as_bytes();
    // 使用 iter 方法在字节数组上创建一个迭代器, iter包装结果, enumerate 将结果包装为一个元组
    for (i, &item) in bytes.iter().enumerate() {
        // 字面值 b' ' 表示代表空格的字节, b表示 byte, 对 ' ' 进行转字节操作
        // item是每一个字节， 字节和字节进行对比
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

fn string_slice_what() {
    // 字符串slice 是 String中一部分值的引用, 看起来像这样的:
    let mut s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];
    println!("hello is {hello}");
    println!("world is {world}");

    // 不同于整个String的引用, 上面的 hello和world, 都是对 s 的部分引用
    // 中括号中表示: [starting_index..ending_index]
    // 通过 &String[starting_index..ending_index] 指定 range 创建一个 slice
    // 这个范围是从 start 到 end - 1, 长度就是 end - start

    // 如果索引从0开始, 可以不写 start
    let slice = &s[..5];
    println!("slice start is {slice}");

    // 索引包含最后一个字符, 可以不写 end
    let slice = &s[6..];
    println!("slice end is {slice}");


    let mut s = String::from("hello world!");
    let s1 = first_word_re(&s);
    println!("the first word in s is {s1}");
    // s.clear();
    // 上述操作会直接报错, 因为 s1 已经计算出结果, 并且结果依赖s, 因此s不能被回收
    // 因为编译器要确保指向 s 的引用持续有效, 不能改变
    // println!("the first word in s1 is {s1}");
    let byte = b' ';
    println!("空格的字节是: {byte}");
}

// 重写 first_word 返回一个 slice
fn first_word_re(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            // 查找到空格之前
            return &s[0..i];
        }
    }
    // 无空格
    &s[..]
}

fn string_literal() {
    // 字符串字面值
    // 之前说过, 字符串字面值被存储在二进制文件中
    // 这里的 s 类型是 &str, 它是一个指向二进制程序特定位置的 slice
    // 这也是为什么 字符串字面值
    let s = "hello world!";
    array_slice();
}

// 除了字符串以外, 其他类型也有 slice
// 比如数组
fn array_slice() {
    let a = [1, 2, 3, 4, 5, 6];
    let slice_arr = &a[1..3];
    // 可以发现他们是一样的, 均为数组a的一部分
    assert_eq!(slice_arr, &a[1..3]);
}


