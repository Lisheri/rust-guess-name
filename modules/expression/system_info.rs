use heim::{cpu, units::ratio, Error};
use serde::Deserialize;
use serde::Serialize;
use std::time::Duration;

#[derive(Debug, Serialize, Deserialize)]
pub struct Cpu {
    // CPU信息
    pub count: u64,
    pub usage: f32,
}

// ? async/await使用需要引入 features, 并且返回必须是一个Result枚举;
pub async fn get_cpu_info(time: u64) -> Result<Cpu, Error> {
    // 通过两次插值计算cpu使用率
    let measurement_1 = cpu::usage().await?;
    futures_timer::Delay::new(Duration::from_millis(time)).await;
    let measurement_2 = cpu::usage().await?;

    let usage = (measurement_2 - measurement_1).get::<ratio::percent>();
    let count = cpu::logical_count().await?;
    let cpu = Cpu { count, usage };
    return Ok(cpu);
}
