// 使用 mem::replace 避免无意义的 clone
// mem::replace() 允许我们换出一个值，把一个什么东西留在原来的地方
// 本质上 mem::replace(&mut a1, a2) 就是将 a2的值给了a1, 然后将原本的a1给了函数返回值(所谓的原地)
use std::mem::{replace};
use num::bigint::BigUint;
use num::traits::{One, Zero};
use std::ptr;

/*
    假设我们有个 &mut MyEnum，它有两个变体，A {name: String, x: u8} 和 B {name: String}。
    现在我们想在 x 是 0 的时候，把 MyEnum::A 改成 MyEnum::B，而且要保持 MyEnum::B 完整。
    我们可以不用克隆 name。
*/
pub fn use_replace() {
    enum MyEnum {
        A { name: String, x: u8 },
        B { name: String }
    }
    fn a_to_b(e: &mut MyEnum) {
        // 下面可变的借用了 e, 所以没有办法直接使用 "*e = ...", 因为借用检查器不允许
        // 也就是说, 对 e 的赋值, 只能发生在 `if let` 子句之外
        *e = if let MyEnum::A { ref mut name, x: 0 } = *e {
            // 这里将name取出, 然后放一个空的字符串回去
            // 注意: 这里的空字符串并没有分配内存
            // 然后构造一个新的枚举变体, 他会赋值给 *e 的结果
            // replace时的name, 对应的是 MyEnum::A中解构的name
            MyEnum::B { name: replace(name, String::new()) }
        } else {
            // 其他情况，函数立刻返回，这样就跳过了给 `*e` 的赋值。
            return;
        }
    }
}

// 同样可以用于fibonacci数列求和
pub fn fibonacci(n: usize) -> BigUint {
    let mut f0: BigUint = Zero::zero();
    let mut f1: BigUint = One::one();
    for _i in 0..n {
        // 每一次的f2就是最新的值, 也就是前两个值之和, 这里f0的所有权直接给f2
        let mut f2: BigUint = f0 + &f1;
        // 获取完最新的f2之后, f0就可以替换为当前的f1, 然后把最新的f2给f1, 用于下一次求和
        // ? replace发生了如下两个操作:
        // ? 1. replace利用 ptr.read(&mut f1)获取了f1的指针地址, 最终返回的也是f1的指针, 因此, f0获取了f1的所有权
        // ? 2. 同时 内部也利用 ptr.write(&mut f1, f2), 将f2转移给了f1
        f0 = replace(&mut f1, f2);
    }
    f0
}
