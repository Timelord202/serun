#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![expect(rustdoc::missing_crate_level_docs)] // it's an example

use serun::cpu;
use serun::cartridge;
use eframe::egui;

fn main() -> eframe::Result {
    env_logger::init();

    // TODO: Run in a separate thread
    let mut cpu = cpu::CPU::default();
    let cart = cartridge::Cartidge::from_path("./tests/nestest.nes").unwrap();
    cpu.load_program(cart.prg_rom);

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    eframe::run_native(
        "CPU Debugger",
        options,
        Box::new(|_cc| {
            Ok(Box::<DebugInfo>::default())
        }),
    )
}

#[derive(Default)]
struct DebugInfo {
    register_a: u8,
    register_x: u8,
    register_y: u8,
    stack_pointer: u8,
    program_counter: u16,
    status: u8,
}

impl eframe::App for DebugInfo {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show_inside(ui, |ui| {
            ui.label(format!("a: 0x{}", self.register_a));
            ui.label(format!("x: 0x{}", self.register_x));
            ui.label(format!("y: 0x{}", self.register_y));
            ui.label(format!("sp: 0x{}", self.stack_pointer));
            ui.label(format!("pc: 0x{}", self.program_counter));
            ui.label(format!("status: 0x{}", self.status));
        });
    }
}