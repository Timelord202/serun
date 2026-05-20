use serde::{Deserialize, Serialize};
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
fn get_test_scenarios(path: &str) -> Vec<TestScenario> {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let serialized: Vec<TestScenario> = serde_json::from_reader(reader).unwrap();
    serialized
}

fn load_test_scenario(cpu: &mut cpu::CPU, scenario: &TestScenario) {
    cpu.pc = scenario.initial.pc;
    cpu.register_a = scenario.initial.a;
    cpu.register_a = scenario.initial.x;
    cpu.status = scenario.initial.p;
    cpu.register_y = scenario.initial.y;

    for (addr, val) in &scenario.initial.ram {
        cpu.memory.write(*addr, *val);
    }
}

fn verify_test_results(cpu: &cpu::CPU, scenario: &TestScenario) {
    // TODO: Implement
}

pub fn run_tests() {
    let mut cpu = cpu::CPU::default();
    let scenarios = get_test_scenarios("./tests/json/00.json");

    for scenario in scenarios {
        load_test_scenario(&mut cpu, &scenario);
        verify_test_results(&cpu, &scenario);
        cpu.memory.raw_memory.clear();
    }
}