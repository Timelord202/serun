use serde::{Deserialize, Serialize};
use std::fmt::{Binary, LowerHex};
use std::fs::File;
use std::io::BufReader;
use serun::cpu;

#[derive(Serialize, Deserialize, Debug)]
struct CpuState {
    pc: u16,
    s: u8,
    a: u8,
    x: u8,
    y: u8,
    p: u8,
    ram: Vec<(u16, u8)>
}


#[derive(Serialize, Deserialize, Debug)]
struct TestScenario {
    name: String,
    initial: CpuState,
    r#final: CpuState,
    cycles: Vec<(u16, u8, String)>
}

// TODO improve how this is parsing files (account more for errors)
fn get_test_scenarios(path: String) -> Vec<TestScenario> {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let serialized: Vec<TestScenario> = serde_json::from_reader(reader).unwrap();
    serialized
}

fn load_test_scenario(cpu: &mut cpu::CPU, scenario: &TestScenario) {
    cpu.pc = scenario.initial.pc;
    cpu.register_a = scenario.initial.a;
    cpu.register_x = scenario.initial.x;
    cpu.status = scenario.initial.p;
    cpu.register_y = scenario.initial.y;
    cpu.sp = scenario.initial.s;

    for (addr, val) in &scenario.initial.ram {
        cpu.memory.write(*addr, *val);
    }
}

fn format_bin<T: Binary>(val: T) -> String {
    format!("{:b}", val)
}

fn format_hex<T: LowerHex>(val: T) -> String {
    format!("{:x}", val)
}

fn verify_test_results(cpu: &cpu::CPU, scenario: &TestScenario) {
    assert_eq!(format_hex(cpu.pc), format_hex(scenario.r#final.pc), "pc isn't correct!");
    assert_eq!(format_hex(cpu.register_a), format_hex(scenario.r#final.a), "a register isn't correct!");
    assert_eq!(format_hex(cpu.register_x), format_hex(scenario.r#final.x), "x register isn't correct!");
    assert_eq!(format_bin(cpu.status), format_bin(scenario.r#final.p), "status register isn't correct!");
    assert_eq!(format_hex(cpu.register_y), format_hex(scenario.r#final.y), "y register isn't correct!");
    assert_eq!(format_hex(cpu.sp), format_hex(scenario.r#final.s), "stack pointer isn't correct!");

    for (addr, val) in &scenario.r#final.ram {
        assert_eq!(cpu.memory.read(*addr), *val, "ram values don't match at addr {}!", addr);
    }
}

// TODO: Generalize both functions
// TODO: Add proper logging
pub fn run_all_tests() {
    let mut cpu = cpu::CPU::default();
    
    for i in 0..=0xFF {
        let path = format!("./tests/json/{:02x}.json", i);
        let scenarios = get_test_scenarios(path);
        println!("Testing instruction {:02x}", i);
        for (j, scenario) in scenarios.iter().enumerate() {
            println!("Running test {}...", j + 1);
            load_test_scenario(&mut cpu, scenario);
            cpu.execute_instruction();
            verify_test_results(&cpu, scenario);
            cpu.memory.raw_memory.fill(0);
            println!("Completed test {}!", j + 1);
        }
    }

    println!("Successfully finished testing!");
}

pub fn run_one_test(hex: &String) {
    let mut cpu = cpu::CPU::default();
    let path = format!("./tests/json/{}.json", hex);
    let scenarios = get_test_scenarios(path);

    println!("Testing instruction {}", hex);
    for (i, scenario) in scenarios.iter().enumerate() {
        println!("Running test {}...", i + 1);
        load_test_scenario(&mut cpu, scenario);
        cpu.execute_instruction();
        verify_test_results(&cpu, scenario);
        cpu.memory.raw_memory.fill(0);
        println!("Completed test {}!", i + 1);
    }
}