static FONT: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80  // F
];

fn main() {
    let mut stack: Vec<u16> = Vec::new();
    let mut memory: [u8; 4096] = [0; 4096];
    let mut display: [[bool; 64]; 32];
    let mut program_counter: u16 = 0x200;
    let mut index_register: u16;
    let mut delay_timer: u8;
    let mut sound_timer: u8;
    let mut v: [u8; 16]; // V registers (V0-VF)

    loop {
        // fetch
        let opcode: u16 = memory[program_counter] << 8 | memory[program_counter + 1];
        program_counter += 2;

        // decode
        match opcode & 0xF000 {
            0x0 => match opcode {
                0x00E0 => {
                    // CLS
                },
                0x00EE => {
                    // RET
                    // TODO: check popped value for None
                    program_counter = stack.pop().unwrap();
                },
                _ => println!("Unknown opcode {}", opcode),
            },
            0x1 => {
                // JP addr
                program_counter = opcode & 0x0FFF;
            },
            0x2 => {
                // CALL addr
                stack.push(program_counter);
                program_counter = opcode & 0x0FFF;
            },
            0x3 => {
                // SE Vx, byte
                let x = opcode & 0x0F00 >> 2;
                let kk = opcode & 0x00FF;

                if v[x] == kk {
                    program_counter += 2;
                }
            },
            0x4 => {
                // SNE Vx, byte
                let x = opcode & 0x0F00 >> 2;
                let kk = opcode & 0x00FF;

                if v[x] != kk {
                    program_counter += 2;
                }
            }
        }
    }
}
