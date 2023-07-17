// 引用安全
pub fn reference_safe() {
    // Rust控制引用安全核心还是在于利用引用规则
    // ? 如下代码是不能通过编译的
    // let r; // 变量r创建并产生其自己的生命周期(始于函数执行到此处, 终于函数执行结束)
    // {
    // let x = 1; // * 变量x创建并产生其自己的生命周期(终止于块级作用域结束)
    // r = &x; // * 此处产生引用 &x 的生命周期, 其生命周期和x保持一致
    // * 当x生命周期结束时 &x也必须结束, Rust一定不允许产生 "悬垂引用"
    // * 借用并保存 &x的生命周期保存在变量r上, 因此变量r的生命周期在上诉语句执行后就和 &x 绑定了

    // ? 这里有一个显而易见的约束: 引用 不可能比 其引用变量本身 更加 "长寿"
    // ? 同时还存在另外一个约束: 保存在变量r中的引用必须, 其类型必须保证他在变量的整个生命周期中都有效
    // * 如果引用不能与保存它的变量一样长寿, 那么保存它的变量迟早会变成悬垂引用

    // 执行到此处后, 变量x已经被回收
    // 但是函数作用域变量r 还可以继续使用, 也就是所谓的指针的生命周期长于其引用的值
    // 意味着离开当前块级作用域后, r将变成 "悬垂引用"
    // }
    // assert_eq!(*r, 1);
    // 因此上述代码编译时, Rust抱怨的是: x只存活到内部块的末尾, 引用却存活到了外部块的末尾, 变成了悬垂引用

    // 上述代码的核心并不在于人为理解这个含义, 而是要落到Rust如何得出这个结论的

    // + 生命周期
    // rust会给程序中的每一个引用类型附加一个生命周期, 它的长短与如何使用该引用匹配
    // 生命周期是程序中可以安全使用引用的范围
    // lifetime 仅用于编译时检查, 是Rust虚构的产物, 而在运行时, 引用只是一段内存地址, 其lifetime 取决于自身类型

    // - 1. 变量的生命周期必须 包含 或 涵盖 从它那里借来的引用的生命周期(限制引用的生命周期有多长)
    // - 2. 引用的生命周期必须 包含 或 涵盖 保存它变量的生命周期(限制引用的生命周期有多短)

    // 要同时满足上述情况, 需要使用如下方式:
    let r;
    {
        let x = 1;
        r = &x;
        assert_eq!(*r, 1);
    }

    // 在从较大数据结构中借用某个部分的引用时, 这些规则会非常自然的应用, 比如借用向量中的元素:
    let v = vec![1, 2, 3];
    let r: &i32 = &v[1];
    // 由于v拥有向量, 而向量拥有元素, 因此v的生命期必须涵盖引用类型&v[1]的生命期

    // 接收引用作为参数
    static mut STASH: &i32 = &128; // 静态变量必须在创建的同时进行初始化
                                   // 如果要给全局静态变量赋值, 那么我们必须为函数声明lifetime
                                   // ? 下面的 <'a>表示对于任意生命期a, 而 &'a 表示函数接收一个具有任意lifetime的 i32 引用
                                   // fn f<'a>(p: &'a i32) {
                                   // ! 必须将参数lifetime 修改为静态lifetime, 涵盖整个程序执行
    fn f(p: &'static i32) {
        // 可修改静态变量本质上不是线程安全的, 因此需要增加unsafe声明
        unsafe {
            // 由于静态变量的lifetime和程序执行过程一样长, 因此它保存的引用生命周期也必须和程序执行过程一样长才行
            // 因此上述函数定义参数lifetime就存在问题, 因为 'a 为任意lifetime, 它只需要涵盖函数执行过程即可
            STASH = p;
        }
    }

    // 只是经过上述修改后的f, 入参只能是静态变量了
    static WORTH_POINTING_AT: i32 = 1000;
    f(&WORTH_POINTING_AT);

    // 在上述函数f定义完成其签名的lifetime后, Rust仅通过参数签名就可以知道f能对自己的参数做什么, 不能做什么。
    // 比如最初使用 f<'a> (p: &'a i32) 时, 无需关注函数使用, 只需要观察函数签名就可以发现, 参数p是不能赋值给可变静态变量的
}

// + 将引用作为参数传递
// 前面讨论了 函数签名 和 函数体 的关系, 下面就是 函数 和 调用者的关系
pub fn reference_to_params() {
    // 假设有如下代码
    fn g<'a>(p: &'a i32) {}
    let x = 10;
    g(&10);

    // 但从g的签名来看, Rust知道它不会把p保存到超出调用生命期的变量里: 任何涵盖调用的生命期都满足 'a
    // 因此Rust在此为 &x 选择了尽可能短的lifetime: 对g的调用
    // 这个lifetime满足所有约束条件: 没有超出x且涵盖对g的全部调用

    // ! 注意:
    // ! 尽管g的定义中包含lifetime参数 'a , 但调用g时并不会管它。
    // ! 一般来说, 只需要在定义函数和类型时候考虑lifetime, 而在使用时, Rust会为开发人员推断lifetime
}

// + 返回引用
pub fn return_reference() {
    // 函数中取得某个数据结构的引用, 然后返回对该结构某一部分的引用, 这个是很常见的需求
    // 比如下面的函数会返回一个对切片中最小元素的引用：
    fn smallest(v: &[i32]) -> &i32 {
        let mut s = &v[0];
        for r in &v[1..] {
            // 遍历数组, 找到最小的值
            if *r < *s {
                s = r;
            }
        }
        s
    }
    // 这里其实是省略了lifetime参数的, 但是Rust会默认 引用参数 v, 拥有和 引用返回值s 一样的 lifetime
    // lifetime参数不省略应该如下定义:
    // fn <'a>smallest(v: &'a [i32]) -> &'a i32 {...}
    // 如果以如下方式调用smallest
    /* 
        let s;
        {
            let arr = [9, 4, 1, 0, 1, 4, 9];
            s = smallest(&arr);
        }
        assert_eq!(*s, 0); // 悬垂引用产生, 指向了已被清除的数组的元素
        根据lifetime参数推断, 参数和返回值必须具有相同的lifetime 'a
        但是在上面的调用中, arr的lifetime不可能比 参数 &arr 长
        但是 &arr 至少活的要和 s一样长
        此时lifetime不能同时满足约束, 因此Rust拒绝编译
    */
    // 但如果将s挪动到块级作用域内, 那么问题就解决了, 他们的存活时间将能够满足约束
    let arr = [9, 4, 1, 0, 1, 4, 9];
    let s = smallest(&arr);
    println!("arr中最小的值为: {}", *s);
    assert_eq!(*s, 0);
}


// + 结构体包含引用
pub fn struct_with_reference() {
    /* struct S {
        r: &i32
    }

    let s;
    {   
        let x = 10;
        s = S { r: &x };
    }
    assert_eq!(*s.r, 10); */

    // 上述代码依然无法通过编译
    // 因为Rust对引用施加的安全约束不会因为把引用藏在了结构体中而失效
    // 实际上这些约束最终也会应用给S
    // 当引用类型出现在另一个类型的定义中时, 必须写出其lifetime, 如下(也可以使用'static, 但是这样r的引用存活时间太长了)
    struct S<'a> {
        // 创建一个带有lifetime 'a 的新S值
        // 把 &x 保存在 r 字段中时, 等于把 'a 完全限制在了 x 的lifetime内
        r: &'a i32
    }
    
    // 这样S就有了一个lifetime, 就想引用类型一样
    // 此后, 每个类型S的值都会有一个新的lifetime 'a , 它会限制开发人员使用这个值的方式。
    // 保存在r中的任何引用的lifetime最好包含 'a, 而 'a 也必须比保存 S的任何值都 "长寿"

    let x  = 10;
    // 赋值语句 s = S {...} 把这个S保存在了一个lifetime直到程序结束的变量中, 于是要求'a至少跟s的lifetime一样长。
    // 但如果向上面一样操作, 此刻Rust又遇到了跟之前一样矛盾的限制: 'a 不能比 x "长寿", 但至少要跟s一样"长寿"。于是Rust拒绝编译。因此需要放开块作用域
    let s = S { r: &x };
    assert_eq!(*s.r, 10);

    // 将带有lifetime参数的struct放到其他的结构体中
    struct T<'a> {
        // 这样依然会报错, 因为Rust不知道T中的s是否和S一样长寿
        // s: S
        // 同样需要显示给出一个lifetime, 比如给一个 static, 但是这样s就只能是整个程序运行期间都需要存在的静态变量了
        // s: S<'static>
        // 不过一般情况下, 我们会给T一个自己的lifetime参数, 比如 'a, 在将这个 'a 给S, 意味着S的存货时间和T一样长
        s: S<'a>
    }
    // 通过声明lifetime参数 'a并在s的类型中也使用该lifetime, Rust就可以将T值的lifetime与其S中所保存引用的lifetime关联起来
}

/* 
    前面展示过通过函数签名可以知道给它传什么引用。
    现在又在类型上看到了类似的情形: 通过类型的lifetime参数, 可以知道该类型是否包含具有相应(非 'static) lifetime的引用, 以及那些引用的情况如何。
    比如, 假设有一个解析函数, 其参数是一个字节切片, 它会返回保存解析结果的一个结构:
    fn parse_record<'i>(input: &'i [u8]) -> Record<'a> {}
    通过上面的函数定义, 我们不需要关注Record的实现, 就可以知道 parse_record 函数返回的结果中包含的引用一定指向我们传入的输入缓冲区, 而不会是别处(因为不是 'static, 且存活时间和 input 一样)
*/

/* 
    * Rust之所以要求包含引用的类型不能省略lifetime参数, 正是为了将这种内部行为外化表现出来。
    * Rust并非不可以给结构体中的每个引用构造lifetime参数, 从而节约掉开发人员手动编写的麻烦
    * 在早期的版本就是这样做的, 但是开发人员反馈这样比较混乱: 知道一个值从另一个值借用了什么还是很有用的, 特别是在排查错误的时候
    * Rust中所有的类型都有lifetime, 包括i32和char。大多数是简单的'static
    * 也就是说, 你想让它们的lifetime有多长就有多长, 比如一个 Vec<i32>值, 它是独立的
    * 不需要在任何特定变量超出作用域时被清除。 但是像 Vec<&'a i32>这样lifetime必须包含在 'a 中的类型, 则必须在其引用的值仍然有效时先被清除
*/

// + 不同lifetime参数
pub fn lifetime_params() {
    // 假设定义了一个结构体, 其中包含两个引用
    struct S<'a, 'b> {
        x: &'a i32,
        y: &'b i32
    }
    // 上面两个引用使用的是一样的lifetime 'a, 如果在使用时像下面一样, 则可能会出现问题:
    let x = 10;
    let r;
    {
        let y = 20;
        {
            let s = S { x: &x, y: &y };
            r = s.x;
        }
    }
    // 目前为止, 上述代码是可以通过编译的, 因为没有产生悬垂指针, y的引用保存在了s中, 但是s会先于y被销毁
    // 而 x的引用保存在了r中, 但是r也不会比x存活的更久
    // 但如果是早期编译器编译上述代码, Rust会抱怨 y 存活时间不够长
    // 可以看到我们声明S时, 传递给x和y的lifetime是一样的, 意味着 x 和 y的引用应该要一样 "长寿"
    // + Rust必须找到一个同时对 s.x 和 s.y 都合适的lifetime
    // + 赋值 r = s.x 要求 'a 涵盖 r的lifetime
    // + 以 &y 初始化 s.y, 要求 'a 的lifetime不能长过y

    // * 于是矛盾就产生了, 没有比y的作用域短却比r的作用域长的lifetime, 这两个约束不可能同时满足, 因此Rust拒绝编译
    // * 事实证明在上述案例中, S中的x和y两个引用, 不应该是一样的lifetime, 因为他们的存活时间根本不同, 只需要修改S的定义, 让两个引用分别拥有不同的lifetime即可

    // 这样s.x和s.y就有了独立的lifetime。
    // 对s.x的操作不会影响到s.y, 因此前面的约束就能够很容易的满足了: 'a可以等于r的lifetime, 而'b可以等于s的lifetime('b也可以等于y的lifetime)
    // ! 只是 Rust 倾向于选择可用的最短lifetime

    // 函数签名也有类似的作用, 比如如下签名:
    fn f<'a>(r: &'a i32, s: &'a i32) -> &'a i32 {
        r // 但是这样有些严格, 因为没有使用的s, 也要限制和r一样的lifetime
    }

    // 换成如下方式
    fn f2<'a, 'b>(r: &'a i32, s: &'b i32) -> &'a i32 {
        r // 宽松多了
    }

    // 不过放宽也有不好的地方, 过多的lifetime参数会导致函数变得难以阅读
    // 尽量采用简单的lifetime, 如果编译不通过再修改也是个不错的方式
}

// + 省略lifetime参数
pub fn to_omit_lifetime_params() {
    // 其实在正常开发过程中, 大部分情况下是不需要显示声明lifetime参数的
    // 但无论我们是否显示声明lifetime, 它其实都存在
    // Rust会在lifetime明显合理的情况下自动进行推断, 允许开发人员省略lifetime参数

    // + 1. 比如下面最简单的情况, 函数不返回任何引用(或者其他需要lifetime参数的类型)
    // 那么永远都不需要显示声明lifetime参数
    struct S<'a, 'b> {
        x: &'a i32,
        y: &'b i32
    }

    fn sum_r_xy(r: &i32, s: S) -> i32 {
        r + s.x + s.y
    }

    // 上面的函数签名如果完整的写出来: fn sum_r_xy<'a, 'b, 'c>(r: &'a i32, s: S<'b, 'c>) -> i32
    // 如果函数确实要返回引用或其他带有lifetime参数的类型, 那么Rust也尽量会让消除歧义的过程简化
    // + 2. 如果函数参数中只出现了一个lifetime, 那Rust会假定返回值中的lifetime都是该lifetime, 如下
    fn first_third(point: &[i32; 3]) -> (&i32, &i32) {
        (&point[0], &point[2])
    }

    // 这个函数包含所有lifetime参数签名如下:
    // fn first_third<'a>(point: &'a [i32, 3]) -> (&'a i32, &'a i32)
    // 正常情况下, 如果函参数中又多个lifetime
    // 那么自然没有理由认为哪一个lifetime更适合返回值, 此时Rust会要求开发人员自己写清楚

    // + 3. 最后一种简写形式, 那就是如果函数是某个struct的方法, 而它又接收引用形式的self参数
    // 此时平衡就打破了: Rust会假定self的lifetime就是返回值需要的lifetime
    // self参数引用的是调用当前方法的值, 相当于C++、java、js等语言类的成员函数中的this
    // 如下:
    struct StringTable {
        elements: Vec<String>,
    }
    impl StringTable {
        fn find_by_prefix(&self, prefix: &str) -> Option<&String> {
            for i in 0..self.elements.len() {
                // elements中存在一个元素以prefix开始, 则返回这个元素
                if self.elements[i].starts_with(prefix) {
                    // ? 这里不是给self加借用标识&, 而是给 elements[i] 这个目标元素加借用标识&
                    return Some(&self.elements[i]);
                }
            }
            None
        }
    }

    // 上述方法的完整签名为: fn find_by_prefix<'a, 'b>(&'a self, prefix: &'b str) -> Option<&'a String>
    // Rust假定无论借用的是什么, 都是从self中借用的, 因此可以省略
    // 不过说到底, 简写不过是降低使用门槛, 如果简写不符合预期, 还是需要开发人员手动声明lifetime(建议都加上, 便于后期维护)
}

