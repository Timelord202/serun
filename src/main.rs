use serun::cpu::CPU;

fn main() {
    let register_x: u8 = 0xff;
    let new_val = (register_x + 1) % 0xff;
    println!("New value: {}", new_val);
}
