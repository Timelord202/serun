use serun::{cartridge, cpu};

fn main() {
    let cartridge = cartridge::Cartidge::from_path("./test_roms/nestest.nes").unwrap();
    //println!("rom: {:?}", cartridge.prg_rom);
    let mut cpu: cpu::CPU = Default::default();
    cpu.load_and_run(cartridge.prg_rom);
}