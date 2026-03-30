
pub struct Register {
    v : [u8; 16],
    i : u16,
}

pub struct CPU {
    pub pc : u16,
    registers : Register,
    sp : u16,
    stack : [u16; 16],

}

impl CPU {
    pub fn new() -> Self {
        CPU {
            pc: 0x200, // Program counter starts at 0x200
            registers: Register {
                v: [0; 16], // 16 general-purpose registers
                i: 0,       // Index register
            },
            sp: 0, // Stack pointer starts at 0
            stack: [0; 16], // Stack to hold return addresses
        }
    }

    //put value on top of stack and refresh its pointer
    pub fn push(&mut self, value: u16) {
    self.stack[self.sp as usize] = value;
    self.sp += 1;
    }

    //return the value on top of the stack
    pub fn pop(&mut self) -> u16 {
        self.sp -= 1;
        self.stack[self.sp as usize]
    }
}

//OPCODES