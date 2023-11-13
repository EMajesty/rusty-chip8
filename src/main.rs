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
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
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

        let x = opcode & 0x0F00 >> 2;
        let y = opcode & 0x00F0 >> 1;
        let kk = opcode & 0x00FF;
        let nnn = opcode & 0x0FFF;

        // decode
        match opcode & 0xF000 {
            0x0 => match opcode {
                0x00E0 => , // CLS
                0x00EE => program_counter = stack.pop().unwrap(), // RET
                _ => println!("Unknown opcode {}", opcode),
            },
            0x1 => program_counter = nnn, // JP addr
            0x2 => {
                // CALL addr
                stack.push(program_counter);
                program_counter = nnn;
            },
            0x3 => if v[x] == kk { program_counter += 2; }, // SE Vx, byte         
            0x4 => if v[x] != kk { program_counter += 2; }, // SNE Vx, byte
            0x5 => if v[x] == v[y] { program_counter += 2; }, // SE Vx, Vy       
            0x6 => v[x] = kk, // LD Vx, byte
            0x7 => v[x] += kk, // ADD Vx, byte
            0x8 => match opcode & 0x000F {
                0x0000 => v[x] = v[y], // LD Vx, Vy
                0x0001 => v[x] = v[x] | v[y], // OR
                0x0002 => v[x] = v[x] & v[y], // AND
                0x0003 => v[x] = v[x] ^ v[y], // XOR
                0x0004 => , // ADD
                0x0005 => , // SUB
                0x0006 => , // SHR
                0x0007 => , // SUBN
                0x000E => , // SHL
                _ => println!("Unknown opcode {}", opcode),
            },
            0x9 => if v[x] != v[y] { program_counter += 2; }, // SNE Vx, Vy
            0xA => index_register = nnn, // LD I, addr
            0xB => program_counter = nnn + v[0], // JP V0, addr
            0xC => , // RND Vx, byte
            0xD => , // DRW Vx, Vy, nibble
            0xE => match opcode & 0xF0FF {
                0xE09E => , // SKP Vx
                0xE0A1 => , // SKNP Vx
                _ => println!("Unknown opcode {}", opcode),
            },
            0xF => match opcode & 0xF0FF {
                0xF007 => , // LD Vx, DT
                0xF00A => , // LC Vx, K
                0xF015 => , // LD DT, Vx
                0xF018 => , // LD ST, Vx
                0xF01E => , // ADD I, Vx
                0xF029 => , // LD F, Vx
                0xF033 => , // LD B, Vx
                0xF055 => , // LD [I], Vx
                0xF065 => , // LD Vx, [I]
                _ => println!("Unknown opcode {}", opcode),
            },
            _ => println!("Unknown opcode {}", opcode),
        }
    }
}
