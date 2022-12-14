/// CHIP-8 Tech Ref: <http://devernay.free.fr/hacks/chip8/C8TECH10.HTM/>
struct Cpu {
    mem: [u8; 4096],
    pos_in_mem: usize,
    registers: [u8; 16],
    stack: [u16; 16],
    stack_ptr: usize,
}

impl Cpu {
    fn run(&mut self) {
        loop {
            let op_byte1 = u16::from(self.mem[self.pos_in_mem]);
            let op_byte2 = u16::from(self.mem[self.pos_in_mem + 1]);
            let opcode: u16 = op_byte1 << 8 | op_byte2;

            let x = ((opcode & 0x0F00) >> 8) as u8;
            let y = ((opcode & 0x00F0) >> 4) as u8;
            let kk = (opcode & 0x00FF) as u8;
            let op_minor = (opcode & 0x000F) as u8;
            let addr = opcode & 0x0FFF;

            self.pos_in_mem += 2;

            match opcode {
                0x0000 => {
                    return;
                }
                0x00E0 => {}
                0x00EE => {
                    self.ret();
                }
                0x1000..=0x1FFF => {
                    self.jmp(addr);
                }
                0x2000..=0x2FFF => {
                    self.call(addr);
                }
                0x3000..=0x3FFF => {
                    self.se(x, kk);
                }
                0x4000..=0x4FFF => {
                    self.sne(x, kk);
                }
                0x5000..=0x5FFF => {
                    self.se(x, y);
                }
                0x6000..=0x6FFF => {
                    self.ld(x, kk);
                }
                0x7000..=0x7FFF => {
                    self.add(x, kk);
                }
                0x8000..=0x8FFF => match op_minor {
                    0 => self.ld(x, self.registers[y as usize]),
                    1 => self.or_xy(x, y),
                    2 => self.and_xy(x, y),
                    3 => self.xor_xy(x, y),
                    4 => {
                        self.add_xy(x, y);
                    }
                    _ => {
                        todo!("opcode: {:04x}", opcode);
                    }
                },
                _ => todo!("opcode {:04x}", opcode),
            }
        }
    }

    fn ld(&mut self, vx: u8, kk: u8) {
        self.registers[vx as usize] = kk;
    }

    fn add(&mut self, vx: u8, kk: u8) {
        self.registers[vx as usize] += kk;
    }

    fn add_xy(&mut self, x: u8, y: u8) {
        self.registers[x as usize] += self.registers[y as usize];
    }

    fn and_xy(&mut self, x: u8, y: u8) {
        let x_ = self.registers[x as usize];
        let y_ = self.registers[y as usize];

        self.registers[x as usize] = x_ & y_;
    }

    fn or_xy(&mut self, x: u8, y: u8) {
        let x_ = self.registers[x as usize];
        let y_ = self.registers[y as usize];

        self.registers[x as usize] = x_ | y_;
    }

    fn xor_xy(&mut self, x: u8, y: u8) {
        let x_ = self.registers[x as usize];
        let y_ = self.registers[y as usize];

        self.registers[x as usize] = x_ ^ y_;
    }

    fn se(&mut self, vx: u8, kk: u8) {
        if vx == kk {
            self.pos_in_mem += 2;
        }
    }

    fn sne(&mut self, vx: u8, kk: u8) {
        if vx != kk {
            self.pos_in_mem += 2;
        }
    }

    fn jmp(&mut self, addr: u16) {
        self.pos_in_mem = addr as usize;
    }

    fn call(&mut self, addr: u16) {
        let sp = self.stack_ptr;
        let stack = &mut self.stack;

        assert!(sp < stack.len(), "Stack overflow");

        stack[sp] = self.pos_in_mem as u16;
        self.stack_ptr += 1;
        self.pos_in_mem = addr as usize;
    }

    fn ret(&mut self) {
        assert!(self.stack_ptr != 0, "Stack underflow");

        self.stack_ptr -= 1;
        self.pos_in_mem = self.stack[self.stack_ptr] as usize;
    }
}

fn main() {
    let mut cpu = Cpu {
        mem: [0; 4096],
        pos_in_mem: 0,
        registers: [0; 16],
        stack: [0; 16],
        stack_ptr: 0,
    };

    cpu.registers[0] = 5;
    cpu.registers[1] = 10;

    cpu.mem[0x000] = 0x21;
    cpu.mem[0x001] = 0x00;
    cpu.mem[0x002] = 0x21;
    cpu.mem[0x003] = 0x00;

    cpu.mem[0x100] = 0x80;
    cpu.mem[0x101] = 0x14;
    cpu.mem[0x102] = 0x80;
    cpu.mem[0x103] = 0x14;
    cpu.mem[0x104] = 0x00;
    cpu.mem[0x105] = 0xEE;

    cpu.run();

    assert_eq!(cpu.registers[0], 45);

    println!("5 + (10 * 2) + (10 * 2) = {}", cpu.registers[0]);
}
