// + 1. 表达式语言
use super::system_info::{get_cpu_info, Cpu};

#[derive(Debug)]
enum CpuStatus {
    Idle,
    Busy,
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
}
