use super::stack::Stack;

const SCREEN_WIDTH: usize = 64;
const SCREEN_HEIGHT: usize = 32;

fn components(x: u16) -> (u16, u16, u16, u16) {
    (x >> 12 & 0xF, x >> 08 & 0xF, x >> 04 & 0xF, x >> 00 & 0xF)
}

fn combine3(n1: u16, n2: u16, n3: u16) -> u16 {
    n1 << 8 + n2 << 4 + n3 << 0
}

fn combine2(n1: u16, n2: u16) -> u16 {
    n1 << 4 + n2 << 0
}

pub struct Chip8 {
    v: [u8; 16], // General registers
    i: u16,

    pc: u16, // ProgramCounter
    sp: u8,  // Stack Pointer
    stack: Stack<u16>,

    dt: u8, // Delay Timer
    st: u8, // Sound timer

    display: [bool; SCREEN_WIDTH * SCREEN_HEIGHT],

    current_opcode: u16,
}

impl Chip8 {
    pub fn new() -> Self {
        Self {
            v: [0; 16],
            i: 0,
            pc: 0,
            sp: 0,
            stack: Stack::new(),
            dt: 0,
            st: 0,
            display: [false; SCREEN_WIDTH * SCREEN_HEIGHT],
            current_opcode: 0,
        }
    }

    fn tick(&mut self) {
        // TODO
        self.current_opcode = 0xFFFF;

        match components(self.current_opcode) {
            // (0x0, _, _, _) => self.(),
            (0x0, 0x0, 0xE, 0x0) => self.cls(),
            (0x0, 0x0, 0xE, 0xE) => self.ret(),
            (0x1, n1, n2, n3) => self.jp(combine3(n1, n2, n3)),
            (0x2, n1, n2, n3) => self.call(combine3(n1, n2, n3)),
            (0x3, x, k1, k2) => self.se(x, combine2(k1, k2)),
            (0x4, x, k1, k2) => self.sne(x, combine2(k1, k2)),
            (0x5, x, y, 0x0) => self.se2(x, y),
            (0x6, x, k1, k2) => self.ld(x, combine2(k1, k2)),
            (0x7, x, k1, k2) => self.add(x, combine2(k1, k2)),
            (0x8, x, y, 0x0) => self.ld(x, y),
            (0x8, x, y, 0x1) => self.or(x, y),
            (0x8, x, y, 0x2) => self.and(x, y),
            (0x8, x, y, 0x3) => self.xor(x, y),
            (0x8, x, y, 0x4) => self.add_carry(x, y),
            (0x8, x, y, 0x5) => self.sub(x, y),
            (0x8, x, y, 0x6) => self.shr(x, y),
            (0x8, x, y, 0x7) => self.subn(x, y),
            (0x8, x, y, 0xE) => self.shl(x, y),
            (0x9, x, y, 0x0) => self.sne2(x, y),
            (0xA, n1, n2, n3) => self.ldi(combine3(n1, n2, n3)),
            (0xB, n1, n2, n3) => self.jp1(combine3(n1, n2, n3)),
            (0xC, x, k1, k2) => self.rnd(x, combine2(k1, k2)),
            (0xD, x, y, n) => self.drw(x, y, n),
            (0xE, x, 0x9, 0xE) => self.skp(x),
            (0xE, x, 0xA, 0x1) => self.sknp(x),
            (0xF, x, 0x0, 0x7) => self.ld_xvdt(x),
            (0xF, x, 0x0, 0xA) => self.ld_keypres(x),
            (0xF, x, 0x1, 0x5) => self.ld_dtxv(x),
            (0xF, x, 0x1, 0x8) => self.ld_st(x),
            (0xF, x, 0x1, 0xE) => self.addi(x),
            (0xF, x, 0x2, 0x9) => self.ldf(x),
            (0xF, x, 0x3, 0x3) => self.ldb(x),
            (0xF, x, 0x5, 0x5) => self.ldivx(x),
            (0xF, x, 0x6, 0x5) => self.ldvxi(x),

            _ => todo!(),
        }
    }
}

// Instructions
impl Chip8 {
    pub fn cls(&mut self) {}

    pub fn ret(&mut self) {}

    pub fn jp(&mut self, address: u16) {}

    pub fn call(&mut self, address: u16) {}

    pub fn se(&mut self, v: u16, value: u16) {}

    pub fn sne(&mut self, v: u16, value: u16) {}

    pub fn se2(&mut self, vx: u16, vy: u16) {}

    pub fn ld(&mut self, v: u16, value: u16) {}

    pub fn add(&mut self, v: u16, value: u16) {}

    pub fn ld2(&mut self, vx: u16, vy: u16) {}

    pub fn or(&mut self, vx: u16, vy: u16) {}

    pub fn and(&mut self, vx: u16, vy: u16) {}

    pub fn xor(&mut self, vx: u16, vy: u16) {}

    pub fn add_carry(&mut self, vx: u16, vy: u16) {}

    pub fn sub(&mut self, vx: u16, vy: u16) {}

    pub fn shr(&mut self, vx: u16, vy: u16) {}

    pub fn subn(&mut self, vx: u16, vy: u16) {}

    pub fn shl(&mut self, vx: u16, vy: u16) {}

    pub fn sne2(&mut self, vx: u16, vy: u16) {}

    pub fn ldi(&mut self, address: u16) {}

    pub fn jp1(&mut self, address: u16) {}

    pub fn rnd(&mut self, v: u16, value: u16) {}

    pub fn drw(&mut self, vx: u16, vy: u16, n: u16) {}

    pub fn skp(&mut self, v: u16) {}

    pub fn sknp(&mut self, v: u16) {}

    pub fn ld_xvdt(&mut self, v: u16) {}

    pub fn ld_keypres(&mut self, v: u16) {}

    pub fn ld_dtxv(&mut self, v: u16) {}

    pub fn ld_st(&mut self, v: u16) {}

    pub fn addi(&mut self, v: u16) {}

    pub fn ldf(&mut self, v: u16) {}

    pub fn ldb(&mut self, v: u16) {}

    pub fn ldivx(&mut self, v: u16) {}

    pub fn ldvxi(&mut self, v: u16) {}
}
