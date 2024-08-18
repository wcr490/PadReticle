#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use base64::{engine::general_purpose, Engine};
use std::{
    fs::File,
    io::Read,
    sync::Arc,
    time::Duration,
};
use tauri::Manager;
use tokio::sync::Mutex;
use tokio::time::sleep;
use toml::Table;
use xci::state::{ButtonState, ACTION_Y, LEFT_BUMPER};

struct AppSettings {
    aim_key: usize,
    polling_interval: u64,
}

impl AppSettings {
    fn new(aim_key: usize, polling_interval: u64) -> Self {
        AppSettings {
            aim_key,
            polling_interval,
        }
    }
}

static mut APP_SETTINGS: AppSettings = AppSettings {
    aim_key: LEFT_BUMPER,
    polling_interval: 50,
};

#[tauri::command]
async fn update_crosshair_image() -> Result<String, String> {
    let mut file = File::open("crosshair.png").map_err(|e| e.to_string())?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).map_err(|e| e.to_string())?;
    Ok(general_purpose::STANDARD.encode(buffer))
}
#[tauri::command]
fn open_crosshair(window: tauri::Window) {
    window
        .get_window("crosshair")
        .expect("Failed to get crosshair window")
        .show()
        .expect("Failed to show window");
}
#[tauri::command]
fn close_crosshair(window: tauri::Window) {
    window
        .get_window("crosshair")
        .expect("Failed to get crosshair window")
        .hide()
        .expect("Failed to show window");
}
#[tauri::command]
fn minimize_main_window(window: tauri::Window) {
    window
        .get_window("main")
        .expect("Failed to get main window")
        .minimize()
        .expect("Failed to minimize main window");
}

fn main() {
    if let Ok(mut file) = File::open("config.toml") {
        let mut config_content = String::new();
        file.read_to_string(&mut config_content)
            .expect("Failed to read config.toml");
        let config: Table = toml::from_str(&config_content).expect("Failed to parse TOML");

        unsafe {
            APP_SETTINGS = AppSettings::new(
                config.get("aim_key").expect("Failed to get aim_key")
                    .as_integer()
                    .expect("Please check 'aim_key' in config.toml") as usize,
                config.get("polling_interval").expect("Failed to get polling_interval")
                    .as_integer()
                    .expect("Please check 'polling_interval' in config.toml")
                    as u64,
            );
        }
    }

    let controller_state = xci::state::ControllerState::new();
    let controller_arc = Arc::new(Mutex::new(controller_state));
    let controller_clone = Arc::clone(&controller_arc);

    tauri::Builder::default()
        .setup(move |app| {
            app.get_window("crosshair").unwrap();
            let _ = app
                .get_window("crosshair")
                .expect("Failed to get crosshair window")
                .set_ignore_cursor_events(true);
            let app_handle = app.app_handle();
            let app_handle_clone = app_handle.clone();

            tauri::async_runtime::spawn(async move {
                loop {
                    let is_connected = controller_clone.lock().await.is_connected;
                    let is_aim_pressed = unsafe {
                        controller_clone.lock().await.buttons[APP_SETTINGS.aim_key]
                            == ButtonState::Pressed
                    };

                    app_handle.emit_all("keep-alive", is_connected).unwrap();
                    app_handle.emit_all("keep-aim", is_aim_pressed).unwrap();
                    unsafe {
                        sleep(Duration::from_millis(APP_SETTINGS.polling_interval)).await;
                    }
                }
            });
            let controller_clone = Arc::clone(&controller_arc);

            tauri::async_runtime::spawn(async move {
                unsafe {
                    let mut state = std::mem::zeroed();
                    loop {
                        let mut controller = controller_clone.lock().await;
                        state = controller.refresh(state);
                    }
                }
            });

            tauri::async_runtime::spawn(async move {
                app_handle_clone.listen_global("update-settings", move |_| {
                    let controller_clone_for_settings = Arc::clone(&controller_arc);
                    tauri::async_runtime::spawn(async move {
                        loop {
                            let controller = controller_clone_for_settings.lock().await;
                            for i in 0..=ACTION_Y {
                                if controller.buttons[i] == ButtonState::Pressed {
                                    println!("Updating aim key to {}", i);
                                    unsafe { APP_SETTINGS.aim_key = i }
                                    sleep(Duration::from_millis(100)).await;
                                }
                            }
                        }
                    });
                });
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            update_crosshair_image,
            open_crosshair,
            close_crosshair,
            minimize_main_window
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
