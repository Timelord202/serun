#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![expect(rustdoc::missing_crate_level_docs)]

use serun::cpu;
use serun::cartridge;
use eframe::egui;
use egui::{RichText, Color32};
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
            let pc = cpu.program_counter as usize;
            let memory = cpu.memory.raw_memory[pc-5..pc+6].to_vec();
            tx.send(CpuSnapshot::from_cpu(&cpu, memory)).unwrap();
            thread::sleep(std::time::Duration::from_millis(1));
        }
    });

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 350.0]),
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
    memory: Vec<u8>
}

impl CpuSnapshot {
    fn from_cpu(data: &cpu::CPU, memory: Vec<u8>) -> CpuSnapshot {
        Self {
            register_a: data.register_a,
            register_x: data.register_x,
            register_y: data.register_y,
            stack_pointer: data.stack_pointer,
            program_counter: data.program_counter,
            status: data.status,
            memory
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
            ui.vertical(|ui| {
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        ui.label(format!("a: {:X}", self.state.register_a));
                        ui.label(format!("x: {:X}", self.state.register_x));
                        ui.label(format!("y: {:X}", self.state.register_y));
                    });
                    ui.horizontal(|ui| {
                        ui.label(format!("sp: {:X}", self.state.stack_pointer));
                        ui.label(format!("pc: {:X}", self.state.program_counter));
                        ui.label(format!("status: {:X}", self.state.status));
                    });
                });

                ui.add_space(10.0);
                ui.separator();
                ui.add_space(10.0);

                ui.vertical(|ui| {
                    // TODO: clean this up
                    let pc = self.state.program_counter as usize;
                    ui.label(format!("{:X}: {:X}", pc - 5, self.state.memory[0]));
                    ui.label(format!("{:X}: {:X}", pc - 4, self.state.memory[1]));
                    ui.label(format!("{:X}: {:X}", pc - 3, self.state.memory[2]));
                    ui.label(format!("{:X}: {:X}", pc - 2, self.state.memory[3]));
                    ui.label(format!("{:X}: {:X}", pc - 1, self.state.memory[4]));
                    ui.label(RichText::new(format!("{:X}: {:X}", self.state.program_counter, self.state.memory[5])).color(Color32::GREEN));
                    ui.label(format!("{:X}: {:X}", pc + 1, self.state.memory[6]));
                    ui.label(format!("{:X}: {:X}", pc + 2, self.state.memory[7]));
                    ui.label(format!("{:X}: {:X}", pc + 3, self.state.memory[8]));
                    ui.label(format!("{:X}: {:X}", pc + 4, self.state.memory[9]));
                    ui.label(format!("{:X}: {:X}", pc + 5, self.state.memory[10]));
                });
            });
        });

        ui.request_repaint();
    }
}