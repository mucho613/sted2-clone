use tauri::{CustomMenuItem, Manager, Menu, Submenu};

pub fn menu() -> Menu {
    let settings = CustomMenuItem::new("settings".to_string(), "Settings");
    let submenu = Submenu::new("File", Menu::new().add_item(settings));

    Menu::new().add_submenu(submenu)
}

pub fn menu_event_handler(event: tauri::WindowMenuEvent) {
    // 既に Settings ウィンドウが開いている場合は、それをアクティブにする
    let window = event.window().get_window("Settings");

    match window {
        Some(window) => {
            window.set_focus().unwrap();
        }
        None => {
            tauri::WindowBuilder::new(
                &event.window().app_handle(),
                "Settings".to_string(),
                tauri::WindowUrl::App("settings".into()),
            )
            .build()
            .unwrap();
        }
    }
}
