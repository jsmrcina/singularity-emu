use crate::bus::main_bus::MainBus;
use crate::traits::{ReadWrite, Clockable, Resettable};
use crate::cartridge::cart::Cart;
use std::fmt;
use std::sync::{Arc, Mutex};
use std::collections::BTreeMap;

include!("instructions.rs");

#[repr(u8)]
pub enum Flags6502
{
    C = (1 << 0), // Carry Bit
    Z = (1 << 1), // Zero
    I = (1 << 2), // Disable Interrupts
    D = (1 << 3), // Decimal Mode (unused)
    B = (1 << 4), // Break
    U = (1 << 5), // Unused
    V = (1 << 6), // Overflow
    N = (1 << 7), // Negative
}

struct Instruction
{
    name: String,
    op: fn(&mut Cpu6502) -> u8,
    addr_mode: fn(&mut Cpu6502) -> u8,
    cycles: u8
}

pub struct Cpu6502
{
    bus: Option<Arc<Mutex<MainBus>>>,
    cartridge: Option<Arc<Mutex<Cart>>>,
    a: u8,
    x: u8,
    y: u8,
    stkp: u8,
    pc: u16,
    status: u8,
    fetched_data: u8,
    addr_abs: u16,
    addr_rel: u16,
    opcode: u8,
    cycles: u8,
    total_cycles: i64,
    ins: [Instruction; 256]
}

// Note: We are only doing address comparisons within this class and they are
// the easiest way to implement this. I might revisit this, but it isn't causing issues.
#[allow(clippy::fn_address_comparisons)]
impl Cpu6502
{
    const STACK_START_ADDRESS: u16 = 0x0100;
    const INTERRUPT_VECTOR: [u16; 2] = [0xFFFE, 0xFFFF];

    pub fn set_bus(&mut self, bus: Option<Arc<Mutex<MainBus>>>)
    {
        self.bus = bus;
    }

    pub fn connect_cartridge(&mut self, cartridge: Arc<Mutex<Cart>>)
    {
        self.cartridge = Some(cartridge);
    }

    pub fn get_flag(&self, f: Flags6502) -> u8
    {
        if (self.status & (f as u8)) > 0
        {
            1
        }
        else
        {
            0
        }
    }

    pub fn set_flag(&mut self, f: Flags6502, b: bool)
    {
        if b
        {
            self.status |= f as u8;
        }
        else
        {
            self.status &= !(f as u8)
        }
    }

    pub fn cpu_read_u16_from_pc(&mut self) -> (bool, u16, u16, u16)
    {
        let mut lo: u8 = 0;
        let mut hi: u8 = 0;

        self.cpu_read(self.pc, &mut lo);
        self.pc += 1;
        self.cpu_read(self.pc, &mut hi);
        self.pc += 1;

        let data: u16 = ((hi as u16) << 8) | (lo as u16);
        (true, hi as u16, lo as u16, data)
    }

    pub fn get_pc(&self) -> u16
    {
        self.pc
    }

    pub fn get_a(&self) -> u8
    {
        self.a
    }

    pub fn get_x(&self) -> u8
    {
        self.x
    }

    pub fn get_y(&self) -> u8
    {
        self.y
    }

    pub fn get_stkp(&self) -> u8
    {
        self.stkp
    }

    // #region Addressing Modes
    pub fn imp(&mut self) -> u8
    {
        // The instruction may operate on the accumulator
        self.fetched_data = self.a;
        0
    }

    pub fn imm(&mut self) -> u8
    {
        self.addr_abs = self.pc;
        self.pc += 1;
        0
    }

    pub fn zp0(&mut self) -> u8
    {
        // Address in the zero page (plus a byte offset that is read)
        let mut addr_abs_u8: u8 = 0;
        self.cpu_read(self.pc, &mut addr_abs_u8);
        self.addr_abs = addr_abs_u8 as u16;
        self.pc += 1;
        self.addr_abs &= 0x00FF;
        0
    }

    pub fn zpx(&mut self) -> u8
    {
        // Address in the zero page (plus a byte offset that is read and incremented by X register)
        // This is used for iterating through ranges in zero page
        let mut addr_abs_u8: u8 = 0;
        self.cpu_read(self.pc, &mut addr_abs_u8);
        self.addr_abs = addr_abs_u8 as u16 + self.x as u16;
        self.pc += 1;
        self.addr_abs &= 0x00FF;
        0
    }

    pub fn zpy(&mut self) -> u8
    {
        // Address in the zero page (plus a byte offset that is read and incremented by Y register)
        let mut addr_abs_u8: u8 = 0;
        self.cpu_read(self.pc, &mut addr_abs_u8);
        self.addr_abs = addr_abs_u8 as u16 + self.y as u16;
        self.pc += 1;
        self.addr_abs &= 0x00FF;
        0
    }

    pub fn rel(&mut self) -> u8
    {
        let mut addr_rel_u8: u8 = 0;
        self.cpu_read(self.pc, &mut addr_rel_u8);
        self.addr_rel = addr_rel_u8 as u16;
        self.pc += 1;

        // If we have a negative offset, we need to set upper bits to 1
        if self.addr_rel & 0x80 == 0x80
        {
            self.addr_rel |= 0xFF00;
        }

        0
    }

    pub fn abs(&mut self) -> u8
    {
        // Absolute addressing (full u16 address)
        (_, _, _, self.addr_abs) = self.cpu_read_u16_from_pc();
        0 
    }

    pub fn abx(&mut self) -> u8
    {
        // Absolute addressing (full u16 address) with X offset
        let hi: u16;
        (_, hi, _, self.addr_abs) = self.cpu_read_u16_from_pc();
        self.addr_abs = self.addr_abs.wrapping_add(self.x as u16);
    
        // If this crosses a page boundary, we need an additional clock cycle for this instruction
        if (self.addr_abs & 0xFF00) != (hi << 8)
        {
            1
        }
        else
        {
            0	
        }
    }

    pub fn aby(&mut self) -> u8
    {
        // Absolute addressing (full u16 address) with Y offset
        let hi: u16;
        (_, hi, _, self.addr_abs) = self.cpu_read_u16_from_pc();
        self.addr_abs = self.addr_abs.wrapping_add(self.y as u16);

        // If this crosses a page boundary, we need an additional clock cycle for this instruction
        if (self.addr_abs & 0xFF00) != (hi << 8)
        {
            1
        }
        else
        {
            0	
        }
    }

    pub fn ind(&mut self) -> u8
    {
        let ptr: u16;
        let ptr_lo: u16;
        (_, _, ptr_lo, ptr) = self.cpu_read_u16_from_pc();

        let mut lo: u8 = 0;
        let mut hi: u8 = 0;
        if ptr_lo == 0x00FF
        {
            self.cpu_read(ptr, &mut lo);
            self.cpu_read(ptr & 0xFF00, &mut hi);

            self.addr_abs = ((hi as u16) << 8) | lo as u16;
        }
        else
        {
            self.cpu_read(ptr, &mut lo);
            self.cpu_read(ptr + 1, &mut hi);

            self.addr_abs = ((hi as u16) << 8) | lo as u16;
        }

        0
    }

    // The read 8-bit address is offset by X and used to read the 16-bit absolute address
    pub fn izx(&mut self) -> u8
    {
        let mut t : u8 = 0;
        self.cpu_read(self.pc, &mut t);
        let t_u16 = t as u16;
	    self.pc += 1;

        let mut lo: u8 = 0;
        self.cpu_read((t_u16 + self.x as u16) & 0x00FF, &mut lo);
        let mut hi: u8 = 0;
        self.cpu_read((t_u16 + self.x as u16 + 1) & 0x00FF, &mut hi);

        self.addr_abs = ((hi as u16) << 8) | lo as u16;

        0
    }

    // The read 8-bit address is read from the zero page as a 16-bit address, then offset by Y
    // If the offset changes a page, we need 1 additional clock cycle
    pub fn izy(&mut self) -> u8
    {
        let mut t : u8 = 0;
        self.cpu_read(self.pc, &mut t);
        let t_u16 = t as u16;
	    self.pc += 1;

        let mut lo: u8 = 0;
        let mut hi: u8 = 0;

        self.cpu_read(t_u16 & 0x00FF, &mut lo);
        self.cpu_read((t_u16 + 1) & 0x00FF, &mut hi);
        self.addr_abs = (lo as u16) | ((hi as u16) << 8);
        self.addr_abs = self.addr_abs.wrapping_add(self.y as u16);

        if (self.addr_abs & 0xFF00) != ((hi as u16) << 8)
        {
            1
        }
        else
        {
            0
        }
    }

    // Opcodes
    // Instruction: Add with carry
    // A += M + C
    pub fn adc(&mut self) -> u8 
    {
        self.fetch();
        let temp: u16 = self.a as u16 + self.fetched_data as u16 + self.get_flag(Flags6502::C) as u16;

        // If we overflow into 16-bit range, set the carry bit
        self.set_flag(Flags6502::C, temp > 255);
        self.set_flag(Flags6502::Z, (temp & 0x00FF) == 0);

        // Taken from https://github.com/OneLoneCoder/olcNES/blob/master/Part%232%20-%20CPU/olc6502.cpp, this excellent guide explains
        // how V gets set:

        // Here we have not gone out of range. The resulting significant bit has not changed.
        // So let's make a truth table to understand when overflow has occurred. Here I take
        // the MSB of each component, where R is RESULT.
        //
        // A  M  R | V | A^R | A^M |~(A^M) | 
        // 0  0  0 | 0 |  0  |  0  |   1   |
        // 0  0  1 | 1 |  1  |  0  |   1   |
        // 0  1  0 | 0 |  0  |  1  |   0   |
        // 0  1  1 | 0 |  1  |  1  |   0   |  so V = ~(A^M) & (A^R)
        // 1  0  0 | 0 |  1  |  1  |   0   |
        // 1  0  1 | 0 |  0  |  1  |   0   |
        // 1  1  0 | 1 |  1  |  0  |   1   |
        // 1  1  1 | 0 |  0  |  0  |   1   |
        //
        // We can see how the above equation calculates V, based on A, M and R. V was chosen
        // based on the following hypothesis:
        //       Positive Number + Positive Number = Negative Result -> Overflow
        //       Negative Number + Negative Number = Positive Result -> Overflow
        //       Positive Number + Negative Number = Either Result -> Cannot Overflow
        //       Positive Number + Positive Number = Positive Result -> OK! No Overflow
        //       Negative Number + Negative Number = Negative Result -> OK! NO Overflow
        let v: u16 = (!(self.a as u16 ^ self.fetched_data as u16)) & ((self.a as u16 ^ temp) & 0x0080);
        self.set_flag(Flags6502::V, v != 0);

        self.set_flag(Flags6502::N, temp & 0x80 == 0x80);

        // Load the result into the accumulator
        self.a = (temp & 0x00FF) as u8;

        1
    }

    // Instruction: Bitwise Logic AND
    // A = A & M
    pub fn and(&mut self) -> u8
    {
        self.fetch();
        self.a &= self.fetched_data;
        self.set_flag(Flags6502::Z, self.a == 0x00); // Zero
        self.set_flag(Flags6502::N, self.a & 0x80 == 0x80); // Negative
        1
    }
    
    // Instruction: Arithmetic Shift Left
    // A = (A << 1), where C is 1 if left shift overflows 8-bit number
    pub fn asl(&mut self) -> u8
    {
        self.fetch();
        let temp: u16 = (self.fetched_data as u16) << 1;
        self.set_flag(Flags6502::C, (temp & 0xFF00) > 0); // Carry
        self.set_flag(Flags6502::Z, (temp & 0x00FF) == 0x00); // Zero
        self.set_flag(Flags6502::N, temp & 0x80 == 0x80); // Negative

        if self.ins[self.opcode as usize].addr_mode == Cpu6502::imp
        {
            self.a = (temp & 0x00FF) as u8;
        }
        else
        {
            self.cpu_write(self.addr_abs, (temp & 0x00FF) as u8);
        }

        0
    }

    fn branch(&mut self)
    {
        self.cycles += 1;
        let overflow_addr: u32 = self.pc as u32 + self.addr_rel as u32;
        self.addr_abs = overflow_addr as u16;

        if (self.addr_abs & 0xFF00) != (self.pc & 0xFF00)
        {
            self.cycles += 1;
        }

        self.pc = self.addr_abs;
    }

    // Instruction: Branch if Carry Clear
    // if (C == 0) then pc = address
    pub fn bcc(&mut self) -> u8
    {
        if self.get_flag(Flags6502::C) == 0
        {
           self.branch();
        }
        0
    }

    // Instruction: Branch if Carry Set
    // if (C == 1) then pc = address
    pub fn bcs(&mut self) -> u8
    {
        if self.get_flag(Flags6502::C) == 1
        {
            self.branch();
        }
        0
    }

    // Instruction: Branch if Equal
    // if (Z == 1) then pc = address
    pub fn beq(&mut self) -> u8
    {
        if self.get_flag(Flags6502::Z) == 1
        {
            self.branch();
        }
        0
    }

    // Instruction: Bit Test
    // Test if A & M, N = M7, V = M6, Z if result is zero
    pub fn bit(&mut self) -> u8
    {
        self.fetch();
        let temp: u16 = (self.a as u16) & (self.fetched_data as u16);
        self.set_flag(Flags6502::Z, (temp & 0x00FF) == 0); // Zero
        self.set_flag(Flags6502::N, self.fetched_data & (1 << 7) == (1 << 7)); // Negative
        self.set_flag(Flags6502::V, self.fetched_data & (1 << 6) == (1 << 6)); // Overflow

        0
    }

    // Instruction: Branch if Negative
    // if (N == 1) then pc = address
    pub fn bmi(&mut self) -> u8
    {
        if self.get_flag(Flags6502::N) == 1
        {
            self.branch();
        }
        0
    }

    // Instruction: Branch if not equal
    // if (Z == 0) then pc = address
    pub fn bne(&mut self) -> u8
    {
        if self.get_flag(Flags6502::Z) == 0
        {
            self.branch();
        }
        0
    }

    // Instruction: Branch if Positive
    // if (N == 0) then pc = address
    pub fn bpl(&mut self) -> u8
    {
        if self.get_flag(Flags6502::N) == 0
        {
            self.branch();
        }
        0
    }

    fn write_pc_to_stack(&mut self)
    {
        self.cpu_write(Cpu6502::STACK_START_ADDRESS + self.stkp as u16, ((self.pc >> 8) & 0x00FF) as u8);
        self.stkp -= 1;
        self.cpu_write(Cpu6502::STACK_START_ADDRESS + self.stkp as u16, (self.pc & 0x00FF) as u8);
        self.stkp -= 1;
    }

    fn read_pc_from_stack(&mut self)
    {
        self.stkp += 1;
        let mut lo: u8 = 0;
        self.cpu_read(Cpu6502::STACK_START_ADDRESS + self.stkp as u16, &mut lo);
        self.pc = lo as u16;

        self.stkp += 1;
        let mut hi: u8 = 0;
        self.cpu_read(Cpu6502::STACK_START_ADDRESS + self.stkp as u16, &mut hi);

        self.pc |= (hi as u16) << 8;
    }

    // Instruction: Break
    // Function: Program Sourced Interrupt
    pub fn brk(&mut self) -> u8
    {
        self.pc += 1;

        self.set_flag(Flags6502::I, true);
        self.write_pc_to_stack();

        // TODO: Not clear when exactly the B-flag should be set based on the documentation, needs investigation
        self.cpu_write(Cpu6502::STACK_START_ADDRESS + self.stkp as u16, self.status | Flags6502::B as u8 | Flags6502::U as u8);
        self.stkp -= 1;

        // TODO: Not clear if we should vary behavior if an NMI happens while BRK is ongoing
        // Go to the interrupt vector
        let mut lo: u8 = 0;
        let mut hi: u8 = 0;
        self.cpu_read(Cpu6502::INTERRUPT_VECTOR[1], &mut lo);
        self.cpu_read(Cpu6502::INTERRUPT_VECTOR[0], &mut hi);
        self.pc = lo as u16 |
                    ((hi as u16) << 8);

        0
    }

    // Instruction: Branch if Overflow Clear
    // Function: if (V == 0) pc = address
    pub fn bvc(&mut self) -> u8
    {
        if self.get_flag(Flags6502::V) == 0
        {
            self.branch();
        }
        0
    }

    // Instruction: Branch if Overflow Set
    // Function: if (V == 1) pc = address
    pub fn bvs(&mut self) -> u8
    {
        if self.get_flag(Flags6502::V) == 1
        {
            self.branch();
        }
        0
    }

    // Instruction: Clear Carry Flag
    // Function: C = 0
    pub fn clc(&mut self) -> u8 
    {
        self.set_flag(Flags6502::C, false);
        0
    }

    // Instruction: Clear Decimal Flag
    // Function: D = 0
    pub fn cld(&mut self) -> u8 
    {
        self.set_flag(Flags6502::D, false);
        0
    }

    // Instruction: Disable Interrupts / Clear Interrupt Flag
    // Function: I = 0
    pub fn cli(&mut self) -> u8 
    {
        self.set_flag(Flags6502::I, false);
        0
    }

    // Instruction: Clear Overflow Flag
    // Function: V = 0
    pub fn clv(&mut self) -> u8 
    {
        self.set_flag(Flags6502::V, false);
        0
    }

    fn cmp_helper(&mut self, var: u16)
    {
        self.fetch();
        let temp: u16 = var.wrapping_sub(self.fetched_data as u16);
        self.set_flag(Flags6502::C, var >= (self.fetched_data as u16));
        self.set_flag(Flags6502::Z, (temp & 0x00FF) == 0x0000);
        self.set_flag(Flags6502::N, temp & 0x0080 == 0x0080);
    }

    pub fn cmp(&mut self) -> u8 
    {
        let temp_a = self.a as u16;
        self.cmp_helper(temp_a);
        1
    }

    pub fn cpx(&mut self) -> u8 
    {
        let temp_x = self.x as u16;
        self.cmp_helper(temp_x);
        0
    }

    pub fn cpy(&mut self) -> u8 
    {
        let temp_y = self.y as u16;
        self.cmp_helper(temp_y);
        0
    }

    pub fn dec(&mut self) -> u8 
    {
        self.fetch();
        let temp: u16 = (self.fetched_data as u16).wrapping_sub(1);
        self.cpu_write(self.addr_abs, (temp & 0x00FF) as u8);
        self.set_flag(Flags6502::Z, (temp & 0x00FF) == 0x0000);
        self.set_flag(Flags6502::N, (temp & 0x0080) == 0x0080);

        0
    }

    // Instruction: Decrement X Register
    // Function: X = X - 1
    pub fn dex(&mut self) -> u8 
    {
        self.x = self.x.wrapping_sub(1);
        self.set_flag(Flags6502::Z, self.x == 0x00);
        self.set_flag(Flags6502::N, (self.x & 0x80) == 0x80);
        0
    }

    // Instruction: Decrement Y Register
    // Function: Y = Y - 1
    pub fn dey(&mut self) -> u8 
    {
        self.y = self.y.wrapping_sub(1);
        self.set_flag(Flags6502::Z, self.y == 0x00);
        self.set_flag(Flags6502::N, (self.y & 0x80) == 0x80);
        0
    }

    // Instruction: Bitwise Logic XOR
    // Function: A = A xor M
    pub fn eor(&mut self) -> u8 
    {
        self.fetch();
        self.a ^= self.fetched_data;
        self.set_flag(Flags6502::Z, self.a == 0x00);
        self.set_flag(Flags6502::N, (self.a & 0x80) == 0x80);
        0
    }

    // Instruction: Increment Value at Memory Location
    // Function: M = M + 1
    pub fn inc(&mut self) -> u8 
    {
        self.fetch();
        let temp: u16 = (self.fetched_data as u16).wrapping_add(1);
        self.cpu_write(self.addr_abs, (temp & 0x00FF) as u8);
        self.set_flag(Flags6502::Z, (temp & 0x00FF) == 0x0000);
        self.set_flag(Flags6502::N, (temp & 0x0080) == 0x0080);
        0
    }

    // Instruction: Increment X Register
    // Function: X = X + 1
    pub fn inx(&mut self) -> u8 
    {
        self.x = self.x.wrapping_add(1);
        self.set_flag(Flags6502::Z, self.x == 0x00);
        self.set_flag(Flags6502::N, (self.x & 0x80) == 0x80);
        0
    }

    // Instruction: Increment Y Register
    // Function: Y = Y + 1
    pub fn iny(&mut self) -> u8 
    {
        self.y = self.y.wrapping_add(1);
        self.set_flag(Flags6502::Z, self.y == 0x00);
        self.set_flag(Flags6502::N, (self.y & 0x80) == 0x80);
        0
    }

    // Instruction: Jump To Location
    // Function: pc = address
    pub fn jmp(&mut self) -> u8 
    {
        self.pc = self.addr_abs;
        0
    }

    // Instruction: Jump To Sub-Routine
    // Function:    Push current pc to stack, pc = address
    pub fn jsr(&mut self) -> u8 
    {
        // Decrement PC to get back to current program counter
        self.pc -= 1;
        self.write_pc_to_stack();
        self.pc = self.addr_abs;
        0
    }

    fn load_helper_write<F>(&mut self, write_closure: F) where F: Fn(&mut Cpu6502)
    {
        self.fetch();
        write_closure(self);
    }

    fn load_helper_update_flags<F>(&mut self, read_closure: F) where F: Fn(&Cpu6502) -> u8
    {
        self.set_flag(Flags6502::Z, read_closure(self) == 0x00);
        self.set_flag(Flags6502::N, (read_closure(self) & 0x80) == 0x80);
    }

    pub fn lda(&mut self) -> u8 
    {
        self.load_helper_write(|x| x.a = x.fetched_data);
        self.load_helper_update_flags(|x| x.a);
        1
    }

    pub fn ldx(&mut self) -> u8 
    {
        self.load_helper_write(|x| x.x = x.fetched_data);
        self.load_helper_update_flags(|x| x.x);
        1
    }

    pub fn ldy(&mut self) -> u8 
    {
        self.load_helper_write(|x| x.y = x.fetched_data);
        self.load_helper_update_flags(|x| x.y);
        1
    }

    pub fn lsr(&mut self) -> u8 
    {
        self.fetch();
        self.set_flag(Flags6502::C, (self.fetched_data & 0x0001) == 0x0001); // Carry
        let temp: u16 = (self.fetched_data as u16) >> 1;
        
        self.set_flag(Flags6502::Z, (temp & 0x00FF) == 0x0000); // Zero
        self.set_flag(Flags6502::N, temp & 0x0080 == 0x0080); // Negative

        if self.ins[self.opcode as usize].addr_mode == Cpu6502::imp
        {
            self.a = (temp & 0x00FF) as u8;
        }
        else
        {
            self.cpu_write(self.addr_abs, (temp & 0x00FF) as u8);
        }

        0
    }

    // Instruction: No-op
    pub fn nop(&mut self) -> u8 
    {
        // There are different kinds of NOPs depending on the opcode
        match self.opcode
        {
            0x1C => 1,
            0x3C => 1,
            0x5C => 1,
            0x7C => 1,
            0xDC => 1,
            0xFC => 1,
            _ => 0
        }
    }

    // Instruction: Bitwise Logic OR
    // Function: A = A | M
    pub fn ora(&mut self) -> u8 
    {
        self.fetch();
        self.a |= self.fetched_data;

        self.set_flag(Flags6502::Z, self.a == 0x00); // Zero
        self.set_flag(Flags6502::N, self.a & 0x80 == 0x80); // Negative
        0
    }

    // Instruction: Push Accumulator to Stack
    // Function: Write A to stkp
    pub fn pha(&mut self) -> u8 
    {
        self.cpu_write(Cpu6502::STACK_START_ADDRESS + self.stkp as u16, self.a);
        self.stkp -= 1;
        0
    }
    
    // Instruction: Push Status Register to Stack
    // Function: status -> stack
    pub fn php(&mut self) -> u8 
    {
        // Note that Unused flag are both set to 1 when writing
        self.cpu_write(Cpu6502::STACK_START_ADDRESS + self.stkp as u16, (self.status) | Flags6502::B as u8 | Flags6502::U as u8);
        // self.set_flag(Flags6502::B, false);
        // self.set_flag(Flags6502::U, false);
        self.stkp -= 1;
        0
    }

    // Instruction: Pop Accumulator off Stack
    // Function: Set A to stack
    pub fn pla(&mut self) -> u8 
    {
        self.stkp += 1;

        let mut result: u8 = 0;
        self.cpu_read(Cpu6502::STACK_START_ADDRESS + self.stkp as u16, &mut result);
        self.a = result;

        self.set_flag(Flags6502::Z, self.a == 0x00); // Zero
        self.set_flag(Flags6502::N, self.a & 0x80 == 0x80); // Negative
        0
    }

    // Instruction: Pop Status Register off Stack
    // Function: Status <- stack
    pub fn plp(&mut self) -> u8 
    {
        self.stkp += 1;

        let mut result: u8 = 0;
        self.cpu_read(Cpu6502::STACK_START_ADDRESS + self.stkp as u16, &mut result);
        self.status = result;
        
        self.set_flag(Flags6502::U, true);
        self.set_flag(Flags6502::B, false);
        0
    }

    // Instruction: Rotate Left
    // Function: Shift fetched data left by one, and 0-bit of result is set to C
    pub fn rol(&mut self) -> u8 
    {
        self.fetch();
        let temp: u16 = ((self.fetched_data as u16) << 1) | self.get_flag(Flags6502::C) as u16;
        self.set_flag(Flags6502::C, temp & 0xFF00 > 0); // Carry
        self.set_flag(Flags6502::Z, (temp & 0x00FF) == 0x0000); // Zero
        self.set_flag(Flags6502::N, temp & 0x0080 == 0x0080); // Negative

        // TODO: Consolidate this pattern
        if self.ins[self.opcode as usize].addr_mode == Cpu6502::imp
        {
            self.a = (temp & 0x00FF) as u8;
        }
        else
        {
            self.cpu_write(self.addr_abs, (temp & 0x00FF) as u8);
        }

        0
    }

    // Instruction: Rotate Right
    // Function: Shift fetched data right by one, and 7-bit of result is set to C
    pub fn ror(&mut self) -> u8 
    {
        self.fetch();
        let temp: u16 = (self.fetched_data as u16) >> 1 | ((self.get_flag(Flags6502::C) as u16) << 7);
        self.set_flag(Flags6502::C, (self.fetched_data & 0x01) == 0x01);
        self.set_flag(Flags6502::Z, (temp & 0x00FF) == 0x0000); // Zero
        self.set_flag(Flags6502::N, (temp & 0x0080) == 0x0080); // Negative

        // TODO: Consolidate this pattern
        if self.ins[self.opcode as usize].addr_mode == Cpu6502::imp
        {
            self.a = (temp & 0x00FF) as u8;
        }
        else
        {
            self.cpu_write(self.addr_abs, (temp & 0x00FF) as u8);
        }

        0
    }

    // Instruction: Return from Interrupt
    // Function: Read status and pc from stkp
    pub fn rti(&mut self) -> u8 
    {
        self.stkp += 1;

        let mut result = 0;
        self.cpu_read(Cpu6502::STACK_START_ADDRESS + self.stkp as u16, &mut result);
        self.status = result;

        // Zero out break and unused values, TODO: Why?
        self.status &= !(Flags6502::B as u8);
        self.status &= !(Flags6502::U as u8);

        self.read_pc_from_stack();
        0
    }

    // Instruction: Return from subroutine
    // Function: Read pc from stack, increment PC to next instruction
    pub fn rts(&mut self) -> u8 
    {
        self.read_pc_from_stack();
        self.pc += 1;
        0
    }

    // Explanation borrowed from https://github.com/OneLoneCoder/olcNES/blob/master/Part%232%20-%20CPU/olc6502.cpp

    // Instruction: Subtraction with Borrow In
    // Function:    A = A - M - (1 - C)
    // Flags Out:   C, V, N, Z
    //
    // Explanation:
    // Given the explanation for ADC above, we can reorganise our data
    // to use the same computation for addition, for subtraction by multiplying
    // the data by -1, i.e. make it negative
    //
    // A = A - M - (1 - C)  ->  A = A + -1 * (M - (1 - C))  ->  A = A + (-M + 1 + C)
    //
    // To make a signed positive number negative, we can invert the bits and add 1
    // (OK, I lied, a little bit of 1 and 2s complement :P)
    //
    //  5 = 00000101
    // -5 = 11111010 + 00000001 = 11111011 (or 251 in our 0 to 255 range)
    //
    // The range is actually unimportant, because if I take the value 15, and add 251
    // to it, given we wrap around at 256, the result is 10, so it has effectively 
    // subtracted 5, which was the original intention. (15 + 251) % 256 = 10
    //
    // Note that the equation above used (1-C), but this got converted to + 1 + C.
    // This means we already have the +1, so all we need to do is invert the bits
    // of M, the data(!) therfore we can simply add, exactly the same way we did 
    // before.
    pub fn sbc(&mut self) -> u8 
    {
        self.fetch();

        // This is an inversion of the bits. Once we invert, the logic just becomes adc()
        let value: u16 = (self.fetched_data as u16) ^ 0x00FF;

        // This logic is taken from adc()   
        let temp: u16 = self.a as u16 + value + self.get_flag(Flags6502::C) as u16;

        // If we overflow into 16-bit range, set the carry bit
        self.set_flag(Flags6502::C, temp > 255);
        self.set_flag(Flags6502::Z, (temp & 0x00FF) == 0);

        let v: u16 = (temp ^ self.a as u16) & (temp ^ value) & 0x0080;
        self.set_flag(Flags6502::V, v != 0);
        self.set_flag(Flags6502::N, temp & 0x80 == 0x80);

        // Load the result into the accumulator
        self.a = (temp & 0x00FF) as u8;

        1
    }

    // Instruction: Set Carry Flag
    // Function: C = 1
    pub fn sec(&mut self) -> u8 
    {
        self.set_flag(Flags6502::C, true);
        0
    }

    // Instruction: Set Decimal Flag
    // Function: D = 1
    pub fn sed(&mut self) -> u8 
    {
        self.set_flag(Flags6502::D, true);
        0
    }

    // Instruction: Set Interrupt Flag / Enable Interrupts
    // Function: I = 1
    pub fn sei(&mut self) -> u8 
    {
        self.set_flag(Flags6502::I, true);
        0
    }

    // Instruction: Store Accumulator at address
    // Function: M = A
    pub fn sta(&mut self) -> u8 
    {
        self.cpu_write(self.addr_abs, self.a);
        0
    }

    // Instruction: Store X at address
    // Function: M = X
    pub fn stx(&mut self) -> u8 
    {
        self.cpu_write(self.addr_abs, self.x);
        0
    }

    // Instruction: Store Y at address
    // Function: M = Y
    pub fn sty(&mut self) -> u8 
    {
        self.cpu_write(self.addr_abs, self.y);
        0
    }

    // Instruction: Transfer Accumulator to X Register
    // Function: X = A
    pub fn tax(&mut self) -> u8 
    {
        self.x = self.a;
        self.set_flag(Flags6502::Z, self.x == 0x00); // Zero
        self.set_flag(Flags6502::N, self.x & 0x80 == 0x80); // Negative
        0
    }

    // Instruction: Transfer Accumulator to Y Register
    // Function: Y = A
    pub fn tay(&mut self) -> u8 
    {
        self.y = self.a;
        self.set_flag(Flags6502::Z, self.y == 0x00); // Zero
        self.set_flag(Flags6502::N, self.y & 0x80 == 0x80); // Negative
        0
    }

    // Instruction: Transfer Stack Pointer to X
    // Function:    X = stkp
    pub fn tsx(&mut self) -> u8 
    {
        self.x = self.stkp;
        self.set_flag(Flags6502::Z, self.x == 0x00); // Zero
        self.set_flag(Flags6502::N, self.x & 0x80 == 0x80); // Negative
        0
    }

    // Instruction: Transfer X to A
    // Function: A = X
    pub fn txa(&mut self) -> u8 
    {
        self.a = self.x;
        self.set_flag(Flags6502::Z, self.a == 0x00); // Zero
        self.set_flag(Flags6502::N, self.a & 0x80 == 0x80); // Negative
        0
    }

    // Instruction: Transfer X to Stack pointer
    // Function: stkp = x
    pub fn txs(&mut self) -> u8 
    {
        self.stkp = self.x;
        0
    }

    // Instruction: Transfer Y to A
    // Function: A = Y
    pub fn tya(&mut self) -> u8 
    {
        self.a = self.y;
        self.set_flag(Flags6502::Z, self.a == 0x00); // Zero
        self.set_flag(Flags6502::N, self.a & 0x80 == 0x80); // Negative
        0
    }

    // The following are unofficial opcodes but are tested by nestest.rom
    pub fn lax(&mut self) -> u8
    {
        self.fetch();
        self.a = self.fetched_data;
        self.x = self.fetched_data;
        self.set_flag(Flags6502::Z, self.a == 0x00);
        self.set_flag(Flags6502::N, (self.a & 0x80) == 0x80);
        1
    }

    pub fn sax(&mut self) -> u8
    {
        self.fetch();
        let a_and_x = self.a & self.x;
        self.cpu_write(self.addr_abs, a_and_x);
        // self.x = and_a_x.wrapping_sub(self.fetched_data);

        //self.set_flag(Flags6502::C, self.x >= self.fetched_data);
        //self.set_flag(Flags6502::Z, self.x == 0x00);
        //self.set_flag(Flags6502::N, (self.x & 0x80) == 0x80);
        0
    }

    pub fn dcp(&mut self) -> u8
    {
        self.fetch();
        let result: u16 = (self.fetched_data as u16).wrapping_sub(1);
        let result_u8 = (result & 0x00FF) as u8;
        self.cpu_write(self.addr_abs, result_u8);

        let temp: u16 = (self.a as u16).wrapping_sub(result_u8 as u16);
        self.set_flag(Flags6502::C, self.a >= result_u8);
        self.set_flag(Flags6502::Z, (temp & 0x00FF) == 0x0000);
        self.set_flag(Flags6502::N, temp & 0x0080 == 0x0080);
        0
    }

    // Also called ins or isc
    pub fn isb(&mut self) -> u8
    {
        self.fetch();
        let mut temp: u16 = (self.fetched_data as u16).wrapping_add(1);
        self.cpu_write(self.addr_abs, (temp & 0x00FF) as u8);
        
        // This is an inversion of the bits. Once we invert, the logic just becomes adc()
        let value: u16 = temp ^ 0x00FF;

        // This logic is taken from adc()   
        temp = self.a as u16 + value + self.get_flag(Flags6502::C) as u16;

        // If we overflow into 16-bit range, set the carry bit
        self.set_flag(Flags6502::C, temp > 255);
        self.set_flag(Flags6502::Z, (temp & 0x00FF) == 0);

        let v: u16 = (temp ^ self.a as u16) & (temp ^ value) & 0x0080;
        self.set_flag(Flags6502::V, v != 0);
        self.set_flag(Flags6502::N, temp & 0x80 == 0x80);

        // Load the result into the accumulator
        self.a = (temp & 0x00FF) as u8;

        0
    }

    pub fn slo(&mut self) -> u8
    {
        self.fetch();
        let temp: u16 = (self.fetched_data as u16) << 1;
        self.set_flag(Flags6502::C, (temp & 0xFF00) > 0); // Carry

        self.a |= temp as u8;
        self.set_flag(Flags6502::Z, self.a == 0x00); // Zero
        self.set_flag(Flags6502::N, self.a & 0x80 == 0x80); // Negative
        self.cpu_write(self.addr_abs, (temp & 0x00FF) as u8);

        0
    }

    pub fn rla(&mut self) -> u8
    {
        self.fetch();
        let temp: u16 = ((self.fetched_data as u16) << 1) | self.get_flag(Flags6502::C) as u16;
        self.set_flag(Flags6502::C, temp & 0xFF00 > 0); // Carry

        self.a &= temp as u8;
        self.set_flag(Flags6502::Z, self.a == 0x00); // Zero
        self.set_flag(Flags6502::N, self.a & 0x80 == 0x80); // Negative
        self.cpu_write(self.addr_abs, (temp & 0x00FF) as u8);
        0
    }

    pub fn anc(&mut self) -> u8
    {
        self.fetch();
        self.a &= self.fetched_data;
        self.set_flag(Flags6502::C, self.a & 0x80 == 0x80); // Carry
        self.set_flag(Flags6502::Z, self.a == 0x00); // Zero
        self.set_flag(Flags6502::N, self.a & 0x80 == 0x80); // Negative
        0
    }

    // Also known as LSE
    // LSRs the contents of a memory location and then EORs the result with the accumulator.
    pub fn sre(&mut self) -> u8
    {
        self.fetch();
        self.set_flag(Flags6502::C, (self.fetched_data & 0x0001) == 0x0001); // Carry
        let temp: u16 = (self.fetched_data as u16) >> 1;
        self.a ^= (temp & 0x00FF) as u8;
        self.set_flag(Flags6502::Z, self.a == 0x00);
        self.set_flag(Flags6502::N, (self.a & 0x80) == 0x80);
        self.cpu_write(self.addr_abs, (temp & 0x00FF) as u8);
        0
    }

    pub fn rra(&mut self) -> u8
    {
        self.fetch();
        let temp: u16 = (self.fetched_data as u16) >> 1 | ((self.get_flag(Flags6502::C) as u16) << 7);
        self.set_flag(Flags6502::C, (self.fetched_data & 0x01) == 0x01);

        let result = self.a as u16 + temp + self.get_flag(Flags6502::C) as u16;

        // If we overflow into 16-bit range, set the carry bit
        self.set_flag(Flags6502::C, result > 255);
        self.set_flag(Flags6502::Z, (result & 0x00FF) == 0);

        let v: u16 = (!(self.a as u16 ^ temp)) & ((self.a as u16 ^ result) & 0x0080);
        self.set_flag(Flags6502::V, v != 0);
        self.set_flag(Flags6502::N, result & 0x80 == 0x80);

        // Load the result into the accumulator
        self.a = (result & 0x00FF) as u8;
        self.cpu_write(self.addr_abs, (temp & 0x00FF) as u8);

        0
    }

    pub fn kil(&mut self) -> u8 
    {
        // Running this instruction kills the device
        panic!("Executed KIL instruction");
    }

    pub fn arr(&mut self) -> u8
    {
        panic!("Executed ARR instruction");
    }

    pub fn xaa(&mut self) -> u8
    {
        panic!("Executed XAA instruction");
    }

    pub fn ahx(&mut self) -> u8
    {
        panic!("Executed AHX instruction");
    }

    pub fn tas(&mut self) -> u8
    {
        panic!("Executed TAS instruction");
    }

    pub fn shx(&mut self) -> u8
    {
        panic!("Executed SHX instruction");
    }

    pub fn las(&mut self) -> u8
    {
        panic!("Executed LAS instruction");
    }

    pub fn axs(&mut self) -> u8
    {
        panic!("Executed AXS instruction");
    }

    pub fn alr(&mut self) -> u8
    {
        panic!("Executed ALR instruction");
    }

    pub fn new() -> Self
    {
        Cpu6502
        {
            bus: None,
            cartridge: None,
            a: 0x00,
            x: 0x00,
            y: 0x00,
            stkp: 0x00,
            pc: 0x0000,
            status: 0x00,
            fetched_data: 0x00,
            addr_abs: 0x0000,
            addr_rel: 0x0000,
            opcode: 0x00,
            cycles: 0x00,
            total_cycles: -1,
            ins: create_instruction_array!()
        }
    }

    pub fn complete(&self) -> bool
    {
        self.cycles == 0
    }

    pub fn disassemble(&mut self, n_start: u16, n_end: u16, include_state: bool) -> BTreeMap<u16, String>
    {
        let mut map = BTreeMap::new();

        let mut addr: u32 = n_start as u32;
        let mut value: u8 = 0;
        let mut lo: u8 = 0;
        let mut hi: u8 = 0;
        let mut line_addr: u16;

        while addr <= n_end as u32
        {
            line_addr = addr as u16;
            let mut instruction: String = format!("{:04X}  ", addr);
            let mut opcode: u8 = 0;
            match &self.bus
            {
                Some(x) =>
                {
                    x.lock().unwrap().cpu_read(addr as u16, &mut opcode)
                },
                None => panic!("Error, missing bus inside CPU")
            };

            instruction += &format!("{:02X} ", opcode);
            let name = self.ins[opcode as usize].name.clone();

            let pad = |instruction_ref: &mut String, name_ref: &String| 
            {
                let mut sub_1: i32 = 0;
                if (*name_ref).starts_with('*')
                {
                    sub_1 = -1;
                }

                let num_spaces_closure;

                if instruction_ref.len() == 9
                {
                    num_spaces_closure = 7 + sub_1;
                }
                else if instruction_ref.len() == 11
                {
                    num_spaces_closure = 5 + sub_1;
                }
                else
                {
                    num_spaces_closure = 2 + sub_1;
                }

                let repeated_blanks_closure = ' '.to_string().repeat(num_spaces_closure as usize);
                *instruction_ref += &repeated_blanks_closure;
                *instruction_ref += name_ref;
            };


            addr += 1;

            if self.ins[opcode as usize].addr_mode == Cpu6502::imp
            {
                pad(&mut instruction, &name);
            }
            else if self.ins[opcode as usize].addr_mode == Cpu6502::imm
            {
                self.cpu_read(addr as u16, &mut value);
                instruction += &format!("{:02X}", value);
                addr += 1;
                pad(&mut instruction, &name);
                instruction += &format!(" #${:02X}", value);
            }
            else if self.ins[opcode as usize].addr_mode == Cpu6502::zp0
            {
                self.cpu_read(addr as u16, &mut lo);
                instruction += &format!("{:02X}", lo);
                addr += 1;
                pad(&mut instruction, &name);

                let mut temp = 0;
                self.cpu_read((lo as u16) & 0x00FF, &mut temp);
                instruction += &format!(" ${:02X} = {:02X}", lo, temp);
            }
            else if self.ins[opcode as usize].addr_mode == Cpu6502::zpx
            {
                self.cpu_read(addr as u16, &mut lo);
                instruction += &format!("{:02X}", lo);
                addr += 1;
                pad(&mut instruction, &name);

                let ind_addr = (lo as u16 + self.x as u16) & 0x00FF;
                let mut data = 0;
                self.cpu_read(ind_addr, &mut data);

                instruction += &format!(" ${:02X},X @ {:02X} = {:02X}", lo, ind_addr, data);
            }
            else if self.ins[opcode as usize].addr_mode == Cpu6502::zpy
            {
                self.cpu_read(addr as u16, &mut lo);
                instruction += &format!("{:02X}", lo);
                addr += 1;
                pad(&mut instruction, &name);
                
                let ind_addr = (lo as u16 + self.y as u16) & 0x00FF;
                let mut data = 0;
                self.cpu_read(ind_addr, &mut data);

                instruction += &format!(" ${:02X},Y @ {:02X} = {:02X}", lo, ind_addr, data);
            }
            else if self.ins[opcode as usize].addr_mode == Cpu6502::izx
            {
                self.cpu_read(addr as u16, &mut lo);
                instruction += &format!("{:02X}", lo);
                addr += 1;
                pad(&mut instruction, &name);

                let ind: u16 = lo as u16;

                self.cpu_read((ind + self.x as u16) & 0x00FF, &mut lo);
                self.cpu_read((ind + self.x as u16 + 1) & 0x00FF, &mut hi);

                let ind_addr = ((hi as u16) << 8) | lo as u16;
                let mut ind_data: u8 = 0;
                self.cpu_read(ind_addr, &mut ind_data);

                instruction += &format!(" (${:02X},X) @ {:02X} = {:04X} = {:02X}", ind, ind + self.x as u16, ind_addr, ind_data);
            }
            else if self.ins[opcode as usize].addr_mode == Cpu6502::izy
            {
                self.cpu_read(addr as u16, &mut lo);
                instruction += &format!("{:02X}", lo);
                addr += 1;
                pad(&mut instruction, &name);

                let ind: u16 = lo as u16;

                self.cpu_read((ind) & 0x00FF, &mut lo);
                self.cpu_read((ind + 1) & 0x00FF, &mut hi);

                let ind_addr = ((hi as u16) << 8) | lo as u16;
                let ind_addr_y = ind_addr.wrapping_add(self.y as u16);

                let mut ind_data: u8 = 0;
                self.cpu_read(ind_addr_y, &mut ind_data);
                instruction += &format!(" (${:02X}),Y = {:04X} @ {:04X} = {:02X}", ind, ind_addr, ind_addr_y, ind_data);
            }
            else if self.ins[opcode as usize].addr_mode == Cpu6502::abs
            {
                self.cpu_read(addr as u16, &mut lo);
                instruction += &format!("{:02X} ", lo);
                addr += 1;
                self.cpu_read(addr as u16, &mut hi);
                instruction += &format!("{:02X}", hi);
                addr += 1;
                let cur_addr = ((hi as u16) << 8) | lo as u16;
                pad(&mut instruction, &name);

                if self.ins[opcode as usize].name.eq("JMP") ||
                    self.ins[opcode as usize].name.eq("JSR")
                    
                {
                    instruction += &format!(" ${:04X}", cur_addr);
                }
                else
                {
                    let mut temp = 0;
                    self.cpu_read(cur_addr, &mut temp);
                    instruction += &format!(" ${:04X} = {:02X}", cur_addr, temp);
                }
            }
            else if self.ins[opcode as usize].addr_mode == Cpu6502::abx
            {
                self.cpu_read(addr as u16, &mut lo);
                instruction += &format!("{:02X} ", lo);
                addr += 1;
                self.cpu_read(addr as u16, &mut hi);
                instruction += &format!("{:02X}", hi);
                addr += 1;
                let cur_addr = ((hi as u16) << 8) | lo as u16;
                pad(&mut instruction, &name);

                let ind_addr = cur_addr.wrapping_add(self.x as u16);
                let mut data = 0;
                self.cpu_read(ind_addr, &mut data);

                instruction += &format!(" ${:04X},X @ {:04X} = {:02X}", cur_addr, ind_addr, data);
            }
            else if self.ins[opcode as usize].addr_mode == Cpu6502::aby
            {
                self.cpu_read(addr as u16, &mut lo);
                instruction += &format!("{:02X} ", lo);
                addr += 1;
                self.cpu_read(addr as u16, &mut hi);
                instruction += &format!("{:02X}", hi);
                addr += 1;
                let cur_addr = ((hi as u16) << 8) | lo as u16;
                pad(&mut instruction, &name);

                let ind_addr = cur_addr.wrapping_add(self.y as u16);
                let mut data = 0;
                self.cpu_read(ind_addr, &mut data);

                instruction += &format!(" ${:04X},Y @ {:04X} = {:02X}", cur_addr, ind_addr, data);
            }
            else if self.ins[opcode as usize].addr_mode == Cpu6502::ind
            {
                self.cpu_read(addr as u16, &mut lo);
                instruction += &format!("{:02X} ", lo);
                addr += 1;
                self.cpu_read(addr as u16, &mut hi);
                instruction += &format!("{:02X}", hi);
                addr += 1;
                let ptr = ((hi as u16) << 8) | lo as u16;
                pad(&mut instruction, &name);
        
                let offset_addr =
                    if lo == 0x00FF
                    {
                        self.cpu_read(ptr, &mut lo);
                        self.cpu_read(ptr & 0xFF00, &mut hi);
            
                        ((hi as u16) << 8) | lo as u16
                    }
                    else
                    {
                        self.cpu_read(ptr, &mut lo);
                        self.cpu_read(ptr + 1, &mut hi);
            
                        ((hi as u16) << 8) | lo as u16
                    };

                instruction += &format!(" (${:04X}) = {:04X}", ptr, offset_addr);
            }
            else if self.ins[opcode as usize].addr_mode == Cpu6502::rel
            {
                self.cpu_read(addr as u16, &mut value);
                instruction += &format!("{:02X}", value);
                addr += 1;
                pad(&mut instruction, &name);
                instruction += &format!(" ${:04X}", addr as i32 + (value as i8) as i32);
            }

            if include_state
            {
                // Pad to 49 characters
                let num_spaces = 47 - instruction.len();
                let repeated_blanks = ' '.to_string().repeat(num_spaces);
                instruction += &repeated_blanks;
                instruction += &format!(" {:?}", self);
            }

            map.insert(line_addr, instruction);
        }

        map
    }

    fn perform_irq(&mut self, pc_read_addr: u16, num_cycles: u8)
    {
        // Save the PC to the stack
        self.write_pc_to_stack();

        // Save the status register to the stack
        self.set_flag(Flags6502::B, false);
        self.set_flag(Flags6502::U, true);
        self.set_flag(Flags6502::I, true);
        self.cpu_write(Cpu6502::STACK_START_ADDRESS + self.stkp as u16, self.status);
        self.stkp -= 1;
        
        // Read the new program counter location from a fixed address
        self.addr_abs = pc_read_addr;
        let mut lo: u8 = 0;
        self.cpu_read(self.addr_abs, &mut lo);
        let mut hi: u8 = 0;
        self.cpu_read(self.addr_abs + 1, &mut hi);

        self.pc = ((hi as u16) << 8) | (lo as u16);

        // Add cycles for IRQ to complete
        self.cycles = num_cycles;
    }

    pub fn irq(&mut self)
    {
        if self.get_flag(Flags6502::I) == 0
        {
            self.perform_irq(0xFFFE, 7);
        }
    }

    pub fn nmi(&mut self)
    {
        self.perform_irq(0xFFFA, 8);
    }

    // Helpers
    pub fn fetch(&mut self) -> u8
    {
        if self.ins[self.opcode as usize].addr_mode != Cpu6502::imp
        {
            let mut read_result: u8 = 0;
            self.cpu_read(self.addr_abs, &mut read_result);
            self.fetched_data = read_result;
        }

        self.fetched_data
    }

}

impl Default for Cpu6502 {
    fn default() -> Self {
        Self::new()
    }
}

impl Resettable for Cpu6502
{
    fn reset(&mut self)
    {
        self.addr_abs = 0xFFFC;
        let mut lo: u8 = 0;
        self.cpu_read(self.addr_abs, &mut lo);
        let mut hi: u8 = 0;
        self.cpu_read(self.addr_abs + 1, &mut hi);
    
        self.pc = ((hi as u16) << 8) | lo as u16;

        self.a = 0;
        self.x = 0;
        self.y = 0;
        self.stkp = 0xFD;

        self.status = Flags6502::U as u8 | Flags6502::I as u8;

        self.addr_rel = 0x0000;
        self.addr_abs = 0x0000;
        self.fetched_data = 0x00;

        self.cycles = 8;
        self.total_cycles = -1;
    }

}

impl fmt::Debug for Cpu6502
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        let mut cycle = 0;
        let mut scan_line = 0;
        match &self.bus
        {
            Some(x) =>
            {
                let mut bus = x.lock().unwrap();
                cycle = bus.get_ppu().lock().unwrap().get_cycle();
                scan_line = bus.get_ppu().lock().unwrap().get_scan_line();
            },
            None =>
            {
            }
        }

        // TODO: Why is total cycles off by 1
        // TODO: Why is cycle off by 5
        write!(f, "A:{:02X} X:{:02X} Y:{:02X} P:{:02X} SP:{:02X} PPU:{:3},{:3} CYC:{}", self.a, self.x, self.y, self.status, self.stkp, scan_line, cycle - 5, self.total_cycles)
    }
}

impl ReadWrite for Cpu6502
{
    fn cpu_write(&mut self, address: u16, data: u8) -> bool
    {
        match &self.bus
        {
            Some(x) => x.lock().unwrap().cpu_write(address, data),
            None => panic!("Error, missing bus inside CPU")
        }
    }

    fn cpu_read(&mut self, address: u16, data: &mut u8) -> bool
    {
        match &self.bus
        {
            Some(x) => x.lock().unwrap().cpu_read(address, data),
            None => panic!("Error, missing bus inside CPU")
        }
    }

    fn ppu_write(&mut self, _: u16, _: u8) -> bool
    {
        todo!()
    }

    fn ppu_read(&self, _: u16, _: &mut u8) -> bool
    {
        todo!()
    }
}

impl Clockable for Cpu6502
{
    fn clock_tick(&mut self) -> bool
    {
        if self.cycles == 0
        {
            let mut read_result: u8 = 0;
            self.cpu_read(self.pc, &mut read_result);
            self.opcode = read_result;

            // TODO: Why?
            self.set_flag(Flags6502::U, true);

            //let result = self.disassemble(self.pc, self.pc);

            self.cycles = self.ins[self.opcode as usize].cycles;
            // let last_pc = self.pc;
            self.pc += 1;

            let additional_cycle1: u8 = (self.ins[self.opcode as usize].addr_mode)(self);
            let additional_cycle2: u8 = (self.ins[self.opcode as usize].op)(self);
            //println!("{}", result.get(&last_pc).unwrap());

            // TODO: Why is this a binary AND?
            self.cycles += additional_cycle1 & additional_cycle2;

            // TODO: Why?
            self.set_flag(Flags6502::U, true);
        }

         self.cycles -= 1;
         self.total_cycles += 1;
         false
    }
}

unsafe impl Send for Cpu6502 {}