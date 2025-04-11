// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use chrono::Utc;
use device_query::{DeviceQuery, DeviceState, Keycode};
use serde::{Deserialize, Serialize};
use slint::SharedString;
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::sync::{Arc, Mutex};
use std::thread;

slint::include_modules!();

#[derive(Debug, Serialize, Deserialize)]
pub struct MousePosition {
    x: i32,
    y: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KeyStrokes {
    keys: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TrackingData {
    date: String,
    mouse_position: Option<MousePosition>,
    key_strokes: Option<KeyStrokes>,
}

fn save_to_file(data: &Vec<TrackingData>, file_path: &str) -> Result<(), Box<dyn Error>> {
    let json = serde_json::to_string_pretty(data)?;
    let mut file = File::create(file_path)?;
    file.write_all(json.as_bytes())?;
    Ok(())
}

fn track_mouse_and_keystrokes(
    tracking_data: Arc<Mutex<Vec<TrackingData>>>,
    is_tracking: Arc<Mutex<bool>>,
    task_name: String,
) -> Result<(), Box<dyn Error>> {
    let device_state = DeviceState::new();

    loop {
        if !*is_tracking.lock().unwrap() {
            println!("Not tracking");
            save_to_file(
                &tracking_data.lock().unwrap(),
                &String::from(task_name + ".json"),
            )?;
            break;
        }

        println!("Tracking");
        let mouse_pos = device_state.get_mouse().coords;
        let keys: Vec<Keycode> = device_state.get_keys();

        let tracking_data_item = TrackingData {
            date: Utc::now().to_rfc3339(),
            mouse_position: if mouse_pos.0 != 0 && mouse_pos.1 != 0 {
                Some(MousePosition {
                    x: mouse_pos.0,
                    y: mouse_pos.1,
                })
            } else {
                None
            },
            key_strokes: if !keys.is_empty() {
                Some(KeyStrokes {
                    keys: keys.iter().map(|k| k.to_string()).collect(),
                })
            } else {
                None
            },
        };

        tracking_data.lock().unwrap().push(tracking_data_item);
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new()?;

    let tracking_data = Arc::new(Mutex::new(Vec::<TrackingData>::new()));
    let is_tracking = Arc::new(Mutex::new(false));

    let shared_tracking_data = tracking_data.clone();
    let shared_is_tracking = is_tracking.clone();
    let second_shared_is_tracking = is_tracking.clone();

    let ui_handle = ui.as_weak();
    let ui_handle_2 = ui.as_weak();
    ui.on_request_create_task(move |task_name: SharedString| {
        let ui = ui_handle.unwrap();
        {
            let mut track_flag = shared_is_tracking.lock().unwrap();
            *track_flag = !*track_flag;
            let mut tracking_data = shared_tracking_data.lock().unwrap();
            tracking_data.clear();
        }

        let task_name_str = task_name.to_string();

        ui.set_task_name(task_name);
        ui.set_is_task_created(true);

        let second_shared_tracking_data = shared_tracking_data.clone();
        let second_shared_is_tracking = shared_is_tracking.clone();

        if *second_shared_is_tracking.lock().unwrap() {
            thread::spawn(move || {
                track_mouse_and_keystrokes(
                    second_shared_tracking_data,
                    second_shared_is_tracking,
                    task_name_str,
                )
                .unwrap();
            });
        }
    });

    ui.on_request_stop_task(move || {
        let ui = ui_handle_2.unwrap();
        {
            let mut track_flag = second_shared_is_tracking.lock().unwrap();
            *track_flag = false;
        }

        ui.set_is_task_created(false);
    });

    ui.run()?;

    Ok(())
}
