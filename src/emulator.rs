use crate::cpu::CPU;
use std::{fs::File, io::Read};
pub struct Emulator {
    cpu: CPU,
    memory: [u8; 4096],
    //TODO keypad
    screen: [bool; 64 * 32], //screen with width=64 and heigt=32 pixels
}

impl Emulator {
    //Initialize the emulator state
    pub fn initialize() -> Self {
        let mut emulator = Emulator {
            cpu: CPU::new(),
            memory: [0; 4096],
            screen: [false; 64 * 32],
        };
        let fontset = [
            0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
            0x20, 0x60, 0x20, 0x20, 0x70, // 1
            0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
            0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
            0x90, 0x90, 0xF0, 0x10, 0x10, // 4
            0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
            0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
            0xF0, 0x10, 0x20, 0x40, 0x40, // 7
            0xF0, 0x90, 0xF0, 0x90, 0xF0, //8
            0xF1, 0x90, 0xF1, 0x10, 0xF1, //9
        ];
        emulator.memory[0..50].copy_from_slice(&fontset); //load fontset into memory
        emulator
    }

    //load the ROM into RAM
    pub fn load_game(&mut self, path: &String) {
        let mut game = File::open(path).expect("Unable to open File. File doesn't exist");
        let mut buffer = Vec::new();
        game.read_to_end(&mut buffer).unwrap();
        self.memory[0x200..0x200 + buffer.len()].copy_from_slice(&buffer);
    }

    pub fn get_screen(&self) -> &[bool] {
        &self.screen
    }

    //get the next opcode from memory
    fn fetch(&mut self) -> u16 {
        let upper_byte = self.memory[self.cpu.get_pc() as usize];
        let lower_byte = self.memory[(self.cpu.get_pc() + 1) as usize];
        self.cpu.inc_pc(2);

        (upper_byte as u16) << 8 | (lower_byte as u16)
    }

    //execute the operation
    fn execute(&mut self, op: u16) {
        let first_digit = (op & 0xF000) >> 12;
        let second_digit = (op & 0x0F00) >> 8;
        let third_digit = (op & 0x0F0) >> 4;
        let fourth_digit = (op & 0x000F);

        println!("opcode= {0:x}", op);
        
        //OPCODES
        match (first_digit, second_digit, third_digit, fourth_digit) {
            (0, 0, 0xE, 0) => self.screen = [false; 64 * 32], //clear_screen
            (1, _, _, _) => {
                //jump
                let nnn = op & 0xFFF;
                self.cpu.set_pc(nnn);
            }
            (6, X, _, _) => {
                //set register vx
                let nn = (op & 0xFF) as u8;
                self.cpu.set_v(X as usize, nn);
            }
            (7, X, _, _) => {
                //add to register vx
                let nn = (op & 0xFF) as u8;
                let vx = self.cpu.get_v(X as usize);
                let sum = vx.wrapping_add(nn);

                self.cpu.set_v(X as usize, sum);
            }
            (A, _, _, _) => {
                //set index register
                let nnn = op & 0xFFF;
                self.cpu.set_i(nnn);
            }
            (D, X, Y, N) => {
                let vx = self.cpu.get_v(X as usize) as u16;
                let vy = self.cpu.get_v(Y as usize) as u16;

                let mut flip = false;
                for y_line in 0..N {
                    //get sprite data
                    let addr = self.cpu.get_i() + y_line;
                    let pixel = self.memory[addr as usize];
                    for x_line in 0..8 {
                        if (pixel & (0b1000_0000 >> x_line)) != 0 {
                            let x = (vx + x_line) as usize % 64;
                            let y = (vy + y_line) as usize % 32;
                            let idx = x + 64 * y;

                            flip |= self.screen[idx];
                            self.screen[idx] ^= true;
                        }
                    }
                }
                if flip {
                    self.cpu.set_v(0xF, 1);
                } else {
                    self.cpu.set_v(0xF, 0);
                }
            }

            _ => todo!("this instruction is currently not covered!"),
        }
    }

    //perform one fetch/execute cycle
    pub fn tick(&mut self) {
        let op = self.fetch();
        self.execute(op);
    }
}
