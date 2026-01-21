use std::{sync::Mutex};

use serun::{cpu, cartridge};
use tauri::{Manager, State};
use serde::Serialize;

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase", tag = "event")]
struct CpuDTO {
    register_a: u8,
    register_x: u8,
    register_y: u8,
    stack_pointer: u8,
    program_counter: u16,
    status: u8,
    memory: Vec<u8>,
}

impl CpuDTO {
    fn new(cpu_state: &cpu::CPU) -> Self {
        Self {
            register_a: cpu_state.register_a,
            register_x: cpu_state.register_x,
            register_y: cpu_state.register_y,
            stack_pointer: cpu_state.stack_pointer,
            program_counter: cpu_state.program_counter,
            status: cpu_state.status,
            memory: cpu_state.memory.raw_memory.clone(),
        }
    }
}

#[derive(Default)]
struct AppState {
    cpu: cpu::CPU
}

#[tauri::command]
fn get_cpu_state(state: State<'_, Mutex<AppState>>) -> CpuDTO  {
  let state = state.lock().unwrap();
  CpuDTO::new(&state.cpu)
}

#[tauri::command]
fn execute_instruction(state: State<'_, Mutex<AppState>>) -> CpuDTO  {
  let mut state = state.lock().unwrap();
  state.cpu.execute_instruction();
  CpuDTO::new(&state.cpu)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let mut app_state = AppState::default();
            let cart = cartridge::Cartidge::from_path("./src/nestest.nes");
            app_state.cpu.load_program(cart.unwrap().prg_rom);
            app.manage(Mutex::new(app_state));
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![execute_instruction, get_cpu_state])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
