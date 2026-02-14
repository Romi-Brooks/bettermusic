pub mod logger;
pub mod frontend;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // 初始化日志（仅传级别，无需文件路径）
    logger::init_logger("info")
        .expect("Failed to initialize logger");

    // 测试日志输出
    app_info!("Starting BetterMusic Tauri app...");


    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![frontend::callable::greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}