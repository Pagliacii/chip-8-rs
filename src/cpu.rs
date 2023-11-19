/// # Chip-8 CPU
///
/// ## Registers
///
/// Chip-8 has 16 general purpose 8-bit registers, usually referred to as Vx,
/// where x is a hexadecimal digit (0 through F). There is also a 16-bit register
/// called I. This register is generally used to store memory addresses, so only
/// the lowest (rightmost) 12 bits are usually used.
///
/// The VF register should not be used by any program, as it is used as a flag
/// by some instructions.
///
/// Chip-8 also has two special purpose 8-bit registers, for the delay and
/// sound timers. When these registers are non-zero, they are automatically
/// decremented at a rate of 60Hz.
///
/// There are also some "pseudo-registers" which are not accessible from Chip-8
/// programs. The program counter (PC) should be 16-bit, and is used to store
/// the currently executing address. The stack pointer (SP) can be 8-bit, it is
/// used to point to the topmost level of the stack.
///
/// The stack is an array of 16 16-bit values, used to store the address that
/// the interpreter should return to when finished with a subroutine.
/// Chip-8 allows for up to 16 levels of nested subroutines.
///
/// ## Instructions
///
/// The original implementation of the Chip-8 language includes 36 different
/// instructions, including math, graphics, and flow control functions.
///
/// Super Chip-48 added an additional 10 instructions, for a total of 46.
///
/// All instructions are 2 bytes long and are stored most-significant-byte first.
/// In memory, the first byte of each instruction should be located at an even
/// addresses. If a program includes sprite data, it should be padded so any
/// instructions following it will be properly situated in RAM.
///
/// This document does not yet contain descriptions of the Super Chip-48 instruc-
/// tions. They are listed at [here](http://devernay.free.fr/hacks/chip8/C8TECH10.HTM#3.1).
///
/// In these listings, the following variables are used:
///
/// - nnn or addr - A 12-bit value, the lowest 12 bits of the instruction
/// - n or nibble - A 4-bit value, the lowest 4 bits of the instruction
/// - x - A 4-bit value, the lower 4 bits of the high byte of the instruction
/// - y - A 4-bit value, the upper 4 bits of the low byte of the instruction
/// - kk or byte - An 8-bit value, the lowest 8 bits of the instruction
use rand::Rng;

mod memory;

#[derive(Debug)]
struct Chip8 {
    // General purpose 8-bit registers (V0 to VF)
    v_registers: [u8; 16],

    // 16-bit register I (used for memory addresses)
    i_register: u16,

    // Delay and sound timers (60Hz)
    delay_timer: u8,
    sound_timer: u8,

    // Program counter (PC) - 16-bit
    program_counter: u16,

    // Stack pointer (SP) - 8-bit
    stack_pointer: u8,

    // Stack (16 16-bit values)
    stack: [u16; 16],

    // Memory
    memory: memory::Memory,
}

impl Chip8 {
    fn new() -> Chip8 {
        Chip8 {
            v_registers: [0; 16],
            i_register: 0,
            delay_timer: 0,
            sound_timer: 0,
            program_counter: 0,
            stack_pointer: 0,
            stack: [0; 16],
            memory: memory::Memory::new(),
        }
    }

    fn execute(&mut self, opcode: u16) {
        match opcode & 0xF000 {
            0x0000 => {}
            0x1000 => self.jump_to(opcode & 0x0FFF),
            0x2000 => self.call_subroutine(opcode & 0x0FFF),
            0x3000 => {
                let x = opcode & 0x0F00 >> 8;
                let byte = opcode & 0x00FF;
                self.skip_if_equal(x as u8, byte as u8);
            }
            0x4000 => {
                let x = opcode & 0x0F00 >> 8;
                let byte = opcode & 0x00FF;
                self.skip_if_not_equal(x as u8, byte as u8);
            }
            0x5000 => {
                let x = opcode & 0x0F00 >> 8;
                let y = opcode & 0x00F0 >> 4;
                self.skip_if_registers_equal(x as u8, y as u8);
            }
            0x6000 => {
                let x = opcode & 0x0F00 >> 8;
                let byte = opcode & 0x00FF;
                self.load_to_register(x as u8, byte as u8);
            }
            0x7000 => {
                let x = opcode & 0x0F00 >> 8;
                let byte = opcode & 0x00FF;
                self.add_to_register(x as u8, byte as u8);
            }
            0x8000 => self.logitcal_op(opcode),
            0x9000 => {
                let x = opcode & 0x0F00 >> 8;
                let y = opcode & 0x00F0 >> 4;
                self.skip_if_registers_not_equal(x as u8, y as u8);
            }
            0xa000 => self.load_i(opcode & 0x0FFF),
            0xb000 => self.jump_with_offset(opcode & 0x0FFF),
            0xc000 => {
                let x = opcode & 0x0F00 >> 8;
                let byte = opcode & 0x00FF;
                self.random_and(x as u8, byte as u8);
            }
            0xd000 => {
                let x = opcode & 0x0F00 >> 8;
                let y = opcode & 0x00F0 >> 4;
                let nibble = opcode & 0x000F;
                self.draw(x as u8, y as u8, nibble as u8);
            }
            0xe000 => self.skip_with_key_status(opcode),
            0xf000 => self.fx_inst(opcode),
        }
    }

    // 1nnn - JP addr
    // Jump to location nnn.
    fn jump_to(&mut self, addr: u16) {
        self.program_counter = addr;
    }

    // 2nnn - CALL addr
    // Call subroutine at nnn.
    fn call_subroutine(&mut self, addr: u16) {
        self.stack_pointer = (self.stack_pointer + 1) % 16;
        self.stack[self.stack_pointer as usize] = self.program_counter;
        self.program_counter = addr;
    }

    // 3xkk - SE Vx, byte
    // Skip next instaruction if Vx == kk.
    fn skip_if_equal(&mut self, x: u8, byte: u8) {
        if self.v_registers[x as usize] == byte {
            self.program_counter += 2;
        }
    }

    // 4xkk - SNE Vx, byte
    // Skip next instaruction if Vx != kk.
    fn skip_if_not_equal(&mut self, x: u8, byte: u8) {
        if self.v_registers[x as usize] != byte {
            self.program_counter += 2;
        }
    }

    // 5xy0 - SE Vx, Vy
    // Skip next instaruction if Vx == Vy.
    fn skip_if_registers_equal(&mut self, x: u8, y: u8) {
        if self.v_registers[x as usize] == self.v_registers[y as usize] {
            self.program_counter += 2;
        }
    }

    // 6xkk - LD Vx, byte
    // Set Vx = kk.
    fn load_to_register(&mut self, x: u8, byte: u8) {
        self.v_registers[x as usize] = byte;
    }

    // 7xkk -ADD Vx, byte
    // Set Vx = Vx + kk.
    fn add_to_register(&mut self, x: u8, byte: u8) {
        self.v_registers[x as usize] = self.v_registers[x as usize].wrapping_add(byte);
    }

    // Logitcal operations
    fn logitcal_op(&mut self, opcode: u16) {
        let x = opcode & 0x0F00 >> 8;
        let y = opcode & 0x00F0 >> 4;
        let c = opcode & 0x000F;
        match c {
            0x0 => self.load_from_to(x, y),
            0x1 => self.or(x, y),
            0x2 => self.and(x, y),
            0x3 => self.xor(x, y),
            0x4 => self.add(x, y),
            0x5 => self.sub(x, y),
            0x6 => self.shr(x, y),
            0x7 => self.subn(x, y),
            0xe => self.shl(x, y),
            _ => panic!("Unknown opcode: {}", opcode),
        }
    }

    // 8xy0 - LD Vx, Vy
    // Set Vx = Vy.
    fn load_from_to(&mut self, x: u8, y: u8) {
        self.v_registers[x as usize] = self.v_registers[y as usize];
    }

    // 8xy1 - OR Vx, Vy
    // Set Vx = Vx OR Vy.
    fn or(&mut self, x: u8, y: u8) {
        self.v_registers[x as usize] |= self.v_registers[y as usize];
    }

    // 8xy2 - AND Vx, Vy
    // Set Vx = Vx AND Vy.
    fn and(&mut self, x: u8, y: u8) {
        self.v_registers[x as usize] &= self.v_registers[y as usize];
    }

    // 8xy3 - XOR Vx, Vy
    // Set Vx = Vx XOR Vy.
    fn xor(&mut self, x: u8, y: u8) {
        self.v_registers[x as usize] ^= self.v_registers[y as usize];
    }

    // 8xy4 - ADD Vx, Vy
    // Set Vx = Vx + Vy, set VF = carry.
    fn add(&mut self, x: u8, y: u8) {
        self.v_registers[0xF] =
            if self.v_registers[x as usize] > (0xFF - self.v_registers[y as usize]) {
                1
            } else {
                0
            };
        self.v_registers[x as usize] =
            self.v_registers[x as usize].wrapping_add(self.v_registers[y as usize]);
    }

    // 8xy5 - SUB Vx, Vy
    // Set Vx = Vx - Vy, set VF = NOT borrow.
    fn sub(&mut self, x: u8, y: u8) {
        self.v_registers[0xF] = if self.v_registers[x as usize] > self.v_registers[y as usize] {
            1
        } else {
            0
        };
        self.v_registers[x as usize] =
            self.v_registers[x as usize].wrapping_sub(self.v_registers[y as usize]);
    }

    // 8xy6 - SHR Vx {, Vy}
    // Set Vx = Vx SHR 1.
    fn shr(&mut self, x: u8, y: u8) {
        self.v_registers[0xF] = self.v_registers[x as usize] & 0x1;
        self.v_registers[x as usize] >>= 1;
    }

    // 8xy7 - SUBN Vx, Vy
    // Set Vx = Vy - Vx, set VF = NOT borrow.
    fn subn(&mut self, x: u8, y: u8) {
        self.v_registers[0xF] = if self.v_registers[y as usize] > self.v_registers[x as usize] {
            1
        } else {
            0
        };
        self.v_registers[x as usize] =
            self.v_registers[y as usize].wrapping_sub(self.v_registers[x as usize]);
    }

    // 8xyE - SHL Vx {, Vy}
    // Set Vx = Vx SHL 1.
    fn shl(&mut self, x: u8, y: u8) {
        self.v_registers[0xF] = (self.v_registers[x as usize] & 0x80) >> 7;
        self.v_registers[x as usize] <<= 1;
    }

    // 9xy0 - SNE Vx, Vy
    // Skip next instruction if Vx != Vy.
    fn skip_if_registers_not_equal(&mut self, x: u8, y: u8) {
        if self.v_registers[x as usize] != self.v_registers[y as usize] {
            self.program_counter += 2;
        }
    }

    // Annn - LD I, addr
    // Set I = nnn.
    fn load_i(&mut self, nnn: u16) {
        self.i_registers = nnn;
    }

    // Bnnn - JP V0, addr
    // Jump to location nnn + V0.
    fn jump_with_offset(&mut self, nnn: u16) {
        self.program_counter = nnn + self.v_registers[0] as u16;
    }

    // Cxkk - RND Vx, byte
    // Set Vx = random byte AND kk.
    fn random_and(&mut self, x: u8, kk: u8) {
        let mut rng = rand::thread_rng();
        let byte: u8 = rng.gen();
        self.v_registers[x as usize] = byte & kk;
    }

    // Dxyn - DRW Vx, Vy, nibble
    // Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision.
    fn draw(&mut self, x: u8, y: u8, nibble: u8) {
        let mut sprite: [u8; 16] = [0; 16];
        for i in 0..(nibble & 0x0F) {
            if let Some(v) = self.memory.access((self.i_registers + i as u16) as usize) {
                sprite[i as usize] = *v;
            } else {
                panic!(
                    "Invalid memory address: 0x{:X}.",
                    self.i_registers + i as u16
                );
            }
        }
        // TODO: Implement collision detection.
    }

    // Skip next instruction if key pressed or not.
    fn skip_with_key_status(&mut self, opcode: u16) {
        let x = opcode & 0x0F00 >> 8;
        let low_byte = opcode & 0x00FF;
        match low_byte {
            0x9E => {}
            0xA1 => {}
            _ => panic!("Invalid opcode: 0x{:X}.", opcode),
        }
    }

    // Ex9E - SKP Vx
    // Skip next instruction if key with the value of Vx is pressed.
    fn skip_if_key_pressed(&mut self, x: u8) {
        // TODO: Implement key press detection.
    }

    // ExA1 - SKNP Vx
    // Skip next instruction if key with the value of Vx is not pressed.
    fn skip_if_key_not_pressed(&mut self, x: u8) {
        // TODO: Implement key press detection.
    }

    // Fx** instructions
    fn fx_inst(&mut self, opcode: u16) {
        let x = opcode & 0x0F00 >> 8;
        let low_byte = opcode & 0x00FF;
        match low_byte {
            0x07 => self.load_delay_timer(x),
            0x0A => self.wait_for_key_press(x),
            0x15 => self.set_delay_timer(x),
            0x18 => self.set_sound_timer(x),
            0x1E => self.add_to_i_register(x),
            0x29 => self.set_i_register(x),
            0x33 => self.store_bcd(x),
            0x55 => self.store_registers(x),
            0x65 => self.load_registers(x),
            _ => panic!("Invalid opcode: 0x{:X}.", opcode),
        }
    }

    // Fx07 - LD Vx, DT
    // Set Vx = delay timer value.
    fn load_delay_timer(&mut self, x: u8) {
        self.v_registers[x as usize] = self.delay_timer;
    }

    // Fx0A - LD Vx, K
    // Wait for a key press, store the value of the key in Vx.
    fn wait_for_key_press(&mut self, x: u8) {
        // TODO: Implement key press detection.
    }

    // Fx15 - LD DT, Vx
    // Set delay timer = Vx.
    fn set_delay_timer(&mut self, x: u8) {
        self.delay_timer = self.v_registers[x as usize];
    }

    // Fx18 - LD ST, Vx
    // Set sound timer = Vx.
    fn set_sound_timer(&mut self, x: u8) {
        self.sound_timer = self.v_registers[x as usize];
    }

    // Fx1E - ADD I, Vx
    // Set I = I + Vx.
    fn add_to_i_register(&mut self, x: u8) {
        if let Some(value) = self.memory.access(self.i_registers as usize) {
            self.memory.assign(
                self.i_registers as usize,
                value.wrapping_add(self.v_registers[x as usize] as u8),
            );
        } else {
            eprintln!("Invalid memory address: 0x{:X}.", self.i_registers);
        }
    }

    // Fx29 - LD F, Vx
    // Set I = location of sprite for digit Vx.
    fn set_i_register(&mut self, x: u8) {
        // TODO: Implement sprite loading.
    }

    // Fx33 - LD B, Vx
    // Store BCD representation of Vx in memory locations I, I+1, and I+2.
    fn store_bcd(&mut self, x: u8) {
        self.memory.assign(
            self.i_registers as usize,
            self.v_registers[x as usize] / 100,
        );
        self.memory.assign(
            (self.i_registers + 1) as usize,
            (self.v_registers[x as usize] / 10) % 10,
        );
        self.memory.assign(
            (self.i_registers + 2) as usize,
            self.v_registers[x as usize] % 10,
        );
    }

    // Fx55 - LD [I], Vx
    // Store registers V0 through Vx in memory starting at location I.
    fn store_registers(&mut self, x: u8) {
        for i in 0..=x as usize {
            self.memory
                .assign(self.i_registers as usize + i, self.v_registers[i]);
        }
    }

    // Fx65 - LD Vx, [I]
    // Read registers V0 through Vx from memory starting at location I.
    fn load_registers(&mut self, x: u8) {
        for i in 0..=x as usize {
            if let Some(value) = self.memory.access(self.i_registers as usize + i) {
                self.v_registers[i] = value;
            }
        }
    }
}
