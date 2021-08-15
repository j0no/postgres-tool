#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
use tauri::{CustomMenuItem, SystemTray, SystemTrayMenu, SystemTrayMenuItem, SystemTrayEvent};
use tauri::{Manager, AppHandle, App};
use std::path::{Path, PathBuf};
use whoami::{username};

mod psql;
use psql::{start_cmd, status_cmd, stop_cmd, CtlStatusResponse};

fn get_status_icon_pathbuf() -> PathBuf {
  match status_cmd() {
    CtlStatusResponse::NoServerRunning => {
      let mut path = PathBuf::new();
      path.push(format!("/home/{}/Projects/postgres-tool/src-tauri/icons/off.png", username()));
      return path;
    },
    CtlStatusResponse::ServerRunning => {
      let mut path = PathBuf::new();
      path.push(format!("/home/{}/Projects/postgres-tool/src-tauri/icons/on.png", username()));
      return path;
    },
    CtlStatusResponse::NoResponse => {
      let mut path = PathBuf::new();
      path.push(format!("/home/{}/Projects/postgres-tool/src-tauri/icons/off.png", username()));
      return path;
    }
  }
}


fn update_status_icon_app_handle(app: &AppHandle){
  let icon_path = get_status_icon_pathbuf();
  app.tray_handle().set_icon(tauri::Icon::File(icon_path)).unwrap();
}

fn main() {

  let status_icon_path = get_status_icon_pathbuf();

  let quit = CustomMenuItem::new("quit".to_string(), "Quit");

  let start = CustomMenuItem::new("start".to_string(), "Start");
  let stop = CustomMenuItem::new("stop".to_string(), "Stop");

 
  let tray_menu = SystemTrayMenu::new()
    .add_item(start)
    .add_item(stop)
    .add_native_item(SystemTrayMenuItem::Separator)
    .add_item(quit);

  let system_tray = SystemTray::new()
    .with_menu(tray_menu)
    .with_icon(tauri::Icon::File(status_icon_path));
  
  let mut app = tauri::Builder::default()
    .setup(|app| {
      let main_window = app.get_window("main").unwrap();
      main_window.hide().unwrap();
      Ok(())
    })
    .system_tray(system_tray)
    .on_system_tray_event(|app, event| {
      match event {
      SystemTrayEvent::LeftClick {
        position: _,
        size: _,
        ..
      } => {
        println!("system tray received a left click");
      }
      SystemTrayEvent::RightClick {
        position: _,
        size: _,
        ..
      } => {
        println!("system tray received a right click");
      }
      SystemTrayEvent::DoubleClick {
        position: _,
        size: _,
        ..
      } => {
        println!("system tray received a double click");
      }
      SystemTrayEvent::MenuItemClick { id, .. } => {
        match id.as_str() {
          "quit" => {
            std::process::exit(0);
          },
          "start" => {
            start_cmd();
            update_status_icon_app_handle(app);
          },
          "stop" => {
            stop_cmd();
            update_status_icon_app_handle(app);
          },
          _ => {}
        }
      }
      _ => {
      }
      } 
    });

    app
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}