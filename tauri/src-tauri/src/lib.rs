use serun::{cpu, cartridge};
use tauri::{Emitter, Manager, State};
use serde::Serialize;
use std::sync::{LazyLock, Mutex};

static CPU: LazyLock<Mutex<cpu::CPU>> = LazyLock::new(|| {
    Mutex::new(cpu::CPU::default())
});

// #[derive(Clone, Serialize)]
// #[serde(rename_all = "camelCase", rename_all_fields = "camelCase", tag = "event", content = "data")]
// enum CpuDto {
//     pc
// }

struct AppState {
    is_running: bool
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn initialize_program(filepath: &str) {
    let cartridge = cartridge::Cartidge::from_path(filepath).unwrap();
    let mut cpu = CPU.lock().unwrap();
    cpu.load_program(cartridge.prg_rom);
}

#[tauri::command]
fn execute_program(state: State<'_, AppState>) {
    let mut cpu = CPU.lock().unwrap();
    
    while state.is_running {
        cpu.execute_instruction();
    }
}

#[tauri::command]
fn execute_instruction() {
    let mut cpu = CPU.lock().unwrap();
    cpu.execute_instruction();
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let initial_state = AppState {
                is_running: false,
            };
            app.manage(initial_state);
            let app_handle = app.handle();
            let app_state  = app.state::<AppState>();
            let is_running = app_state.is_running.clone();
            tauri::async_runtime::spawn(async move {
                loop {
                    if is_running {
                        let mut cpu = CPU.lock().unwrap();
                        cpu.execute_instruction();
                        // app_handle.emit("cpu-instruction-complete", payload);
                    }
                }
            });
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            initialize_program,
            execute_program,
            execute_instruction
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
