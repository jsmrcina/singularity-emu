use crate::traits::ReadWrite;

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

struct Instruction<'a>
{
    _name: String,
    op: fn(&mut CPU6502<'a>) -> u8,
    addr_mode: fn(&mut CPU6502<'a>) -> u8,
    cycles: u8
}

pub struct CPU6502<'a>
{
    bus: &'a mut dyn ReadWrite,
    a: u8,
    x: u8,
    y: u8,
    _stkp: u8,
    pc: u16,
    status: u8,
    fetched_data: u8,
    addr_abs: u16,
    addr_rel: u16,
    opcode: u8,
    cycles: u8,
    ins: [Instruction<'a>; 256]
}

impl<'a> CPU6502<'a>
{
    const _STACK_START_ADDRESS: u16 = 0x1000;

    pub fn new(bus: &'a mut dyn ReadWrite) -> Self
    {
        let cpu = CPU6502
        {
            bus: bus,
            a: 0x00,
            x: 0x00,
            y: 0x00,
            _stkp: 0x00,
            pc: 0x0000,
            status: 0x00,
            fetched_data: 0x00,
            addr_abs: 0x0000,
            addr_rel: 0x0000,
            opcode: 0x00,
            cycles: 0x00,
            ins: [
                // Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7},
                // Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7},
                // Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7},
                // Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7},
                // Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7},
                // Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7},
                // Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7},
                // Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7},
                // Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7},
                // Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7},
                // Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7},
                // Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7},
                // Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7},
                // Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7},
                // Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7},
                // Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7}, Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7},

                Instruction { _name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7 }, Instruction { _name: String::from("ORA"), op: CPU6502::ora, addr_mode: CPU6502::izx, cycles: 6 }, Instruction { _name: String::from("???"), op: CPU6502::xxx, addr_mode: CPU6502::imp, cycles: 2 }, Instruction { _name: String::from("???"), op: CPU6502::xxx, addr_mode: CPU6502::imp, cycles: 8 }, Instruction { _name: String::from("???"), op: CPU6502::nop, addr_mode: CPU6502::imp, cycles: 3 }, Instruction { _name: String::from("ORA"), op: CPU6502::ora, addr_mode: CPU6502::zp0, cycles: 3 }, Instruction { _name: String::from("ASL"), op: CPU6502::asl, addr_mode: CPU6502::zp0, cycles: 5 }, Instruction { _name: String::from("???"), op: CPU6502::xxx, addr_mode: CPU6502::imp, cycles: 5 }, Instruction { _name: String::from("PHP"), op: CPU6502::php, addr_mode: CPU6502::imp, cycles: 3 }, Instruction { _name: String::from("ORA"), op: CPU6502::ora, addr_mode: CPU6502::imm, cycles: 2 }, Instruction { _name: String::from("ASL"), op: CPU6502::asl, addr_mode: CPU6502::imp, cycles: 2 }, Instruction { _name: String::from("???"), op: CPU6502::xxx, addr_mode: CPU6502::imp, cycles: 2 }, Instruction { _name: String::from("???"), op: CPU6502::nop, addr_mode: CPU6502::imp, cycles: 4 }, Instruction { _name: String::from("ORA"), op: CPU6502::ora, addr_mode: CPU6502::abs, cycles: 4 }, Instruction { _name: String::from("ASL"), op: CPU6502::asl, addr_mode: CPU6502::abs, cycles: 6 }, Instruction { _name: String::from("???"), op: CPU6502::xxx, addr_mode: CPU6502::imp, cycles: 6 },
                Instruction { _name: String::from("BPL"), op: CPU6502::bpl, addr_mode: CPU6502::rel, cycles: 2 }, Instruction { _name: String::from("ORA"), op: CPU6502::ora, addr_mode: CPU6502::izy, cycles: 5 }, Instruction { _name: String::from("???"), op: CPU6502::xxx, addr_mode: CPU6502::imp, cycles: 2 }, Instruction { _name: String::from("???"), op: CPU6502::xxx, addr_mode: CPU6502::imp, cycles: 8 }, Instruction { _name: String::from("???"), op: CPU6502::nop, addr_mode: CPU6502::imp, cycles: 4 }, Instruction { _name: String::from("ORA"), op: CPU6502::ora, addr_mode: CPU6502::zpx, cycles: 4 }, Instruction { _name: String::from("ASL"), op: CPU6502::asl, addr_mode: CPU6502::zpx, cycles: 6 }, Instruction { _name: String::from("???"), op: CPU6502::xxx, addr_mode: CPU6502::imp, cycles: 6 }, Instruction { _name: String::from("CLC"), op: CPU6502::clc, addr_mode: CPU6502::imp, cycles: 2 }, Instruction { _name: String::from("ORA"), op: CPU6502::ora, addr_mode: CPU6502::aby, cycles: 4 }, Instruction { _name: String::from("???"), op: CPU6502::nop, addr_mode: CPU6502::imp, cycles: 2 }, Instruction { _name: String::from("???"), op: CPU6502::xxx, addr_mode: CPU6502::imp, cycles: 7 }, Instruction { _name: String::from("???"), op: CPU6502::nop, addr_mode: CPU6502::imp, cycles: 4 }, Instruction { _name: String::from("ORA"), op: CPU6502::ora, addr_mode: CPU6502::abx, cycles: 4 }, Instruction { _name: String::from("ASL"), op: CPU6502::asl, addr_mode: CPU6502::abx, cycles: 7 }, Instruction { _name: String::from("???"), op: CPU6502::xxx, addr_mode: CPU6502::imp, cycles: 7 },
                Instruction { _name: String::from("JSR"), op: CPU6502::jsr, addr_mode: CPU6502::abs, cycles: 6 }, Instruction { _name: String::from("AND"), op: CPU6502::and, addr_mode: CPU6502::izx, cycles: 6 }, Instruction { _name: String::from("???"), op: CPU6502::xxx, addr_mode: CPU6502::imp, cycles: 2 }, Instruction { _name: String::from("???"), op: CPU6502::xxx, addr_mode: CPU6502::imp, cycles: 8 }, Instruction { _name: String::from("BIT"), op: CPU6502::bit, addr_mode: CPU6502::zp0, cycles: 3 }, Instruction { _name: String::from("AND"), op: CPU6502::and, addr_mode: CPU6502::zp0, cycles: 3 }, Instruction { _name: String::from("ROL"), op: CPU6502::rol, addr_mode: CPU6502::zp0, cycles: 5 }, Instruction { _name: String::from("???"), op: CPU6502::xxx, addr_mode: CPU6502::imp, cycles: 5 }, Instruction { _name: String::from("PLP"), op: CPU6502::plp, addr_mode: CPU6502::imp, cycles: 4 }, Instruction { _name: String::from("AND"), op: CPU6502::and, addr_mode: CPU6502::imm, cycles: 2 }, Instruction { _name: String::from("ROL"), op: CPU6502::rol, addr_mode: CPU6502::imp, cycles: 2 }, Instruction { _name: String::from("???"), op: CPU6502::xxx, addr_mode: CPU6502::imp, cycles: 2 }, Instruction { _name: String::from("BIT"), op: CPU6502::bit, addr_mode: CPU6502::abs, cycles: 4 }, Instruction { _name: String::from("AND"), op: CPU6502::and, addr_mode: CPU6502::abs, cycles: 4 }, Instruction { _name: String::from("ROL"), op: CPU6502::rol, addr_mode: CPU6502::abs, cycles: 6 }, Instruction { _name: String::from("???"), op: CPU6502::xxx, addr_mode: CPU6502::imp, cycles: 6 },
                Instruction { _name: String::from("BMI"), op: CPU6502::bmi, addr_mode: CPU6502::rel, cycles: 2 }, Instruction { _name: String::from("AND"), op: CPU6502::and, addr_mode: CPU6502::izy, cycles: 5 }, Instruction { _name: String::from("???"), op: CPU6502::xxx, addr_mode: CPU6502::imp, cycles: 2 }, Instruction { _name: String::from("???"), op: CPU6502::xxx, addr_mode: CPU6502::imp, cycles: 8 }, Instruction { _name: String::from("???"), op: CPU6502::nop, addr_mode: CPU6502::imp, cycles: 4 }, Instruction { _name: String::from("AND"), op: CPU6502::and, addr_mode: CPU6502::zpx, cycles: 4 }, Instruction { _name: String::from("ROL"), op: CPU6502::rol, addr_mode: CPU6502::zpx, cycles: 6 }, Instruction { _name: String::from("???"), op: CPU6502::xxx, addr_mode: CPU6502::imp, cycles: 6 }, Instruction { _name: String::from("SEC"), op: CPU6502::sec, addr_mode: CPU6502::imp, cycles: 2 }, Instruction { _name: String::from("AND"), op: CPU6502::and, addr_mode: CPU6502::aby, cycles: 4 }, Instruction { _name: String::from("???"), op: CPU6502::nop, addr_mode: CPU6502::imp, cycles: 2 }, Instruction { _name: String::from("???"), op: CPU6502::xxx, addr_mode: CPU6502::imp, cycles: 7 }, Instruction { _name: String::from("???"), op: CPU6502::nop, addr_mode: CPU6502::imp, cycles: 4 }, Instruction { _name: String::from("AND"), op: CPU6502::and, addr_mode: CPU6502::abx, cycles: 4 }, Instruction { _name: String::from("ROL"), op: CPU6502::rol, addr_mode: CPU6502::abx, cycles: 7 }, Instruction { _name: String::from("???"), op: CPU6502::xxx, addr_mode: CPU6502::imp, cycles: 7 },
                Instruction { _name: String::from("RTI"), op: CPU6502::rti, addr_mode: CPU6502::imp, cycles: 6 }, Instruction { _name: String::from("EOR"), op: CPU6502::eor, addr_mode: CPU6502::izx, cycles: 6 }, Instruction { _name: String::from("???"), op: CPU6502::xxx, addr_mode: CPU6502::imp, cycles: 2 }, Instruction { _name: String::from("???"), op: CPU6502::xxx, addr_mode: CPU6502::imp, cycles: 8 }, Instruction { _name: String::from("???"), op: CPU6502::nop, addr_mode: CPU6502::imp, cycles: 3 }, Instruction { _name: String::from("EOR"), op: CPU6502::eor, addr_mode: CPU6502::zp0, cycles: 3 }, Instruction { _name: String::from("LSR"), op: CPU6502::lsr, addr_mode: CPU6502::zp0, cycles: 5 }, Instruction { _name: String::from("???"), op: CPU6502::xxx, addr_mode: CPU6502::imp, cycles: 5 }, Instruction { _name: String::from("PHA"), op: CPU6502::pha, addr_mode: CPU6502::imp, cycles: 3 }, Instruction { _name: String::from("EOR"), op: CPU6502::eor, addr_mode: CPU6502::imm, cycles: 2 }, Instruction { _name: String::from("LSR"), op: CPU6502::lsr, addr_mode: CPU6502::imp, cycles: 2 }, Instruction { _name: String::from("???"), op: CPU6502::xxx, addr_mode: CPU6502::imp, cycles: 2 }, Instruction { _name: String::from("JMP"), op: CPU6502::jmp, addr_mode: CPU6502::abs, cycles: 3 }, Instruction { _name: String::from("EOR"), op: CPU6502::eor, addr_mode: CPU6502::abs, cycles: 4 }, Instruction { _name: String::from("LSR"), op: CPU6502::lsr, addr_mode: CPU6502::abs, cycles: 6 }, Instruction { _name: String::from("???"), op: CPU6502::xxx, addr_mode: CPU6502::imp, cycles: 6 },
                Instruction { _name: String::from("BVC"), op: CPU6502::bvc, addr_mode: CPU6502::rel, cycles: 2 }, Instruction { _name: String::from("EOR"), op: CPU6502::eor, addr_mode: CPU6502::izy, cycles: 5 }, Instruction { _name: String::from("???"), op: CPU6502::xxx, addr_mode: CPU6502::imp, cycles: 2 }, Instruction { _name: String::from("???"), op: CPU6502::xxx, addr_mode: CPU6502::imp, cycles: 8 }, Instruction { _name: String::from("???"), op: CPU6502::nop, addr_mode: CPU6502::imp, cycles: 4 }, Instruction { _name: String::from("EOR"), op: CPU6502::eor, addr_mode: CPU6502::zpx, cycles: 4 }, Instruction { _name: String::from("LSR"), op: CPU6502::lsr, addr_mode: CPU6502::zpx, cycles: 6 }, Instruction { _name: String::from("???"), op: CPU6502::xxx, addr_mode: CPU6502::imp, cycles: 6 }, Instruction { _name: String::from("CLI"), op: CPU6502::cli, addr_mode: CPU6502::imp, cycles: 2 }, Instruction { _name: String::from("EOR"), op: CPU6502::eor, addr_mode: CPU6502::aby, cycles: 4 }, Instruction { _name: String::from("???"), op: CPU6502::nop, addr_mode: CPU6502::imp, cycles: 2 }, Instruction { _name: String::from("???"), op: CPU6502::xxx, addr_mode: CPU6502::imp, cycles: 7 }, Instruction { _name: String::from("???"), op: CPU6502::nop, addr_mode: CPU6502::imp, cycles: 4 }, Instruction { _name: String::from("EOR"), op: CPU6502::eor, addr_mode: CPU6502::abx, cycles: 4 }, Instruction { _name: String::from("LSR"), op: CPU6502::lsr, addr_mode: CPU6502::abx, cycles: 7 }, Instruction { _name: String::from("???"), op: CPU6502::xxx, addr_mode: CPU6502::imp, cycles: 7 },
                Instruction { _name: String::from("RTS"), op: CPU6502::rts, addr_mode: CPU6502::imp, cycles: 6 }, Instruction { _name: String::from("ADC"), op: CPU6502::adc, addr_mode: CPU6502::izx, cycles: 6 }, Instruction { _name: String::from("???"), op: CPU6502::xxx, addr_mode: CPU6502::imp, cycles: 2 }, Instruction { _name: String::from("???"), op: CPU6502::xxx, addr_mode: CPU6502::imp, cycles: 8 }, Instruction { _name: String::from("???"), op: CPU6502::nop, addr_mode: CPU6502::imp, cycles: 3 }, Instruction { _name: String::from("ADC"), op: CPU6502::adc, addr_mode: CPU6502::zp0, cycles: 3 }, Instruction { _name: String::from("ROR"), op: CPU6502::ror, addr_mode: CPU6502::zp0, cycles: 5 }, Instruction { _name: String::from("???"), op: CPU6502::xxx, addr_mode: CPU6502::imp, cycles: 5 }, Instruction { _name: String::from("PLA"), op: CPU6502::pla, addr_mode: CPU6502::imp, cycles: 4 }, Instruction { _name: String::from("ADC"), op: CPU6502::adc, addr_mode: CPU6502::imm, cycles: 2 }, Instruction { _name: String::from("ROR"), op: CPU6502::ror, addr_mode: CPU6502::imp, cycles: 2 }, Instruction { _name: String::from("???"), op: CPU6502::xxx, addr_mode: CPU6502::imp, cycles: 2 }, Instruction { _name: String::from("JMP"), op: CPU6502::jmp, addr_mode: CPU6502::ind, cycles: 5 }, Instruction { _name: String::from("ADC"), op: CPU6502::adc, addr_mode: CPU6502::abs, cycles: 4 }, Instruction { _name: String::from("ROR"), op: CPU6502::ror, addr_mode: CPU6502::abs, cycles: 6 }, Instruction { _name: String::from("???"), op: CPU6502::xxx, addr_mode: CPU6502::imp, cycles: 6 },
                Instruction { _name: String::from("BVS"), op: CPU6502::bvs, addr_mode: CPU6502::rel, cycles: 2 }, Instruction { _name: String::from("ADC"), op: CPU6502::adc, addr_mode: CPU6502::izy, cycles: 5 }, Instruction { _name: String::from("???"), op: CPU6502::xxx, addr_mode: CPU6502::imp, cycles: 2 }, Instruction { _name: String::from("???"), op: CPU6502::xxx, addr_mode: CPU6502::imp, cycles: 8 }, Instruction { _name: String::from("???"), op: CPU6502::nop, addr_mode: CPU6502::imp, cycles: 4 }, Instruction { _name: String::from("ADC"), op: CPU6502::adc, addr_mode: CPU6502::zpx, cycles: 4 }, Instruction { _name: String::from("ROR"), op: CPU6502::ror, addr_mode: CPU6502::zpx, cycles: 6 }, Instruction { _name: String::from("???"), op: CPU6502::xxx, addr_mode: CPU6502::imp, cycles: 6 }, Instruction { _name: String::from("SEI"), op: CPU6502::sei, addr_mode: CPU6502::imp, cycles: 2 }, Instruction { _name: String::from("ADC"), op: CPU6502::adc, addr_mode: CPU6502::aby, cycles: 4 }, Instruction { _name: String::from("???"), op: CPU6502::nop, addr_mode: CPU6502::imp, cycles: 2 }, Instruction { _name: String::from("???"), op: CPU6502::xxx, addr_mode: CPU6502::imp, cycles: 7 }, Instruction { _name: String::from("???"), op: CPU6502::nop, addr_mode: CPU6502::imp, cycles: 4 }, Instruction { _name: String::from("ADC"), op: CPU6502::adc, addr_mode: CPU6502::abx, cycles: 4 }, Instruction { _name: String::from("ROR"), op: CPU6502::ror, addr_mode: CPU6502::abx, cycles: 7 }, Instruction { _name: String::from("???"), op: CPU6502::xxx, addr_mode: CPU6502::imp, cycles: 7 },
                Instruction { _name: String::from("???"), op: CPU6502::nop, addr_mode: CPU6502::imp, cycles: 2 }, Instruction { _name: String::from("STA"), op: CPU6502::sta, addr_mode: CPU6502::izx, cycles: 6 }, Instruction { _name: String::from("???"), op: CPU6502::nop, addr_mode: CPU6502::imp, cycles: 2 }, Instruction { _name: String::from("???"), op: CPU6502::xxx, addr_mode: CPU6502::imp, cycles: 6 }, Instruction { _name: String::from("STY"), op: CPU6502::sty, addr_mode: CPU6502::zp0, cycles: 3 }, Instruction { _name: String::from("STA"), op: CPU6502::sta, addr_mode: CPU6502::zp0, cycles: 3 }, Instruction { _name: String::from("STX"), op: CPU6502::stx, addr_mode: CPU6502::zp0, cycles: 3 }, Instruction { _name: String::from("???"), op: CPU6502::xxx, addr_mode: CPU6502::imp, cycles: 3 }, Instruction { _name: String::from("DEY"), op: CPU6502::dey, addr_mode: CPU6502::imp, cycles: 2 }, Instruction { _name: String::from("???"), op: CPU6502::nop, addr_mode: CPU6502::imp, cycles: 2 }, Instruction { _name: String::from("TXA"), op: CPU6502::txa, addr_mode: CPU6502::imp, cycles: 2 }, Instruction { _name: String::from("???"), op: CPU6502::xxx, addr_mode: CPU6502::imp, cycles: 2 }, Instruction { _name: String::from("STY"), op: CPU6502::sty, addr_mode: CPU6502::abs, cycles: 4 }, Instruction { _name: String::from("STA"), op: CPU6502::sta, addr_mode: CPU6502::abs, cycles: 4 }, Instruction { _name: String::from("STX"), op: CPU6502::stx, addr_mode: CPU6502::abs, cycles: 4 }, Instruction { _name: String::from("???"), op: CPU6502::xxx, addr_mode: CPU6502::imp, cycles: 4 },
                Instruction { _name: String::from("BCC"), op: CPU6502::bcc, addr_mode: CPU6502::rel, cycles: 2 }, Instruction { _name: String::from("STA"), op: CPU6502::sta, addr_mode: CPU6502::izy, cycles: 6 }, Instruction { _name: String::from("???"), op: CPU6502::xxx, addr_mode: CPU6502::imp, cycles: 2 }, Instruction { _name: String::from("???"), op: CPU6502::xxx, addr_mode: CPU6502::imp, cycles: 6 }, Instruction { _name: String::from("STY"), op: CPU6502::sty, addr_mode: CPU6502::zpx, cycles: 4 }, Instruction { _name: String::from("STA"), op: CPU6502::sta, addr_mode: CPU6502::zpx, cycles: 4 }, Instruction { _name: String::from("STX"), op: CPU6502::stx, addr_mode: CPU6502::zpy, cycles: 4 }, Instruction { _name: String::from("???"), op: CPU6502::xxx, addr_mode: CPU6502::imp, cycles: 4 }, Instruction { _name: String::from("TYA"), op: CPU6502::tya, addr_mode: CPU6502::imp, cycles: 2 }, Instruction { _name: String::from("STA"), op: CPU6502::sta, addr_mode: CPU6502::aby, cycles: 5 }, Instruction { _name: String::from("TXS"), op: CPU6502::txs, addr_mode: CPU6502::imp, cycles: 2 }, Instruction { _name: String::from("???"), op: CPU6502::xxx, addr_mode: CPU6502::imp, cycles: 5 }, Instruction { _name: String::from("???"), op: CPU6502::nop, addr_mode: CPU6502::imp, cycles: 5 }, Instruction { _name: String::from("STA"), op: CPU6502::sta, addr_mode: CPU6502::abx, cycles: 5 }, Instruction { _name: String::from("???"), op: CPU6502::xxx, addr_mode: CPU6502::imp, cycles: 5 }, Instruction { _name: String::from("???"), op: CPU6502::xxx, addr_mode: CPU6502::imp, cycles: 5 },
                Instruction { _name: String::from("LDY"), op: CPU6502::ldy, addr_mode: CPU6502::imm, cycles: 2 }, Instruction { _name: String::from("LDA"), op: CPU6502::lda, addr_mode: CPU6502::izx, cycles: 6 }, Instruction { _name: String::from("LDX"), op: CPU6502::ldx, addr_mode: CPU6502::imm, cycles: 2 }, Instruction { _name: String::from("???"), op: CPU6502::xxx, addr_mode: CPU6502::imp, cycles: 6 }, Instruction { _name: String::from("LDY"), op: CPU6502::ldy, addr_mode: CPU6502::zp0, cycles: 3 }, Instruction { _name: String::from("LDA"), op: CPU6502::lda, addr_mode: CPU6502::zp0, cycles: 3 }, Instruction { _name: String::from("LDX"), op: CPU6502::ldx, addr_mode: CPU6502::zp0, cycles: 3 }, Instruction { _name: String::from("???"), op: CPU6502::xxx, addr_mode: CPU6502::imp, cycles: 3 }, Instruction { _name: String::from("TAY"), op: CPU6502::tay, addr_mode: CPU6502::imp, cycles: 2 }, Instruction { _name: String::from("LDA"), op: CPU6502::lda, addr_mode: CPU6502::imm, cycles: 2 }, Instruction { _name: String::from("TAX"), op: CPU6502::tax, addr_mode: CPU6502::imp, cycles: 2 }, Instruction { _name: String::from("???"), op: CPU6502::xxx, addr_mode: CPU6502::imp, cycles: 2 }, Instruction { _name: String::from("LDY"), op: CPU6502::ldy, addr_mode: CPU6502::abs, cycles: 4 }, Instruction { _name: String::from("LDA"), op: CPU6502::lda, addr_mode: CPU6502::abs, cycles: 4 }, Instruction { _name: String::from("LDX"), op: CPU6502::ldx, addr_mode: CPU6502::abs, cycles: 4 }, Instruction { _name: String::from("???"), op: CPU6502::xxx, addr_mode: CPU6502::imp, cycles: 4 },
                Instruction { _name: String::from("BCS"), op: CPU6502::bcs, addr_mode: CPU6502::rel, cycles: 2 }, Instruction { _name: String::from("LDA"), op: CPU6502::lda, addr_mode: CPU6502::izy, cycles: 5 }, Instruction { _name: String::from("???"), op: CPU6502::xxx, addr_mode: CPU6502::imp, cycles: 2 }, Instruction { _name: String::from("???"), op: CPU6502::xxx, addr_mode: CPU6502::imp, cycles: 5 }, Instruction { _name: String::from("LDY"), op: CPU6502::ldy, addr_mode: CPU6502::zpx, cycles: 4 }, Instruction { _name: String::from("LDA"), op: CPU6502::lda, addr_mode: CPU6502::zpx, cycles: 4 }, Instruction { _name: String::from("LDX"), op: CPU6502::ldx, addr_mode: CPU6502::zpy, cycles: 4 }, Instruction { _name: String::from("???"), op: CPU6502::xxx, addr_mode: CPU6502::imp, cycles: 4 }, Instruction { _name: String::from("CLV"), op: CPU6502::clv, addr_mode: CPU6502::imp, cycles: 2 }, Instruction { _name: String::from("LDA"), op: CPU6502::lda, addr_mode: CPU6502::aby, cycles: 4 }, Instruction { _name: String::from("TSX"), op: CPU6502::tsx, addr_mode: CPU6502::imp, cycles: 2 }, Instruction { _name: String::from("???"), op: CPU6502::xxx, addr_mode: CPU6502::imp, cycles: 4 }, Instruction { _name: String::from("LDY"), op: CPU6502::ldy, addr_mode: CPU6502::abx, cycles: 4 }, Instruction { _name: String::from("LDA"), op: CPU6502::lda, addr_mode: CPU6502::abx, cycles: 4 }, Instruction { _name: String::from("LDX"), op: CPU6502::ldx, addr_mode: CPU6502::aby, cycles: 4 }, Instruction { _name: String::from("???"), op: CPU6502::xxx, addr_mode: CPU6502::imp, cycles: 4 },
                Instruction { _name: String::from("CPY"), op: CPU6502::cpy, addr_mode: CPU6502::imm, cycles: 2 }, Instruction { _name: String::from("CMP"), op: CPU6502::cmp, addr_mode: CPU6502::izx, cycles: 6 }, Instruction { _name: String::from("???"), op: CPU6502::nop, addr_mode: CPU6502::imp, cycles: 2 }, Instruction { _name: String::from("???"), op: CPU6502::xxx, addr_mode: CPU6502::imp, cycles: 8 }, Instruction { _name: String::from("CPY"), op: CPU6502::cpy, addr_mode: CPU6502::zp0, cycles: 3 }, Instruction { _name: String::from("CMP"), op: CPU6502::cmp, addr_mode: CPU6502::zp0, cycles: 3 }, Instruction { _name: String::from("DEC"), op: CPU6502::dec, addr_mode: CPU6502::zp0, cycles: 5 }, Instruction { _name: String::from("???"), op: CPU6502::xxx, addr_mode: CPU6502::imp, cycles: 5 }, Instruction { _name: String::from("INY"), op: CPU6502::iny, addr_mode: CPU6502::imp, cycles: 2 }, Instruction { _name: String::from("CMP"), op: CPU6502::cmp, addr_mode: CPU6502::imm, cycles: 2 }, Instruction { _name: String::from("DEX"), op: CPU6502::dex, addr_mode: CPU6502::imp, cycles: 2 }, Instruction { _name: String::from("???"), op: CPU6502::xxx, addr_mode: CPU6502::imp, cycles: 2 }, Instruction { _name: String::from("CPY"), op: CPU6502::cpy, addr_mode: CPU6502::abs, cycles: 4 }, Instruction { _name: String::from("CMP"), op: CPU6502::cmp, addr_mode: CPU6502::abs, cycles: 4 }, Instruction { _name: String::from("DEC"), op: CPU6502::dec, addr_mode: CPU6502::abs, cycles: 6 }, Instruction { _name: String::from("???"), op: CPU6502::xxx, addr_mode: CPU6502::imp, cycles: 6 },
                Instruction { _name: String::from("BNE"), op: CPU6502::bne, addr_mode: CPU6502::rel, cycles: 2 }, Instruction { _name: String::from("CMP"), op: CPU6502::cmp, addr_mode: CPU6502::izy, cycles: 5 }, Instruction { _name: String::from("???"), op: CPU6502::xxx, addr_mode: CPU6502::imp, cycles: 2 }, Instruction { _name: String::from("???"), op: CPU6502::xxx, addr_mode: CPU6502::imp, cycles: 8 }, Instruction { _name: String::from("???"), op: CPU6502::nop, addr_mode: CPU6502::imp, cycles: 4 }, Instruction { _name: String::from("CMP"), op: CPU6502::cmp, addr_mode: CPU6502::zpx, cycles: 4 }, Instruction { _name: String::from("DEC"), op: CPU6502::dec, addr_mode: CPU6502::zpx, cycles: 6 }, Instruction { _name: String::from("???"), op: CPU6502::xxx, addr_mode: CPU6502::imp, cycles: 6 }, Instruction { _name: String::from("CLD"), op: CPU6502::cld, addr_mode: CPU6502::imp, cycles: 2 }, Instruction { _name: String::from("CMP"), op: CPU6502::cmp, addr_mode: CPU6502::aby, cycles: 4 }, Instruction { _name: String::from("NOP"), op: CPU6502::nop, addr_mode: CPU6502::imp, cycles: 2 }, Instruction { _name: String::from("???"), op: CPU6502::xxx, addr_mode: CPU6502::imp, cycles: 7 }, Instruction { _name: String::from("???"), op: CPU6502::nop, addr_mode: CPU6502::imp, cycles: 4 }, Instruction { _name: String::from("CMP"), op: CPU6502::cmp, addr_mode: CPU6502::abx, cycles: 4 }, Instruction { _name: String::from("DEC"), op: CPU6502::dec, addr_mode: CPU6502::abx, cycles: 7 }, Instruction { _name: String::from("???"), op: CPU6502::xxx, addr_mode: CPU6502::imp, cycles: 7 },
                Instruction { _name: String::from("CPX"), op: CPU6502::cpx, addr_mode: CPU6502::imm, cycles: 2 }, Instruction { _name: String::from("SBC"), op: CPU6502::sbc, addr_mode: CPU6502::izx, cycles: 6 }, Instruction { _name: String::from("???"), op: CPU6502::nop, addr_mode: CPU6502::imp, cycles: 2 }, Instruction { _name: String::from("???"), op: CPU6502::xxx, addr_mode: CPU6502::imp, cycles: 8 }, Instruction { _name: String::from("CPX"), op: CPU6502::cpx, addr_mode: CPU6502::zp0, cycles: 3 }, Instruction { _name: String::from("SBC"), op: CPU6502::sbc, addr_mode: CPU6502::zp0, cycles: 3 }, Instruction { _name: String::from("INC"), op: CPU6502::inc, addr_mode: CPU6502::zp0, cycles: 5 }, Instruction { _name: String::from("???"), op: CPU6502::xxx, addr_mode: CPU6502::imp, cycles: 5 }, Instruction { _name: String::from("INX"), op: CPU6502::inx, addr_mode: CPU6502::imp, cycles: 2 }, Instruction { _name: String::from("SBC"), op: CPU6502::sbc, addr_mode: CPU6502::imm, cycles: 2 }, Instruction { _name: String::from("NOP"), op: CPU6502::nop, addr_mode: CPU6502::imp, cycles: 2 }, Instruction { _name: String::from("???"), op: CPU6502::sbc, addr_mode: CPU6502::imp, cycles: 2 }, Instruction { _name: String::from("CPX"), op: CPU6502::cpx, addr_mode: CPU6502::abs, cycles: 4 }, Instruction { _name: String::from("SBC"), op: CPU6502::sbc, addr_mode: CPU6502::abs, cycles: 4 }, Instruction { _name: String::from("INC"), op: CPU6502::inc, addr_mode: CPU6502::abs, cycles: 6 }, Instruction { _name: String::from("???"), op: CPU6502::xxx, addr_mode: CPU6502::imp, cycles: 6 },
                Instruction { _name: String::from("BEQ"), op: CPU6502::beq, addr_mode: CPU6502::rel, cycles: 2 }, Instruction { _name: String::from("SBC"), op: CPU6502::sbc, addr_mode: CPU6502::izy, cycles: 5 }, Instruction { _name: String::from("???"), op: CPU6502::xxx, addr_mode: CPU6502::imp, cycles: 2 }, Instruction { _name: String::from("???"), op: CPU6502::xxx, addr_mode: CPU6502::imp, cycles: 8 }, Instruction { _name: String::from("???"), op: CPU6502::nop, addr_mode: CPU6502::imp, cycles: 4 }, Instruction { _name: String::from("SBC"), op: CPU6502::sbc, addr_mode: CPU6502::zpx, cycles: 4 }, Instruction { _name: String::from("INC"), op: CPU6502::inc, addr_mode: CPU6502::zpx, cycles: 6 }, Instruction { _name: String::from("???"), op: CPU6502::xxx, addr_mode: CPU6502::imp, cycles: 6 }, Instruction { _name: String::from("SED"), op: CPU6502::sed, addr_mode: CPU6502::imp, cycles: 2 }, Instruction { _name: String::from("SBC"), op: CPU6502::sbc, addr_mode: CPU6502::aby, cycles: 4 }, Instruction { _name: String::from("NOP"), op: CPU6502::nop, addr_mode: CPU6502::imp, cycles: 2 }, Instruction { _name: String::from("???"), op: CPU6502::xxx, addr_mode: CPU6502::imp, cycles: 7 }, Instruction { _name: String::from("???"), op: CPU6502::nop, addr_mode: CPU6502::imp, cycles: 4 }, Instruction { _name: String::from("SBC"), op: CPU6502::sbc, addr_mode: CPU6502::abx, cycles: 4 }, Instruction { _name: String::from("INC"), op: CPU6502::inc, addr_mode: CPU6502::abx, cycles: 7 }, Instruction { _name: String::from("???"), op: CPU6502::xxx, addr_mode: CPU6502::imp, cycles: 7 },
            ]
        };

        return cpu;
    }

    pub fn get_flag(&self, f: Flags6502) -> u8
    {
        return self.status & (1 << (f as u8));
    }

    pub fn set_flag(&mut self, f: Flags6502, b: bool)
    {
        self.status = self.status | ((b as u8) << (f as u8));
    }

    // #region Addressing Modes
    pub fn imp(&mut self) -> u8
    {
        // The instruction may operate on the accumulator
        self.fetched_data = self.a;
        return 0;
    }

    pub fn imm(&mut self) -> u8
    {
        self.addr_abs = self.pc;
        self.pc += 1;
        return 0;
    }

    pub fn zp0(&mut self) -> u8
    {
        // Address in the zero page (plus a byte offset that is read)
        self.addr_abs = self.read(self.pc) as u16;
        self.pc += 1;
        self.addr_abs &= 0x00FF;
        return 0;
    }

    pub fn zpx(&mut self) -> u8
    {
        // Address in the zero page (plus a byte offset that is read and incremented by X register)
        // This is used for iterating through ranges in zero page
        self.addr_abs = (self.read(self.pc) + self.x) as u16;
        self.pc += 1;
        self.addr_abs &= 0x00FF;
        return 0;
    }

    pub fn zpy(&mut self) -> u8
    {
        // Address in the zero page (plus a byte offset that is read and incremented by Y register)
        self.addr_abs = (self.read(self.pc) + self.y) as u16;
        self.pc += 1;
        self.addr_abs &= 0x00FF;
        return 0;
    }

    pub fn rel(&mut self) -> u8
    {
        self.addr_rel = self.read(self.pc) as u16;
        self.pc += 1;

        // If we have a negative offset, we need to set upper bits to 1
        if self.addr_rel & 0x80 == 0x80
        {
            self.addr_rel |= 0xFF00;
        }

        return 0;
    }

    pub fn abs(&mut self) -> u8
    {
        // Absolute addressing (full u16 address)
        let lo: u16 = self.read(self.pc) as u16;
        self.pc += 1;
        let hi: u16 = self.read(self.pc) as u16;
        self.pc += 1;

        self.addr_abs = (hi << 8) | lo;
        return 0; 
    }

    pub fn abx(&mut self) -> u8
    {
        // Absolute addressing (full u16 address) with X offset
        let lo: u16 = self.read(self.pc) as u16;
        self.pc += 1;
        let hi: u16 = self.read(self.pc) as u16;
        self.pc += 1;
    
        self.addr_abs = (hi << 8) | lo;
        self.addr_abs += self.x as u16;
    
        // If this crosses a page boundary, we need an additional clock cycle for this instruction
        if (self.addr_abs & 0xFF00) != (hi << 8)
        {
            return 1;
        }
        else
        {
            return 0;	
        }
    }

    pub fn aby(&mut self) -> u8
    {
        // Absolute addressing (full u16 address) with Y offset
        let lo: u16 = self.read(self.pc) as u16;
        self.pc += 1;
        let hi: u16 = self.read(self.pc) as u16;
        self.pc += 1;

        self.addr_abs = (hi << 8) | lo;
        self.addr_abs += self.y as u16;

        // If this crosses a page boundary, we need an additional clock cycle for this instruction
        if (self.addr_abs & 0xFF00) != (hi << 8)
        {
            return 1;
        }
        else
        {
            return 0;	
        }
    }

    // TODO: Stopped here, this is indirect addressing
    pub fn ind(&mut self) -> u8 { return 0; }
    pub fn izx(&mut self) -> u8 { return 0; }
    pub fn izy(&mut self) -> u8 { return 0; }

    // Opcodes
    pub fn adc(&mut self) -> u8 { return 0; }
    pub fn and(&mut self) -> u8 { return 0; }
    pub fn asl(&mut self) -> u8 { return 0; }
    pub fn bcc(&mut self) -> u8 { return 0; }
    pub fn bcs(&mut self) -> u8 { return 0; }
    pub fn beq(&mut self) -> u8 { return 0; }
    pub fn bit(&mut self) -> u8 { return 0; }
    pub fn bmi(&mut self) -> u8 { return 0; }
    pub fn bne(&mut self) -> u8 { return 0; }
    pub fn bpl(&mut self) -> u8 { return 0; }
    pub fn brk(&mut self) -> u8 { return 0; }
    pub fn bvc(&mut self) -> u8 { return 0; }
    pub fn bvs(&mut self) -> u8 { return 0; }
    pub fn clc(&mut self) -> u8 { return 0; }
    pub fn cld(&mut self) -> u8 { return 0; }
    pub fn cli(&mut self) -> u8 { return 0; }
    pub fn clv(&mut self) -> u8 { return 0; }
    pub fn cmp(&mut self) -> u8 { return 0; }
    pub fn cpx(&mut self) -> u8 { return 0; }
    pub fn cpy(&mut self) -> u8 { return 0; }
    pub fn dec(&mut self) -> u8 { return 0; }
    pub fn dex(&mut self) -> u8 { return 0; }
    pub fn dey(&mut self) -> u8 { return 0; }
    pub fn eor(&mut self) -> u8 { return 0; }
    pub fn inc(&mut self) -> u8 { return 0; }
    pub fn inx(&mut self) -> u8 { return 0; }
    pub fn iny(&mut self) -> u8 { return 0; }
    pub fn jmp(&mut self) -> u8 { return 0; }
    pub fn jsr(&mut self) -> u8 { return 0; }
    pub fn lda(&mut self) -> u8 { return 0; }
    pub fn ldx(&mut self) -> u8 { return 0; }
    pub fn ldy(&mut self) -> u8 { return 0; }
    pub fn lsr(&mut self) -> u8 { return 0; }
    pub fn nop(&mut self) -> u8 { return 0; }
    pub fn ora(&mut self) -> u8 { return 0; }
    pub fn pha(&mut self) -> u8 { return 0; }
    pub fn php(&mut self) -> u8 { return 0; }
    pub fn pla(&mut self) -> u8 { return 0; }
    pub fn plp(&mut self) -> u8 { return 0; }
    pub fn rol(&mut self) -> u8 { return 0; }
    pub fn ror(&mut self) -> u8 { return 0; }
    pub fn rti(&mut self) -> u8 { return 0; }
    pub fn rts(&mut self) -> u8 { return 0; }
    pub fn sbc(&mut self) -> u8 { return 0; }
    pub fn sec(&mut self) -> u8 { return 0; }
    pub fn sed(&mut self) -> u8 { return 0; }
    pub fn sei(&mut self) -> u8 { return 0; }
    pub fn sta(&mut self) -> u8 { return 0; }
    pub fn stx(&mut self) -> u8 { return 0; }
    pub fn sty(&mut self) -> u8 { return 0; }
    pub fn tax(&mut self) -> u8 { return 0; }
    pub fn tay(&mut self) -> u8 { return 0; }
    pub fn tsx(&mut self) -> u8 { return 0; }
    pub fn txa(&mut self) -> u8 { return 0; }
    pub fn txs(&mut self) -> u8 { return 0; }
    pub fn tya(&mut self) -> u8 { return 0; }

    
    pub fn xxx(&mut self) -> u8 { return 0; }

    // Other signals
    pub fn clock_tick(&mut self)
    {
        if self.cycles == 0
        {
            self.opcode = self.read(self.pc);
            self.pc += 1;

            self.cycles = self.ins[self.opcode as usize].cycles;
            let additional_cycle1: u8 = (self.ins[self.opcode as usize].addr_mode)(self);
            let additional_cycle2: u8 = (self.ins[self.opcode as usize].op)(self);

            // TODO: Why is this a binary AND?
            self.cycles += additional_cycle1 & additional_cycle2;
        }

        self.cycles -= 1;
    }

    pub fn reset(&mut self) {}
    pub fn irq(&mut self) {}
    pub fn nmi(&mut self) {}

    // Helpers
    pub fn fetch() {}

}

impl<'a> ReadWrite for CPU6502<'a>
{
    fn write(&mut self, address: u16, data: u8)
    {
        self.bus.write(address, data);
    }

    fn read(&self, address: u16) -> u8
    {
        return self.bus.read(address);
    }
}