// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_notification::init())
        .invoke_handler(tauri::generate_handler![schedule_forgetting_curve_notifications])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// add myself
use tauri_plugin_notification::{NotificationExt, Schedule};

#[tauri::command]
fn schedule_forgetting_curve_notifications(app: tauri::AppHandle, content: String) {
    let schedules = [
        ("1日後", 60 * 60 * 24),
        ("3日後", 60 * 60 * 24 * 3),
        ("1週間後", 60 * 60 * 24 * 7),
        ("1か月後", 60 * 60 * 24 * 30),
    ];

    for (label, delay_sec) in schedules {
        let id_int = (rand::random::<u32>() % 2147483647) as i32; // 重複を避けるID
        
        // 現在時刻から delay_sec 後の時刻を計算
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let target_time = now + delay_sec;

        // 通知の作成とスケジュール
       let _ = app.notification()
            .builder()
            .id(id_int)
            .title(format!("復習リマインド ({})", label))
            .body(&content)
            .schedule(Schedule::At {
                // 通知を送る特定の日時を指定
                date: (std::time::SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(target_time)).into(),
                // 繰り返し設定（必要なければ false）
                repeating: false,
                // デバイスがアイドル状態（省電力モード等）でも通知するか
                allow_while_idle: true,
            })
            .show();
    }
}