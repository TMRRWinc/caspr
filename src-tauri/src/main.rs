#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use std::{sync::Mutex, collections::HashMap, path::PathBuf, env};
use std::error::Error;

use serde_json::json;
use tauri::{AppHandle, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu, CustomMenuItem, State, async_runtime::block_on, PhysicalSize, PhysicalPosition, Window, Monitor};
use url::Url;
use tauri_plugin_autostart::MacosLauncher;

use tauri::{Wry};
use tauri_plugin_store::{with_store, StoreCollection};

use enigo::Enigo;
use window_shadows::set_shadow;

#[derive(Clone, serde::Serialize)]
struct Payload {
  args: Vec<String>,
  cwd: String,
}

struct WindowState {
  open: Mutex<bool>,
  window_positions: Mutex<HashMap<String, PhysicalPosition<i32>>>,
  window_sizes: Mutex<HashMap<String, PhysicalSize<u32>>>,
}

impl Default for WindowState {
  fn default() -> Self {
    Self {
      open: Mutex::new(false),
      window_positions: Mutex::new(HashMap::new()),
      window_sizes: Mutex::new(HashMap::new()),
    }
  }
}

fn make_tray() -> SystemTray {
  let tray = SystemTrayMenu::new()
    .add_item(CustomMenuItem::new("show".to_string(), "Show App"))
    .add_native_item(tauri::SystemTrayMenuItem::Separator)
    .add_item(CustomMenuItem::new("settings".to_string(), "Settings"))
    .add_native_item(tauri::SystemTrayMenuItem::Separator)
    .add_item(CustomMenuItem::new("quit".to_string(), "Quit App"));

  return SystemTray::new().with_menu(tray);
}

#[tauri::command]
fn get_os() -> String {

  #[cfg(target_os = "macos")]
  return "macos".to_string();
  
  #[cfg(target_os = "windows")]
  return "windows".to_string();
}

#[tauri::command]
async fn toggle_app(app: AppHandle) {
  let window = app.get_window("main").expect("No window found with label 'main'");
  if window.is_visible().expect("Error checking window visibility") {
    window.hide().expect("Error hiding main app window");
  } else {
    let window_state = app.state::<WindowState>();
    let active_monitor = get_monitor_from_mouse_position(&window).expect("Error getting active monitor from mouse position");
    let window_size = derive_app_size(window_state, &active_monitor).expect("Error deriving app size");

    let window_state = app.state::<WindowState>();
    let window_position = derive_app_position(window_state, &active_monitor, window_size).expect("Error deriving app position");

    window.set_position(window_position).expect("Error setting window position");
    window.set_size(window_size).expect("Error setting window size");
    show_app(app.app_handle()).await;
  }
}

#[tauri::command]
fn window_moved(app: AppHandle) {
  let window_state = app.state::<WindowState>();
  let window = app.get_window("main").expect("No window found with label 'main'");
  let window_position = window.inner_position().expect("Error getting window position");
  let monitor = match get_monitor_from_physical_position(&window, window_position) {
    Ok(monitor) => monitor,
    Err(_) => return
  }; 
  let monitor_name = monitor.name().expect("Error getting monitor name").to_string();
  let monitor_position = monitor.position();
  let monitor_unique_name = [monitor_name, monitor_position.x.to_string(), monitor_position.y.to_string()].join("-");
  let mut window_positions = window_state.window_positions.lock().unwrap();
  window_positions.insert(monitor_unique_name, window_position);
}

fn get_monitor_from_physical_position(window: &Window, position: PhysicalPosition<i32>) -> Result<Monitor, ()> {
  let available_monitors = window.available_monitors().expect("Error getting available monitors");

  let active_monitor = available_monitors.into_iter().find(|monitor| {
    let monitor_scale_factor = monitor.scale_factor();

    let monitor_position = monitor.position();
    let monitor_size = monitor.size();

    let monitor_position_logical = monitor_position.to_logical::<f64>(monitor_scale_factor);
    let monitor_size_logical = monitor_size.to_logical::<f64>(monitor_scale_factor);

    let position_logical = position.to_logical::<f64>(monitor_scale_factor);

    let x1 = monitor_position.x;
    let x2 = monitor_position.x + monitor_size.width as i32;
    let y1 = monitor_position.y;
    let y2 = monitor_position.y + monitor_size.height as i32;

    let x = position.x;
    let y = position.y;

    let x1_logical = monitor_position_logical.x;
    let x2_logical = monitor_position_logical.x + monitor_size_logical.width;
    let y1_logical = monitor_position_logical.y;
    let y2_logical = monitor_position_logical.y + monitor_size_logical.height;

    let x_logical = position_logical.x;
    let y_logical = position_logical.y;

    (x >= x1 && x <= x2 && y >= y1 && y <= y2) || (x_logical >= x1_logical && x_logical <= x2_logical && y_logical >= y1_logical && y_logical <= y2_logical)
  });

  match active_monitor {
    Some(monitor) => Ok(monitor),
    None => Err(())
  }
}

fn get_monitor_from_mouse_position(window: &Window) -> Result<Monitor, ()> {
  let cursor_location = Enigo::mouse_location();
  get_monitor_from_physical_position(window, PhysicalPosition::new(cursor_location.0, cursor_location.1))
}

#[tauri::command]
fn window_resized(app: AppHandle) {
  let window_state = app.state::<WindowState>();
  let window = app.get_window("main").expect("No window found with label 'main'");

  let window_position = window.inner_position().expect("Error getting window position");
  let monitor = get_monitor_from_physical_position(&window, window_position).expect("Error getting monitor from physical position");
  let monitor_name = monitor.name().expect("Error getting monitor name").to_string();
  let monitor_position = monitor.position();
  let monitor_unique_name = [monitor_name, monitor_position.x.to_string(), monitor_position.y.to_string()].join("-");
  let mut window_sizes = window_state.window_sizes.lock().unwrap();
  
  // check for monitor_unique_name in window_sizes if not found, retrieve value and print it
  let window_size: PhysicalSize<u32>;

  let existing_window_height: u32;
  if window_sizes.contains_key(&monitor_unique_name) {
    existing_window_height = window_sizes.get(&monitor_unique_name).expect("Error getting window size").height;
  } else {
    existing_window_height = 338;
  }
  
  if *window_state.open.lock().unwrap() {
    window_size = window.inner_size().expect("Error getting window size");
  } else {
    window_size = PhysicalSize::new(window.inner_size().expect("Error getting window size").width, existing_window_height);
  }

  window_sizes.insert(monitor_unique_name, window_size);

  // store window sizes to tauri-plugin-store
  let stores = app.state::<StoreCollection<Wry>>();
  let path = PathBuf::from(".preferences.dat");
  
  with_store(app.app_handle(), stores, path, |store| {
    store.insert("window_sizes".to_string(), json!(*window_sizes)).expect("Error inserting window sizes");
    store.save()
  }).expect("Error saving window sizes");

}

fn derive_app_size (window_state: State<WindowState>, monitor: &Monitor) -> Result<PhysicalSize<f32>, ()> {
  let open: bool = *window_state.open.lock().unwrap();
  let app_height = 94.0;
  let app_width = 700.0 * monitor.scale_factor() as f32;
  let app_max_height = 338.0;

  let window_sizes = window_state.window_sizes.lock().unwrap();
  
  let monitor_name = monitor.name().expect("Error getting monitor name").to_string();
  let monitor_position = monitor.position();
  let monitor_unique_name = [monitor_name, monitor_position.x.to_string(), monitor_position.y.to_string()].join("-");

  if window_sizes.contains_key(&monitor_unique_name) {
    let window_size = window_sizes.get(&monitor_unique_name).expect("Error getting window size");
    if open {
      return Ok(PhysicalSize::new(window_size.width as f32, window_size.height as f32));
    }
    return Ok(PhysicalSize::new(window_size.width as f32, app_height * monitor.scale_factor() as f32));
  }
  // Only reaches this code when there is no saved size for the monitor
  if open {
    return Ok(PhysicalSize::new(app_width, app_max_height * monitor.scale_factor() as f32));
  }
  return Ok(PhysicalSize::new(app_width, app_height * monitor.scale_factor() as f32));
}

fn derive_app_position (window_state: State<WindowState>, monitor: &Monitor, app_size: PhysicalSize<f32>) -> Result<PhysicalPosition<i32>, ()> {
  let monitor_position = monitor.position();
  let monitor_size = monitor.size();

  // Calculate position to display app on active monitor
  let x = monitor_position.x + (monitor_size.width as i32 / 2) - app_size.width as i32 / 2;
  let y = monitor_position.y + ((monitor_size.height as f32 * 0.05) as i32);
  
  let window_positions = window_state.window_positions.lock().unwrap();
  // check for monitor name in keys of window_positions
  let monitor_name = monitor.name().expect("Error getting monitor name").to_string();
  let monitor_unique_name = [monitor_name, monitor_position.x.to_string(), monitor_position.y.to_string()].join("-");
  if window_positions.contains_key(&monitor_unique_name) {
    let window_position = window_positions.get(&monitor_unique_name).expect("Error getting window position");
    return Ok(PhysicalPosition::new(window_position.x, window_position.y));
  }
  

  let position = PhysicalPosition::new(x,y);
  return Ok(position);
}


#[tauri::command]
fn set_expand(app: AppHandle, open: bool, state: State<WindowState>) {
  *state.open.lock().unwrap() = open;

  let window = app.get_window("main").expect("No window found with label 'main'");
  let window_state = app.state::<WindowState>();
  let active_monitor = get_monitor_from_mouse_position(&window).expect("Error getting active monitor from mouse position");
  let window_size = derive_app_size(window_state, &active_monitor).expect("Error deriving app size");

  window.set_size(window_size).expect("Error setting window size");
}


#[tauri::command]
async fn show_app(app: AppHandle) {  
  let window = app.get_window("main").expect("No window found with label 'main'");
  let window_state = app.state::<WindowState>();
  let active_monitor = get_monitor_from_mouse_position(&window).expect("Error getting active monitor from mouse position");
  let window_size = derive_app_size(window_state, &active_monitor).expect("Error deriving app size");
  
  let window_state = app.state::<WindowState>();
  let window_position = derive_app_position(window_state, &active_monitor, window_size).expect("Error deriving app position");

  window.set_position(window_position).expect("Error setting window position");
  window.set_size(window_size).expect("Error setting window size");

  #[cfg(any(windows, target_os = "windows"))]
  set_shadow(&window, true).unwrap();

  window.show().expect("Error showing main app window");
    window.set_focus().expect("Error focusing main app window");
}

#[tauri::command]
fn hide_app(app: AppHandle) {
  let window = app.get_window("main").expect("No window found with label 'main'");
  window.hide().expect("Error hiding main app window");
}



fn handle_tray_event(app: &AppHandle, event: SystemTrayEvent) {
  if let SystemTrayEvent::LeftClick { .. } = event {
    // block_on(toggle_app(app.app_handle()));
    block_on(show_app(app.app_handle()));
  } else if let SystemTrayEvent::MenuItemClick { id, .. } = event {
    match id.as_str() {
      "settings" => {
        block_on(open_settings(app.app_handle()));
      }
      "show" => {
        block_on(show_app(app.app_handle()));
      }
      "quit" => {
        app.exit(0);
      }
      _ => {}
    }
  }
}



#[tauri::command]
async fn open_settings(handle: tauri::AppHandle) {
  let settings_window = match handle.get_window("settings") {
    Some(settings_window) => settings_window,
    None => {
      let url =  tauri::WindowUrl::App("/settings/general".into());
      
      tauri::WindowBuilder::new(
        &handle,
        "settings", /* the unique window label */
        url
      ).title("caspr.ai | Settings").center().min_inner_size(600.0, 400.0).decorations(false).resizable(true).transparent(true).skip_taskbar(true).inner_size(650.0, 400.0).always_on_top(true).build().expect("Error building settings window")
    }
  };

  let active_monitor = get_monitor_from_mouse_position(&settings_window).expect("Error getting active monitor from mouse position");
  let window_size = PhysicalSize::new(650.0, 400.0);
  let window_state = handle.state::<WindowState>();
  let window_position = derive_app_position(window_state, &active_monitor, window_size).expect("Error deriving app position");
  settings_window.set_position(window_position).expect("Error setting window position");

  settings_window.show().expect("Error showing settings window");
  settings_window.set_focus().expect("Error showing settings window"); 
}

#[tauri::command]
async fn open_onboarding(handle: tauri::AppHandle, step: u8) {
  // if step provided, open onboarding window to that step otherwise open to step 0

  let onboarding_window = match handle.get_window("onboarding") {
    Some(onboarding_window) => onboarding_window,
    None => {
      tauri::WindowBuilder::new(
        &handle,
        "onboarding", /* the unique window label */
        // tauri::WindowUrl::App("/onboarding?step=3".into())
        tauri::WindowUrl::App(("/onboarding?step=".to_owned() + &step.to_string()).into())
      ).title("caspr.ai | Onboarding").center().resizable(true).min_inner_size(650.0, 400.0).decorations(false).transparent(true).skip_taskbar(false).inner_size(650.0, 400.0).build().expect("Error building onboarding window")
    }
  };
  onboarding_window.show().expect("Error showing onboarding window");
  onboarding_window.set_focus().expect("Error showing onboarding window");
}

#[tauri::command]
async fn get_os_theme() -> String {
  let theme = dark_light::detect();
  match theme {
    dark_light::Mode::Dark => "dark",
    dark_light::Mode::Light => "light",
    dark_light::Mode::Default => "default"
  }.to_string()
}

fn set_window_sizes_from_storage(app: &AppHandle) {
  let window_state = app.state::<WindowState>();
  let mut window_sizes = window_state.window_sizes.lock().unwrap();

  let stores = app.state::<StoreCollection<Wry>>();
  let path = PathBuf::from(".preferences.dat");
  
  with_store(app.app_handle(), stores, path, |store| {
    let sizes = store.get("window_sizes");
    
    if let Some(sizes) = sizes {
      for (key, value) in sizes.as_object().unwrap() {
        window_sizes.insert(key.to_string(), serde_json::from_value(value.clone()).unwrap());
      }
    }
    store.save()
  }).expect("Error retrieving window sizes");
}

fn set_os_theme_pref(app: &AppHandle) {
  let stores = app.state::<StoreCollection<Wry>>();
  let path = PathBuf::from(".preferences.dat");

  let theme = dark_light::detect();
  let theme_string = match theme {
    dark_light::Mode::Dark => "dark",
    dark_light::Mode::Light => "light",
    dark_light::Mode::Default => "default"
  };

  with_store(app.app_handle(), stores, path, |store| {
    store.insert("os_theme".to_string(), theme_string.into()).expect("Error inserting theme into store");
    store.save()
  }).expect("Error retrieving theme");
}

fn get_api_key(app: &AppHandle) {
  let stores = app.state::<StoreCollection<Wry>>();
  let path = PathBuf::from(".preferences.dat");

  with_store(app.app_handle(), stores, path, |store| {
    let api_key = store.get("apiKey");
    
    match api_key {
      Some(api_key) => {
        env::set_var("OPENAI_API_KEY", api_key.to_string().replace('"', ""));
        store.save()
      }
    
      None => {Ok(())}
    }
  }).expect("Error retrieving api key");
}

use async_openai::{
  types::{CreateChatCompletionRequestArgs, ChatCompletionRequestMessage},
  Client,
};

use futures::StreamExt;

#[derive(serde::Serialize, Clone)]
struct StreamPayload<'a> {
  type_: String,
  id: &'a str,
  content: &'a String
}

async fn openai_stream(messages: Vec<ChatCompletionRequestMessage>, handle: AppHandle) -> Result<(), Box<dyn Error>> {
  let client = Client::new();

  let request = CreateChatCompletionRequestArgs::default()
    .model("gpt-3.5-turbo")
    .max_tokens(512u16)
    .messages(messages).build()?;

  let mut stream = client.chat().create_stream(request).await?;

  handle.emit_all("stream-event", StreamPayload {type_: String::from("start"), content: &String::from("stream started"), id: &String::from("")}).unwrap();
  while let Some(result) = stream.next().await {
    match result {
        Ok(response) => {
          response.choices.iter().for_each(|chat_choice| {
              let mut res_id: String = "".to_string();

              if let Some(ref id) = response.id {
                // writeln!(lock, "{}", id).unwrap();
                res_id = id.to_owned();
              } 
              if let Some(ref role) = chat_choice.delta.role {
                handle.emit_all("stream-event", StreamPayload {type_: "role".to_string(), id: res_id.as_str(), content: &role.to_string()}).unwrap();
              }
              if let Some(ref content) = chat_choice.delta.content {
                handle.emit_all("stream-event", StreamPayload {type_: "content".to_string(), id: res_id.as_str(), content}).unwrap();
              }
              if let Some(ref finish_reason) = chat_choice.finish_reason {
                handle.emit_all("stream-event", StreamPayload {type_: "finish".to_string(), id: res_id.as_str(), content: &finish_reason.to_string()}).unwrap(); 
              }
            });
          }
          Err(err) => {
            println!("error: {err}");
            handle.emit_all("stream-event", StreamPayload {type_: "error".to_string(), id: "unknown", content: &err.to_string()}).unwrap(); 
        }
    }
  };
  Ok(())
}

#[tauri::command]
async fn stream(handle: AppHandle, messages: Vec<ChatCompletionRequestMessage>) {
  std::thread::spawn(move || {
    block_on(openai_stream(messages, handle)).unwrap();
  });
}

fn main() {
  tauri::Builder::default().setup(|_app| {
      set_window_sizes_from_storage(&_app.app_handle());
      set_os_theme_pref(&_app.app_handle());
      get_api_key(&_app.app_handle());

      #[cfg(target_os = "macos")]
      return Ok(_app.set_activation_policy(tauri::ActivationPolicy::Accessory));

      #[cfg(target_os = "windows")]
      return Ok(());
    })
    .manage(WindowState { ..Default::default()} )
    // .plugin(sentry_plugin)
    .plugin(tauri_plugin_autostart::init(MacosLauncher::LaunchAgent, Some(vec!["--hide-on-startup"])))
    .plugin(tauri_plugin_single_instance::init(|app, argv, cwd| {
      app.emit_all("single-instance", Payload {args: argv, cwd}).unwrap();
    }))
    .plugin(tauri_plugin_store::Builder::default().build())
    .plugin(tauri_plugin_sql::Builder::default().build())
    .system_tray(make_tray())
    .on_system_tray_event(handle_tray_event)
    .invoke_handler(tauri::generate_handler![toggle_app, show_app, hide_app, open_settings, open_onboarding, get_os, set_expand, window_moved, window_resized, get_os_theme, stream])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
