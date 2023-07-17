// 共享与修改

fn other_null_pointer() {
    // 其他产生空指针的情况
    let v = vec![4, 8, 19, 27, 34, 10];
    let r = &v;
    // let aside = v; // 报错, Vector v所有权转移到了 aside 中, 造成 r 变为悬垂引用
    r[0];
    // 终其整个生命周期, 共享引用的目标值都是只读的
    // 不能重新给它赋值或转移该值。而在上面的代码中, r的lifetime内发生了转移向量的操作, Rust必然会拒绝的
    // 可以换成下面的方式
    let v = vec![1, 2, 3, 4];
    {
        let r = &v;
        r[0];
    }
    let aside = v;
    // 这样就不会报错了， 因为r先离开了作用域, 引用的lifetime在转移之前就结束了
}

fn slice_destroy_reference() {
    // 假设有如下函数, 这是标准库 extend_from_slice 的缩水版（缺少很多优化）
    fn extend(vec: &mut Vec<f64>, slice: &[f64]) {
        for elt in slice {
            vec.push(*elt);
        }
    }
    // 通过上述函数可以基于别的向量或数组的切片构建一个Vector
    let mut wave: Vec<f64> = Vec::new();
    let head = vec![0.0, 1.0];
    let tail = [0.0, -1.0];

    extend(&mut wave, &head); // 用另一个Vector扩展wave
    extend(&mut wave, &tail); // 用数组扩展wave
    assert_eq!(wave, vec![0.0, 1.0, 0.0, -1.0]);
    // 这样就构造了一个正弦函数周期。 
    // ? 如果想在增加一个波形， 是否可以增加向量本身?
    // 乍一看好像没啥问题
    // 但其实在给向量追加元素时, 如果缓冲区满了, 就必须分配一块更大的空间。
    // 假设wave开始时的空间能容纳4个元素， 因此在extend想添加第五个时必须分配更大的缓冲区。
    // 这个extend函数的vec参数借用了wave, 而wave又要给自己重新分配一个能容纳8个元素的缓冲区。
    // 但是slice依然指向原来4个元素的缓冲区, 该内存已被清除
    // ? 这个问题并非是Rust独有的, 在很多语言中修改集合的同时还在使用指向集合的引用很容易出问题
    // extend(&mut wave, &wave); // error
    // ? 这样的bug非常难以测试， 很可能在实际测试过程中由于Vector的长度比较长, 问题一直没有暴露出来
    // 不过Rust会报错, Rust的意思是, 可以借用Vector的可修改引用, 也可以借用对其元素的共享引用, 但是这两个引用的lifetime不能重叠
    // 但是在上述代码中, 这两个引用的lifetime都包含extend调用, 因此Rust拒绝编译

    // 其实又回到了Rust关于修改和共享的规则:
    // + 共享访问是只读访问: 共享引用借用的值是只读的。
    // - 在共享引用的整个lifetime内, 任何事物都不能修改其引用目标, 也不能修改其引用目标可触及的值。
    // - 结构中不存在指向任何目标的可修改引用, 所有者是只读的. 
    // - 实际上这个值已经冻结了
    // + 可修改访问是排他访问
    // - 可修改引用借用的值只能通过该引用访问。
    // - 在可修改引用的整个lifetime内, 没有其他路径可触及其引用目标。或触及其引用目标可触及的值
    // - 唯一能够与可修改引用的lifetime重叠的, 就是从可修改引用自身借用的引用

}

pub fn share_and_edit() {
    other_null_pointer();
}