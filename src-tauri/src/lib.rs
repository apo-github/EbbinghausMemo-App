// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// add myself
use tauri_plugin_notification::NotificationExt;

#[tauri::command]
fn schedule_forgetting_curve_notifications(app: tauri::AppHandle, content: String) {
    let schedules = [
        ("1日後", 60 * 60 * 24),
        ("3日後", 60 * 60 * 24 * 3),
        ("1週間後", 60 * 60 * 24 * 7),
        ("1か月後", 60 * 60 * 24 * 30),
    ];

    for (label, delay_sec) in schedules {
        let identifier = format!("todo-{}-{}", label, uuid::Uuid::new_v4()); // 重複を避けるID
        
        // モバイルOSのスケジューラに登録
        let _ = app.notification()
            .builder()
            .identifier(&identifier)
            .title(format!("復習リマインド ({})", label))
            .body(&content)
            .schedule(tauri_plugin_notification::Schedule::After {
                seconds: delay_sec,
            })
            .unwrap()
            .show();
    }
}