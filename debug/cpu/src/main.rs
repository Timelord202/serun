#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![expect(rustdoc::missing_crate_level_docs)]

use serun::cpu;
use serun::cartridge;
use eframe::egui;
use egui::{RichText, Color32};
use std::sync::mpsc::Receiver;
use std::thread;
use std::sync::mpsc;
use std::cmp;

// Controls how many addresses are displayed in the debugger
const DEBUG_ADDRS: usize = 11;
const MEM_LEN: usize = 0x10000;

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
            let min_addr = cmp::min(
                MEM_LEN - DEBUG_ADDRS,
                pc.saturating_sub(DEBUG_ADDRS / 2),
            );
            let max_addr = cmp::min(
                MEM_LEN,
                min_addr + DEBUG_ADDRS,
            );
            let memory = cpu.memory.raw_memory[min_addr..max_addr].to_vec();
            tx.send(CpuSnapshot::from_cpu(&cpu, memory)).unwrap();
            if pc >= 0xFFFF - DEBUG_ADDRS {
                thread::sleep(std::time::Duration::from_secs(1));
            }
            else {
                thread::sleep(std::time::Duration::from_millis(1));
            }
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

                // Displays memory addresses
                ui.vertical(|ui| {
                    let pc = self.state.program_counter as usize;
                    let mem_len = self.state.memory.len();
                    let half_mem_len = mem_len / 2;
                    let max_addr = 0xFFFFusize;
                    let start_addr = if pc < half_mem_len {
                        0
                    } else if pc > max_addr - half_mem_len {
                        max_addr - mem_len + 1
                    } else {
                        pc - half_mem_len
                    };

                    for i in 0..mem_len {
                        let addr = start_addr + i;
                        let text = format!("{:04X}: {:02X}", addr, self.state.memory[i]);

                        if addr == pc {
                            ui.label(RichText::new(text).color(Color32::GREEN));
                        } else {
                            ui.label(text);
                        }
                    }
                });
            });
        });

        ui.request_repaint();
    }
}