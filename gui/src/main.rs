#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![expect(rustdoc::missing_crate_level_docs)]

use serun::cpu;
use serun::cartridge;
use eframe::egui;
use std::sync::mpsc::Receiver;
use std::thread;
use std::sync::mpsc;

fn main() -> eframe::Result {
    env_logger::init();
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let mut cpu = cpu::CPU::default();
        let cart = cartridge::Cartidge::from_path("./tests/nestest.nes").unwrap();
        cpu.load_program(cart.prg_rom);
        loop {
            cpu.execute_instruction();
            tx.send(CpuSnapshot::from(&cpu)).unwrap();
            thread::sleep(std::time::Duration::from_secs(1));
        }
    });

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    eframe::run_native(
        "CPU Debugger",
        options,
        Box::new(|_cc| {
            Ok(Box::new(CpuState::new(rx)))
        }),
    )
}

#[derive(Default)]
struct CpuSnapshot {
    register_a: u8,
    register_x: u8,
    register_y: u8,
    stack_pointer: u8,
    program_counter: u16,
    status: u8,
}

impl From<&cpu::CPU> for CpuSnapshot {
    fn from(data: &cpu::CPU) -> Self {
        Self {
            register_a: data.register_a,
            register_x: data.register_x,
            register_y: data.register_y,
            stack_pointer: data.stack_pointer,
            program_counter: data.program_counter,
            status: data.status,
        }
    }
}

struct CpuState {
    rx: Receiver<CpuSnapshot>,
    state: CpuSnapshot
}

impl CpuState {
    fn new(rx: Receiver<CpuSnapshot>) -> Self {
        Self {
            rx,
            state: CpuSnapshot::default()
        }
    }
}

impl eframe::App for CpuState {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        while let Ok(snapshot) = self.rx.try_recv() {
            self.state = snapshot;
        }

        egui::CentralPanel::default().show_inside(ui, |ui| {
            ui.label(format!("a: 0x{}", self.state.register_a));
            ui.label(format!("x: 0x{}", self.state.register_x));
            ui.label(format!("y: 0x{}", self.state.register_y));
            ui.label(format!("sp: 0x{}", self.state.stack_pointer));
            ui.label(format!("pc: 0x{}", self.state.program_counter));
            ui.label(format!("status: 0x{}", self.state.status));
        });

        ui.request_repaint();
    }
}