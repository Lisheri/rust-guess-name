// 为 User 实现 Clone trait, 这样就可以使用 clone 了
#[derive(Debug, Clone)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// 方法
impl User {
    // &self 是 self: &Self 缩写
    fn get_name(&self) -> &str {
        // 这里要用 slice, 否则所有权丢失
        &self.username[..]
    }
}

// 元组结构体
#[derive(Debug)]
struct Point(i32, i32, i32);
// 单元结构体
#[derive(Debug)]
struct AlwaysEqual;

pub fn structs_main() {
    /*
    println!("结构体");
    // 实例化结构体, 有点类似其他语言类的实例化
    let mut user1 = User {
        active: true,
        username: String::from("lisher"),
        email: String::from("123123123@qq.com"),
        sign_in_count: 1,
    };
    // 修改字段值
    user1.email = String::from("123123111@qq.com");
    let user2 = User {
        // 使用旧实例的字段创建新的实例
        sign_in_count: user1.sign_in_count + 1,
        // 剩余操作符, 必须放最后, 指定其他字段来自user1
        // 这里如果不clone, 因为 user1中的字段已经移动到了user2中, 后续不能使用user1了
        // ? 如果需要 clone, 需要先实现 clone
        ..user1.clone()
    };
    // :#? 中的 # 标识换行打印结构体
    println!("user2: {:#?}", user2);
    println!("user1.name: {:#?}", user1.get_name());
    println!("user1.name: {:#?}", user1.username);
    let empty_struct = AlwaysEqual {};
    println!("always_equal: {:#?}", empty_struct);
    let point = Point(10, 11, 12);
    println!("point.1: {:#?}", point.1);
    */

    // * 结构体函数和结构体方法
    // let rectangle = Rectangle {
    //     width: 30,
    //     height: 40
    // };
    // 这里会报错, 因为缺少一个 Display trait, println!宏中的 "{}" 就是基于 Display trait 实现的占位输出
    // 结构体并没有原生提供一个 Display trait, 但是可以增加派生属性, 也就是#[derive(Debug)], 这样就可以对结构体进行打印了
    // 可以使用 {:?}, 也可以使用 {:#?}, 推荐后者, 结构更加规整

    // dbg!宏 和 println!宏 不太一样, dbg!宏接收一个表达式的所有权, 而 println! 接收的是引用
    // 同时, 前者打印到 标准错误控制台流(stderr), 后者打印到 标准输出控制台流(stdout)
    // println!("rect1 is {:#?}", rectangle);
    // 由于我们不希望 dbg! 获得表达式所有权, 因此这里选择传递一个引用
    // dbg!(&rectangle);
    // let area_func = area_with_struct(&rectangle);
    // println!("area_func: {:?}", area_func);
    // let area_fn_in_struct = rectangle.area_in_struct();
    // println!("area_fn_in_struct: {:?}", area_fn_in_struct);

    // 方法
    func_main();
}

// 一般函数使用
fn area(width: u32, height: u32) -> u32 {
    width * height
}

#[derive(Debug, Clone)]
struct Rectangle {
    width: u32,
    height: u32
}
// 结构体重构上述area函数
fn area_with_struct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

// 直接使用结构体方法
// ? impl 是 implementation 的缩写, 和 struct 同名的情况下会自动关联
impl Rectangle {
    // 结构体方法会自动获取 self引用, 指向当前结构体实例
    // 这个个 &self 实际上是 self: &Self 缩写
    // 方法的第一个参数必须有一个名为 self 的 Self 类型的参数
    // 而 Self 是 struct, 因此需要声明为对 实例的 借用, 也可以选择直接获取所有权 self: Self
    fn area_in_struct(&self) -> u32 {
        &self.width * &self.height
    }

    // 接收一个other, 也就是其他Rectangle实例引用作为参数
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // 关联函数
    // 与impl后面明明的 struct 相关, 但是可以定义不以 self 为第一个参数的关联函数(非方法)
    // 比如 String::from 就是一个关联函数
    // 不是方法的 关联函数 经常被用作返回一个结构体新实例的构造函数。
    // 一般被称为 new, 但是 Rust 并没有 new 关键字
    fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
        }
    }

    // 创建一个正方形
    fn square(size: u32) -> Self {
        Self::new(size, size)
    }
}
// 可以定义多个impl块, 后续会继续讨论

// 使用函数构造结构体实例, 类似构造函数
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        // 简写
        username,
        email,
        sign_in_count: 1,
    }
}

fn func_main() {
    // rust 会自动引用, 比如
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };
    // 下面两者是一样的, rust会自动添加引用
    println!("width1: {}", rect1.area_in_struct());
    println!("width2: {}", &rect1.area_in_struct());
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    // 主要原因在于方法明确声明了是获取所有权还是获取所有权的引用

    let sq = Rectangle::square(3);
    println!("sq 是: {:#?}", sq);
}

