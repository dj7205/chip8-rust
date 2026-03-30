use crate::cpu::CPU;
use std::{fs::File, io::Read};
pub struct Emulator {
    cpu: CPU,
    memory: [u8; 4096],

    display: [bool; 64 * 32], //screen with width=64 and heigt=32 pixels
}

impl Emulator {
    //Initialize the emulator state
    pub fn initialize() -> Self {
        let mut emulator = Emulator {
            cpu: CPU::new(),
            memory: [0; 4096],
            display: [false; 64 * 32],
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
        //TODO: Keypad?
        emulator.memory[0..50].copy_from_slice(&fontset); //load fontset into memory
        emulator
    }

    pub fn load_game(&mut self, path: &String) {
        let mut game = File::open(path).expect("Unable to open File. File doesn't exist");
        let mut buffer = Vec::new();
        game.read_to_end(&mut buffer);
        self.memory[0x200..0x200+buffer.len()].copy_from_slice(&buffer);
    
    }

    pub fn start_emulation(&self) {
        loop {}
    }

    //get the next opcode from memory
    fn fetch(&mut self) -> u16{
        //TODO
        let upper = self.memory[self.cpu.pc as usize];
        let lower = self.memory[(self.cpu.pc + 1) as usize];
        self.cpu.pc += 2;
        
        (upper as u16) << 8 | (lower as u16)
    }

    //execute the operation
    fn execute(&self, op: u16) {
        unimplemented!()
    }

    //perform one fetch/execute cycle
    pub fn tick(&mut self) {
        let op = self.fetch();
        self.execute(op);
    }
}
