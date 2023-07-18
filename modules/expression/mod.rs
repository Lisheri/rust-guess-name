pub mod language;
pub mod system_info;

// 入口
pub async fn expression_main() {
    println!("expression_main");
    language::expression_language().await;
}
