pub struct Chip8 {
    pub vram: [u8; 2048],
    mem: [u8; 4096],
    v: [u8; 16],
    pc: u16,
    i: u16,
    dt: u8,
    st: u8
}

impl Chip8 {
    pub fn load(file: &str) -> Self {
        let mem = std::fs::read(file).expect("Failed to load file");
        let mut c8 = Chip8 {
            vram: [0u8; 2048],
            mem: [0u8; 4096],
            v: [0u8; 16],
            pc: 512,
            i: 0,
            dt: 0,
            st: 0,
        };

        c8.mem[512..512 + mem.len()].copy_from_slice(&mem[..]);

        c8
    }

    pub fn execute(&mut self) {
        let mut ins = [0u8; 4];

        ins[0] = self.mem[self.pc as usize] >> 4;
        ins[1] = self.mem[self.pc as usize] & 0xF;

        ins[2] = self.mem[(self.pc + 1) as usize] >> 4;
        ins[3] = self.mem[(self.pc + 1) as usize] & 0xF;

        self.pc += 2;

        println!("Executing {:?}", ins);

        match ins {
            [0x0, 0x0, 0xE, 0x0] => {
                self.op_00e0();
            }
            [0x1, n3, n2, n1] => {
                let nnn = (n3 as u16) << 8 | (n2 as u16) << 4 | n1 as u16;
                self.op_1nnn(nnn);
            }
            [0x6, x, n2, n1] => {
                let nn = n2 << 4 | n1;
                self.op_6xnn(x, nn);
            }
            [0x7, x, n2, n1] => {
                let nn = n2 << 4 | n1;
                self.op_7xnn(x, nn);
            }
            [0xA, _, _, _] => {
                let nnn = (ins[1] as u16) << 8 | (ins[2] as u16) << 4 | ins[3] as u16;
                self.op_annn(nnn);
            }
            [0xD, x, y, n] => {
                self.op_dxyn(x, y, n);
            }

            _ => { panic!("Bad opcode") }
        }
    }

    fn op_00e0(&mut self) {
        self.vram[..].fill(0);
    }

    fn op_1nnn(&mut self, nnn: u16) {
        self.pc = nnn;
    }

    fn op_6xnn(&mut self, x: u8, nn: u8) {
        self.v[x as usize] = nn;
    }

    fn op_7xnn(&mut self, x: u8, nn: u8) {
        self.v[x as usize] = u8::wrapping_add(self.v[x as usize], nn);
    }

    fn op_annn(&mut self, nnn: u16) {
        self.i = nnn;
    }

    fn op_dxyn(&mut self, x: u8, y: u8, n: u8) {        
        for row in 0..n {
            let sprite: u8 = self.mem[(self.i as usize + row as usize)];

            for bit in (0..8).rev() {
                let mask: u8 = 1 << bit;

                let dx: usize = ((self.v[x as usize] + 7 - bit) % 64).into();
                let dy: usize = ((self.v[y as usize] + row) % 32).into();



                self.vram[dx + dy * 64] ^= (sprite & mask) >> (bit);
            }
        }
    }
}