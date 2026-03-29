
pub struct CPU {
    pc : u16,
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
}


pub struct Register {
    v : [u8; 16],
    i : u16,
}