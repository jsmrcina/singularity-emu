use crate::traits::ReadWrite;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::BTreeMap;

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

struct Instruction<'a>
{
    name: String,
    op: fn(&mut CPU6502<'a>) -> u8,
    addr_mode: fn(&mut CPU6502<'a>) -> u8,
    cycles: u8
}

pub struct CPU6502<'a>
{
    bus: Option<Rc<RefCell<dyn ReadWrite>>>,
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
    ins: [Instruction<'a>; 256]
}

impl<'a> CPU6502<'a>
{
    const STACK_START_ADDRESS: u16 = 0x1000;
    const INTERRUPT_VECTOR: [u16; 2] = [0xFFFE, 0xFFFF];

    pub fn new() -> Self
    {
        let cpu = CPU6502
        {
            bus: None,
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
            ins: [
                Instruction { name: String::from("BRK"), op: CPU6502::brk, addr_mode: CPU6502::imm, cycles: 7 }, Instruction { name: String::from("ORA"), op: CPU6502::ora, addr_mode: CPU6502::izx, cycles: 6 }, Instruction { name: String::from("???"), op: CPU6502::xxx, addr_mode: CPU6502::imp, cycles: 2 }, Instruction { name: String::from("???"), op: CPU6502::xxx, addr_mode: CPU6502::imp, cycles: 8 }, Instruction { name: String::from("???"), op: CPU6502::nop, addr_mode: CPU6502::imp, cycles: 3 }, Instruction { name: String::from("ORA"), op: CPU6502::ora, addr_mode: CPU6502::zp0, cycles: 3 }, Instruction { name: String::from("ASL"), op: CPU6502::asl, addr_mode: CPU6502::zp0, cycles: 5 }, Instruction { name: String::from("???"), op: CPU6502::xxx, addr_mode: CPU6502::imp, cycles: 5 }, Instruction { name: String::from("PHP"), op: CPU6502::php, addr_mode: CPU6502::imp, cycles: 3 }, Instruction { name: String::from("ORA"), op: CPU6502::ora, addr_mode: CPU6502::imm, cycles: 2 }, Instruction { name: String::from("ASL"), op: CPU6502::asl, addr_mode: CPU6502::imp, cycles: 2 }, Instruction { name: String::from("???"), op: CPU6502::xxx, addr_mode: CPU6502::imp, cycles: 2 }, Instruction { name: String::from("???"), op: CPU6502::nop, addr_mode: CPU6502::imp, cycles: 4 }, Instruction { name: String::from("ORA"), op: CPU6502::ora, addr_mode: CPU6502::abs, cycles: 4 }, Instruction { name: String::from("ASL"), op: CPU6502::asl, addr_mode: CPU6502::abs, cycles: 6 }, Instruction { name: String::from("???"), op: CPU6502::xxx, addr_mode: CPU6502::imp, cycles: 6 },
                Instruction { name: String::from("BPL"), op: CPU6502::bpl, addr_mode: CPU6502::rel, cycles: 2 }, Instruction { name: String::from("ORA"), op: CPU6502::ora, addr_mode: CPU6502::izy, cycles: 5 }, Instruction { name: String::from("???"), op: CPU6502::xxx, addr_mode: CPU6502::imp, cycles: 2 }, Instruction { name: String::from("???"), op: CPU6502::xxx, addr_mode: CPU6502::imp, cycles: 8 }, Instruction { name: String::from("???"), op: CPU6502::nop, addr_mode: CPU6502::imp, cycles: 4 }, Instruction { name: String::from("ORA"), op: CPU6502::ora, addr_mode: CPU6502::zpx, cycles: 4 }, Instruction { name: String::from("ASL"), op: CPU6502::asl, addr_mode: CPU6502::zpx, cycles: 6 }, Instruction { name: String::from("???"), op: CPU6502::xxx, addr_mode: CPU6502::imp, cycles: 6 }, Instruction { name: String::from("CLC"), op: CPU6502::clc, addr_mode: CPU6502::imp, cycles: 2 }, Instruction { name: String::from("ORA"), op: CPU6502::ora, addr_mode: CPU6502::aby, cycles: 4 }, Instruction { name: String::from("???"), op: CPU6502::nop, addr_mode: CPU6502::imp, cycles: 2 }, Instruction { name: String::from("???"), op: CPU6502::xxx, addr_mode: CPU6502::imp, cycles: 7 }, Instruction { name: String::from("???"), op: CPU6502::nop, addr_mode: CPU6502::imp, cycles: 4 }, Instruction { name: String::from("ORA"), op: CPU6502::ora, addr_mode: CPU6502::abx, cycles: 4 }, Instruction { name: String::from("ASL"), op: CPU6502::asl, addr_mode: CPU6502::abx, cycles: 7 }, Instruction { name: String::from("???"), op: CPU6502::xxx, addr_mode: CPU6502::imp, cycles: 7 },
                Instruction { name: String::from("JSR"), op: CPU6502::jsr, addr_mode: CPU6502::abs, cycles: 6 }, Instruction { name: String::from("AND"), op: CPU6502::and, addr_mode: CPU6502::izx, cycles: 6 }, Instruction { name: String::from("???"), op: CPU6502::xxx, addr_mode: CPU6502::imp, cycles: 2 }, Instruction { name: String::from("???"), op: CPU6502::xxx, addr_mode: CPU6502::imp, cycles: 8 }, Instruction { name: String::from("BIT"), op: CPU6502::bit, addr_mode: CPU6502::zp0, cycles: 3 }, Instruction { name: String::from("AND"), op: CPU6502::and, addr_mode: CPU6502::zp0, cycles: 3 }, Instruction { name: String::from("ROL"), op: CPU6502::rol, addr_mode: CPU6502::zp0, cycles: 5 }, Instruction { name: String::from("???"), op: CPU6502::xxx, addr_mode: CPU6502::imp, cycles: 5 }, Instruction { name: String::from("PLP"), op: CPU6502::plp, addr_mode: CPU6502::imp, cycles: 4 }, Instruction { name: String::from("AND"), op: CPU6502::and, addr_mode: CPU6502::imm, cycles: 2 }, Instruction { name: String::from("ROL"), op: CPU6502::rol, addr_mode: CPU6502::imp, cycles: 2 }, Instruction { name: String::from("???"), op: CPU6502::xxx, addr_mode: CPU6502::imp, cycles: 2 }, Instruction { name: String::from("BIT"), op: CPU6502::bit, addr_mode: CPU6502::abs, cycles: 4 }, Instruction { name: String::from("AND"), op: CPU6502::and, addr_mode: CPU6502::abs, cycles: 4 }, Instruction { name: String::from("ROL"), op: CPU6502::rol, addr_mode: CPU6502::abs, cycles: 6 }, Instruction { name: String::from("???"), op: CPU6502::xxx, addr_mode: CPU6502::imp, cycles: 6 },
                Instruction { name: String::from("BMI"), op: CPU6502::bmi, addr_mode: CPU6502::rel, cycles: 2 }, Instruction { name: String::from("AND"), op: CPU6502::and, addr_mode: CPU6502::izy, cycles: 5 }, Instruction { name: String::from("???"), op: CPU6502::xxx, addr_mode: CPU6502::imp, cycles: 2 }, Instruction { name: String::from("???"), op: CPU6502::xxx, addr_mode: CPU6502::imp, cycles: 8 }, Instruction { name: String::from("???"), op: CPU6502::nop, addr_mode: CPU6502::imp, cycles: 4 }, Instruction { name: String::from("AND"), op: CPU6502::and, addr_mode: CPU6502::zpx, cycles: 4 }, Instruction { name: String::from("ROL"), op: CPU6502::rol, addr_mode: CPU6502::zpx, cycles: 6 }, Instruction { name: String::from("???"), op: CPU6502::xxx, addr_mode: CPU6502::imp, cycles: 6 }, Instruction { name: String::from("SEC"), op: CPU6502::sec, addr_mode: CPU6502::imp, cycles: 2 }, Instruction { name: String::from("AND"), op: CPU6502::and, addr_mode: CPU6502::aby, cycles: 4 }, Instruction { name: String::from("???"), op: CPU6502::nop, addr_mode: CPU6502::imp, cycles: 2 }, Instruction { name: String::from("???"), op: CPU6502::xxx, addr_mode: CPU6502::imp, cycles: 7 }, Instruction { name: String::from("???"), op: CPU6502::nop, addr_mode: CPU6502::imp, cycles: 4 }, Instruction { name: String::from("AND"), op: CPU6502::and, addr_mode: CPU6502::abx, cycles: 4 }, Instruction { name: String::from("ROL"), op: CPU6502::rol, addr_mode: CPU6502::abx, cycles: 7 }, Instruction { name: String::from("???"), op: CPU6502::xxx, addr_mode: CPU6502::imp, cycles: 7 },
                Instruction { name: String::from("RTI"), op: CPU6502::rti, addr_mode: CPU6502::imp, cycles: 6 }, Instruction { name: String::from("EOR"), op: CPU6502::eor, addr_mode: CPU6502::izx, cycles: 6 }, Instruction { name: String::from("???"), op: CPU6502::xxx, addr_mode: CPU6502::imp, cycles: 2 }, Instruction { name: String::from("???"), op: CPU6502::xxx, addr_mode: CPU6502::imp, cycles: 8 }, Instruction { name: String::from("???"), op: CPU6502::nop, addr_mode: CPU6502::imp, cycles: 3 }, Instruction { name: String::from("EOR"), op: CPU6502::eor, addr_mode: CPU6502::zp0, cycles: 3 }, Instruction { name: String::from("LSR"), op: CPU6502::lsr, addr_mode: CPU6502::zp0, cycles: 5 }, Instruction { name: String::from("???"), op: CPU6502::xxx, addr_mode: CPU6502::imp, cycles: 5 }, Instruction { name: String::from("PHA"), op: CPU6502::pha, addr_mode: CPU6502::imp, cycles: 3 }, Instruction { name: String::from("EOR"), op: CPU6502::eor, addr_mode: CPU6502::imm, cycles: 2 }, Instruction { name: String::from("LSR"), op: CPU6502::lsr, addr_mode: CPU6502::imp, cycles: 2 }, Instruction { name: String::from("???"), op: CPU6502::xxx, addr_mode: CPU6502::imp, cycles: 2 }, Instruction { name: String::from("JMP"), op: CPU6502::jmp, addr_mode: CPU6502::abs, cycles: 3 }, Instruction { name: String::from("EOR"), op: CPU6502::eor, addr_mode: CPU6502::abs, cycles: 4 }, Instruction { name: String::from("LSR"), op: CPU6502::lsr, addr_mode: CPU6502::abs, cycles: 6 }, Instruction { name: String::from("???"), op: CPU6502::xxx, addr_mode: CPU6502::imp, cycles: 6 },
                Instruction { name: String::from("BVC"), op: CPU6502::bvc, addr_mode: CPU6502::rel, cycles: 2 }, Instruction { name: String::from("EOR"), op: CPU6502::eor, addr_mode: CPU6502::izy, cycles: 5 }, Instruction { name: String::from("???"), op: CPU6502::xxx, addr_mode: CPU6502::imp, cycles: 2 }, Instruction { name: String::from("???"), op: CPU6502::xxx, addr_mode: CPU6502::imp, cycles: 8 }, Instruction { name: String::from("???"), op: CPU6502::nop, addr_mode: CPU6502::imp, cycles: 4 }, Instruction { name: String::from("EOR"), op: CPU6502::eor, addr_mode: CPU6502::zpx, cycles: 4 }, Instruction { name: String::from("LSR"), op: CPU6502::lsr, addr_mode: CPU6502::zpx, cycles: 6 }, Instruction { name: String::from("???"), op: CPU6502::xxx, addr_mode: CPU6502::imp, cycles: 6 }, Instruction { name: String::from("CLI"), op: CPU6502::cli, addr_mode: CPU6502::imp, cycles: 2 }, Instruction { name: String::from("EOR"), op: CPU6502::eor, addr_mode: CPU6502::aby, cycles: 4 }, Instruction { name: String::from("???"), op: CPU6502::nop, addr_mode: CPU6502::imp, cycles: 2 }, Instruction { name: String::from("???"), op: CPU6502::xxx, addr_mode: CPU6502::imp, cycles: 7 }, Instruction { name: String::from("???"), op: CPU6502::nop, addr_mode: CPU6502::imp, cycles: 4 }, Instruction { name: String::from("EOR"), op: CPU6502::eor, addr_mode: CPU6502::abx, cycles: 4 }, Instruction { name: String::from("LSR"), op: CPU6502::lsr, addr_mode: CPU6502::abx, cycles: 7 }, Instruction { name: String::from("???"), op: CPU6502::xxx, addr_mode: CPU6502::imp, cycles: 7 },
                Instruction { name: String::from("RTS"), op: CPU6502::rts, addr_mode: CPU6502::imp, cycles: 6 }, Instruction { name: String::from("ADC"), op: CPU6502::adc, addr_mode: CPU6502::izx, cycles: 6 }, Instruction { name: String::from("???"), op: CPU6502::xxx, addr_mode: CPU6502::imp, cycles: 2 }, Instruction { name: String::from("???"), op: CPU6502::xxx, addr_mode: CPU6502::imp, cycles: 8 }, Instruction { name: String::from("???"), op: CPU6502::nop, addr_mode: CPU6502::imp, cycles: 3 }, Instruction { name: String::from("ADC"), op: CPU6502::adc, addr_mode: CPU6502::zp0, cycles: 3 }, Instruction { name: String::from("ROR"), op: CPU6502::ror, addr_mode: CPU6502::zp0, cycles: 5 }, Instruction { name: String::from("???"), op: CPU6502::xxx, addr_mode: CPU6502::imp, cycles: 5 }, Instruction { name: String::from("PLA"), op: CPU6502::pla, addr_mode: CPU6502::imp, cycles: 4 }, Instruction { name: String::from("ADC"), op: CPU6502::adc, addr_mode: CPU6502::imm, cycles: 2 }, Instruction { name: String::from("ROR"), op: CPU6502::ror, addr_mode: CPU6502::imp, cycles: 2 }, Instruction { name: String::from("???"), op: CPU6502::xxx, addr_mode: CPU6502::imp, cycles: 2 }, Instruction { name: String::from("JMP"), op: CPU6502::jmp, addr_mode: CPU6502::ind, cycles: 5 }, Instruction { name: String::from("ADC"), op: CPU6502::adc, addr_mode: CPU6502::abs, cycles: 4 }, Instruction { name: String::from("ROR"), op: CPU6502::ror, addr_mode: CPU6502::abs, cycles: 6 }, Instruction { name: String::from("???"), op: CPU6502::xxx, addr_mode: CPU6502::imp, cycles: 6 },
                Instruction { name: String::from("BVS"), op: CPU6502::bvs, addr_mode: CPU6502::rel, cycles: 2 }, Instruction { name: String::from("ADC"), op: CPU6502::adc, addr_mode: CPU6502::izy, cycles: 5 }, Instruction { name: String::from("???"), op: CPU6502::xxx, addr_mode: CPU6502::imp, cycles: 2 }, Instruction { name: String::from("???"), op: CPU6502::xxx, addr_mode: CPU6502::imp, cycles: 8 }, Instruction { name: String::from("???"), op: CPU6502::nop, addr_mode: CPU6502::imp, cycles: 4 }, Instruction { name: String::from("ADC"), op: CPU6502::adc, addr_mode: CPU6502::zpx, cycles: 4 }, Instruction { name: String::from("ROR"), op: CPU6502::ror, addr_mode: CPU6502::zpx, cycles: 6 }, Instruction { name: String::from("???"), op: CPU6502::xxx, addr_mode: CPU6502::imp, cycles: 6 }, Instruction { name: String::from("SEI"), op: CPU6502::sei, addr_mode: CPU6502::imp, cycles: 2 }, Instruction { name: String::from("ADC"), op: CPU6502::adc, addr_mode: CPU6502::aby, cycles: 4 }, Instruction { name: String::from("???"), op: CPU6502::nop, addr_mode: CPU6502::imp, cycles: 2 }, Instruction { name: String::from("???"), op: CPU6502::xxx, addr_mode: CPU6502::imp, cycles: 7 }, Instruction { name: String::from("???"), op: CPU6502::nop, addr_mode: CPU6502::imp, cycles: 4 }, Instruction { name: String::from("ADC"), op: CPU6502::adc, addr_mode: CPU6502::abx, cycles: 4 }, Instruction { name: String::from("ROR"), op: CPU6502::ror, addr_mode: CPU6502::abx, cycles: 7 }, Instruction { name: String::from("???"), op: CPU6502::xxx, addr_mode: CPU6502::imp, cycles: 7 },
                Instruction { name: String::from("???"), op: CPU6502::nop, addr_mode: CPU6502::imp, cycles: 2 }, Instruction { name: String::from("STA"), op: CPU6502::sta, addr_mode: CPU6502::izx, cycles: 6 }, Instruction { name: String::from("???"), op: CPU6502::nop, addr_mode: CPU6502::imp, cycles: 2 }, Instruction { name: String::from("???"), op: CPU6502::xxx, addr_mode: CPU6502::imp, cycles: 6 }, Instruction { name: String::from("STY"), op: CPU6502::sty, addr_mode: CPU6502::zp0, cycles: 3 }, Instruction { name: String::from("STA"), op: CPU6502::sta, addr_mode: CPU6502::zp0, cycles: 3 }, Instruction { name: String::from("STX"), op: CPU6502::stx, addr_mode: CPU6502::zp0, cycles: 3 }, Instruction { name: String::from("???"), op: CPU6502::xxx, addr_mode: CPU6502::imp, cycles: 3 }, Instruction { name: String::from("DEY"), op: CPU6502::dey, addr_mode: CPU6502::imp, cycles: 2 }, Instruction { name: String::from("???"), op: CPU6502::nop, addr_mode: CPU6502::imp, cycles: 2 }, Instruction { name: String::from("TXA"), op: CPU6502::txa, addr_mode: CPU6502::imp, cycles: 2 }, Instruction { name: String::from("???"), op: CPU6502::xxx, addr_mode: CPU6502::imp, cycles: 2 }, Instruction { name: String::from("STY"), op: CPU6502::sty, addr_mode: CPU6502::abs, cycles: 4 }, Instruction { name: String::from("STA"), op: CPU6502::sta, addr_mode: CPU6502::abs, cycles: 4 }, Instruction { name: String::from("STX"), op: CPU6502::stx, addr_mode: CPU6502::abs, cycles: 4 }, Instruction { name: String::from("???"), op: CPU6502::xxx, addr_mode: CPU6502::imp, cycles: 4 },
                Instruction { name: String::from("BCC"), op: CPU6502::bcc, addr_mode: CPU6502::rel, cycles: 2 }, Instruction { name: String::from("STA"), op: CPU6502::sta, addr_mode: CPU6502::izy, cycles: 6 }, Instruction { name: String::from("???"), op: CPU6502::xxx, addr_mode: CPU6502::imp, cycles: 2 }, Instruction { name: String::from("???"), op: CPU6502::xxx, addr_mode: CPU6502::imp, cycles: 6 }, Instruction { name: String::from("STY"), op: CPU6502::sty, addr_mode: CPU6502::zpx, cycles: 4 }, Instruction { name: String::from("STA"), op: CPU6502::sta, addr_mode: CPU6502::zpx, cycles: 4 }, Instruction { name: String::from("STX"), op: CPU6502::stx, addr_mode: CPU6502::zpy, cycles: 4 }, Instruction { name: String::from("???"), op: CPU6502::xxx, addr_mode: CPU6502::imp, cycles: 4 }, Instruction { name: String::from("TYA"), op: CPU6502::tya, addr_mode: CPU6502::imp, cycles: 2 }, Instruction { name: String::from("STA"), op: CPU6502::sta, addr_mode: CPU6502::aby, cycles: 5 }, Instruction { name: String::from("TXS"), op: CPU6502::txs, addr_mode: CPU6502::imp, cycles: 2 }, Instruction { name: String::from("???"), op: CPU6502::xxx, addr_mode: CPU6502::imp, cycles: 5 }, Instruction { name: String::from("???"), op: CPU6502::nop, addr_mode: CPU6502::imp, cycles: 5 }, Instruction { name: String::from("STA"), op: CPU6502::sta, addr_mode: CPU6502::abx, cycles: 5 }, Instruction { name: String::from("???"), op: CPU6502::xxx, addr_mode: CPU6502::imp, cycles: 5 }, Instruction { name: String::from("???"), op: CPU6502::xxx, addr_mode: CPU6502::imp, cycles: 5 },
                Instruction { name: String::from("LDY"), op: CPU6502::ldy, addr_mode: CPU6502::imm, cycles: 2 }, Instruction { name: String::from("LDA"), op: CPU6502::lda, addr_mode: CPU6502::izx, cycles: 6 }, Instruction { name: String::from("LDX"), op: CPU6502::ldx, addr_mode: CPU6502::imm, cycles: 2 }, Instruction { name: String::from("???"), op: CPU6502::xxx, addr_mode: CPU6502::imp, cycles: 6 }, Instruction { name: String::from("LDY"), op: CPU6502::ldy, addr_mode: CPU6502::zp0, cycles: 3 }, Instruction { name: String::from("LDA"), op: CPU6502::lda, addr_mode: CPU6502::zp0, cycles: 3 }, Instruction { name: String::from("LDX"), op: CPU6502::ldx, addr_mode: CPU6502::zp0, cycles: 3 }, Instruction { name: String::from("???"), op: CPU6502::xxx, addr_mode: CPU6502::imp, cycles: 3 }, Instruction { name: String::from("TAY"), op: CPU6502::tay, addr_mode: CPU6502::imp, cycles: 2 }, Instruction { name: String::from("LDA"), op: CPU6502::lda, addr_mode: CPU6502::imm, cycles: 2 }, Instruction { name: String::from("TAX"), op: CPU6502::tax, addr_mode: CPU6502::imp, cycles: 2 }, Instruction { name: String::from("???"), op: CPU6502::xxx, addr_mode: CPU6502::imp, cycles: 2 }, Instruction { name: String::from("LDY"), op: CPU6502::ldy, addr_mode: CPU6502::abs, cycles: 4 }, Instruction { name: String::from("LDA"), op: CPU6502::lda, addr_mode: CPU6502::abs, cycles: 4 }, Instruction { name: String::from("LDX"), op: CPU6502::ldx, addr_mode: CPU6502::abs, cycles: 4 }, Instruction { name: String::from("???"), op: CPU6502::xxx, addr_mode: CPU6502::imp, cycles: 4 },
                Instruction { name: String::from("BCS"), op: CPU6502::bcs, addr_mode: CPU6502::rel, cycles: 2 }, Instruction { name: String::from("LDA"), op: CPU6502::lda, addr_mode: CPU6502::izy, cycles: 5 }, Instruction { name: String::from("???"), op: CPU6502::xxx, addr_mode: CPU6502::imp, cycles: 2 }, Instruction { name: String::from("???"), op: CPU6502::xxx, addr_mode: CPU6502::imp, cycles: 5 }, Instruction { name: String::from("LDY"), op: CPU6502::ldy, addr_mode: CPU6502::zpx, cycles: 4 }, Instruction { name: String::from("LDA"), op: CPU6502::lda, addr_mode: CPU6502::zpx, cycles: 4 }, Instruction { name: String::from("LDX"), op: CPU6502::ldx, addr_mode: CPU6502::zpy, cycles: 4 }, Instruction { name: String::from("???"), op: CPU6502::xxx, addr_mode: CPU6502::imp, cycles: 4 }, Instruction { name: String::from("CLV"), op: CPU6502::clv, addr_mode: CPU6502::imp, cycles: 2 }, Instruction { name: String::from("LDA"), op: CPU6502::lda, addr_mode: CPU6502::aby, cycles: 4 }, Instruction { name: String::from("TSX"), op: CPU6502::tsx, addr_mode: CPU6502::imp, cycles: 2 }, Instruction { name: String::from("???"), op: CPU6502::xxx, addr_mode: CPU6502::imp, cycles: 4 }, Instruction { name: String::from("LDY"), op: CPU6502::ldy, addr_mode: CPU6502::abx, cycles: 4 }, Instruction { name: String::from("LDA"), op: CPU6502::lda, addr_mode: CPU6502::abx, cycles: 4 }, Instruction { name: String::from("LDX"), op: CPU6502::ldx, addr_mode: CPU6502::aby, cycles: 4 }, Instruction { name: String::from("???"), op: CPU6502::xxx, addr_mode: CPU6502::imp, cycles: 4 },
                Instruction { name: String::from("CPY"), op: CPU6502::cpy, addr_mode: CPU6502::imm, cycles: 2 }, Instruction { name: String::from("CMP"), op: CPU6502::cmp, addr_mode: CPU6502::izx, cycles: 6 }, Instruction { name: String::from("???"), op: CPU6502::nop, addr_mode: CPU6502::imp, cycles: 2 }, Instruction { name: String::from("???"), op: CPU6502::xxx, addr_mode: CPU6502::imp, cycles: 8 }, Instruction { name: String::from("CPY"), op: CPU6502::cpy, addr_mode: CPU6502::zp0, cycles: 3 }, Instruction { name: String::from("CMP"), op: CPU6502::cmp, addr_mode: CPU6502::zp0, cycles: 3 }, Instruction { name: String::from("DEC"), op: CPU6502::dec, addr_mode: CPU6502::zp0, cycles: 5 }, Instruction { name: String::from("???"), op: CPU6502::xxx, addr_mode: CPU6502::imp, cycles: 5 }, Instruction { name: String::from("INY"), op: CPU6502::iny, addr_mode: CPU6502::imp, cycles: 2 }, Instruction { name: String::from("CMP"), op: CPU6502::cmp, addr_mode: CPU6502::imm, cycles: 2 }, Instruction { name: String::from("DEX"), op: CPU6502::dex, addr_mode: CPU6502::imp, cycles: 2 }, Instruction { name: String::from("???"), op: CPU6502::xxx, addr_mode: CPU6502::imp, cycles: 2 }, Instruction { name: String::from("CPY"), op: CPU6502::cpy, addr_mode: CPU6502::abs, cycles: 4 }, Instruction { name: String::from("CMP"), op: CPU6502::cmp, addr_mode: CPU6502::abs, cycles: 4 }, Instruction { name: String::from("DEC"), op: CPU6502::dec, addr_mode: CPU6502::abs, cycles: 6 }, Instruction { name: String::from("???"), op: CPU6502::xxx, addr_mode: CPU6502::imp, cycles: 6 },
                Instruction { name: String::from("BNE"), op: CPU6502::bne, addr_mode: CPU6502::rel, cycles: 2 }, Instruction { name: String::from("CMP"), op: CPU6502::cmp, addr_mode: CPU6502::izy, cycles: 5 }, Instruction { name: String::from("???"), op: CPU6502::xxx, addr_mode: CPU6502::imp, cycles: 2 }, Instruction { name: String::from("???"), op: CPU6502::xxx, addr_mode: CPU6502::imp, cycles: 8 }, Instruction { name: String::from("???"), op: CPU6502::nop, addr_mode: CPU6502::imp, cycles: 4 }, Instruction { name: String::from("CMP"), op: CPU6502::cmp, addr_mode: CPU6502::zpx, cycles: 4 }, Instruction { name: String::from("DEC"), op: CPU6502::dec, addr_mode: CPU6502::zpx, cycles: 6 }, Instruction { name: String::from("???"), op: CPU6502::xxx, addr_mode: CPU6502::imp, cycles: 6 }, Instruction { name: String::from("CLD"), op: CPU6502::cld, addr_mode: CPU6502::imp, cycles: 2 }, Instruction { name: String::from("CMP"), op: CPU6502::cmp, addr_mode: CPU6502::aby, cycles: 4 }, Instruction { name: String::from("NOP"), op: CPU6502::nop, addr_mode: CPU6502::imp, cycles: 2 }, Instruction { name: String::from("???"), op: CPU6502::xxx, addr_mode: CPU6502::imp, cycles: 7 }, Instruction { name: String::from("???"), op: CPU6502::nop, addr_mode: CPU6502::imp, cycles: 4 }, Instruction { name: String::from("CMP"), op: CPU6502::cmp, addr_mode: CPU6502::abx, cycles: 4 }, Instruction { name: String::from("DEC"), op: CPU6502::dec, addr_mode: CPU6502::abx, cycles: 7 }, Instruction { name: String::from("???"), op: CPU6502::xxx, addr_mode: CPU6502::imp, cycles: 7 },
                Instruction { name: String::from("CPX"), op: CPU6502::cpx, addr_mode: CPU6502::imm, cycles: 2 }, Instruction { name: String::from("SBC"), op: CPU6502::sbc, addr_mode: CPU6502::izx, cycles: 6 }, Instruction { name: String::from("???"), op: CPU6502::nop, addr_mode: CPU6502::imp, cycles: 2 }, Instruction { name: String::from("???"), op: CPU6502::xxx, addr_mode: CPU6502::imp, cycles: 8 }, Instruction { name: String::from("CPX"), op: CPU6502::cpx, addr_mode: CPU6502::zp0, cycles: 3 }, Instruction { name: String::from("SBC"), op: CPU6502::sbc, addr_mode: CPU6502::zp0, cycles: 3 }, Instruction { name: String::from("INC"), op: CPU6502::inc, addr_mode: CPU6502::zp0, cycles: 5 }, Instruction { name: String::from("???"), op: CPU6502::xxx, addr_mode: CPU6502::imp, cycles: 5 }, Instruction { name: String::from("INX"), op: CPU6502::inx, addr_mode: CPU6502::imp, cycles: 2 }, Instruction { name: String::from("SBC"), op: CPU6502::sbc, addr_mode: CPU6502::imm, cycles: 2 }, Instruction { name: String::from("NOP"), op: CPU6502::nop, addr_mode: CPU6502::imp, cycles: 2 }, Instruction { name: String::from("???"), op: CPU6502::sbc, addr_mode: CPU6502::imp, cycles: 2 }, Instruction { name: String::from("CPX"), op: CPU6502::cpx, addr_mode: CPU6502::abs, cycles: 4 }, Instruction { name: String::from("SBC"), op: CPU6502::sbc, addr_mode: CPU6502::abs, cycles: 4 }, Instruction { name: String::from("INC"), op: CPU6502::inc, addr_mode: CPU6502::abs, cycles: 6 }, Instruction { name: String::from("???"), op: CPU6502::xxx, addr_mode: CPU6502::imp, cycles: 6 },
                Instruction { name: String::from("BEQ"), op: CPU6502::beq, addr_mode: CPU6502::rel, cycles: 2 }, Instruction { name: String::from("SBC"), op: CPU6502::sbc, addr_mode: CPU6502::izy, cycles: 5 }, Instruction { name: String::from("???"), op: CPU6502::xxx, addr_mode: CPU6502::imp, cycles: 2 }, Instruction { name: String::from("???"), op: CPU6502::xxx, addr_mode: CPU6502::imp, cycles: 8 }, Instruction { name: String::from("???"), op: CPU6502::nop, addr_mode: CPU6502::imp, cycles: 4 }, Instruction { name: String::from("SBC"), op: CPU6502::sbc, addr_mode: CPU6502::zpx, cycles: 4 }, Instruction { name: String::from("INC"), op: CPU6502::inc, addr_mode: CPU6502::zpx, cycles: 6 }, Instruction { name: String::from("???"), op: CPU6502::xxx, addr_mode: CPU6502::imp, cycles: 6 }, Instruction { name: String::from("SED"), op: CPU6502::sed, addr_mode: CPU6502::imp, cycles: 2 }, Instruction { name: String::from("SBC"), op: CPU6502::sbc, addr_mode: CPU6502::aby, cycles: 4 }, Instruction { name: String::from("NOP"), op: CPU6502::nop, addr_mode: CPU6502::imp, cycles: 2 }, Instruction { name: String::from("???"), op: CPU6502::xxx, addr_mode: CPU6502::imp, cycles: 7 }, Instruction { name: String::from("???"), op: CPU6502::nop, addr_mode: CPU6502::imp, cycles: 4 }, Instruction { name: String::from("SBC"), op: CPU6502::sbc, addr_mode: CPU6502::abx, cycles: 4 }, Instruction { name: String::from("INC"), op: CPU6502::inc, addr_mode: CPU6502::abx, cycles: 7 }, Instruction { name: String::from("???"), op: CPU6502::xxx, addr_mode: CPU6502::imp, cycles: 7 },
            ]
        };

        return cpu;
    }

    pub fn set_bus(&mut self, bus: Option<Rc<RefCell<dyn ReadWrite>>>)
    {
        self.bus = bus;
    }

    pub fn get_flag(&self, f: Flags6502) -> u8
    {
        return self.status & (f as u8);
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

    pub fn get_pc(&self) -> u16
    {
        return self.pc;
    }

    pub fn get_a(&self) -> u8
    {
        return self.a;
    }

    pub fn get_x(&self) -> u8
    {
        return self.x;
    }

    pub fn get_y(&self) -> u8
    {
        return self.y;
    }

    pub fn get_stkp(&self) -> u8
    {
        return self.stkp;
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

    pub fn ind(&mut self) -> u8
    {
        let ptr_lo: u16 = self.read(self.pc) as u16;
        self.pc += 1;
        let ptr_hi: u16 = self.read(self.pc) as u16;
        self.pc += 1;

        let ptr: u16 = (ptr_hi << 8) | ptr_lo;

        if ptr_lo == 0x00FF
        {
            self.addr_abs = (((self.read(ptr & 0xFF00 as u16) as u16) << 8) | self.read(ptr) as u16) as u16;
        }
        else
        {
            self.addr_abs = (((self.read(ptr + 1) as u16) << 8) | self.read(ptr + 0) as u16) as u16;
        }

        return 0;
    }

    // The read 8-bit address is offset by X and used to read the 16-bit absolute address
    pub fn izx(&mut self) -> u8
    {
        let t : u16 = self.read(self.pc) as u16;
	    self.pc += 1;

        let lo: u16 = self.read((t + self.x as u16) & 0x00FF) as u16;
        let hi: u16 = self.read((t + self.x as u16 + 1) & 0x00FF) as u16;

        self.addr_abs = (hi << 8) | lo;

        return 0;
    }

    // The read 8-bit address is read from the zero page as a 16-bit address, then offset by Y
    // If the offset changes a page, we need 1 additional clock cycle
    pub fn izy(&mut self) -> u8
    {
        let t : u16 = self.read(self.pc) as u16;
        self.pc += 1;

        let lo: u16 = self.read(t & 0x00FF) as u16;
        let hi: u16 = self.read((t + 1) & 0x00FF) as u16;

        self.addr_abs = (hi << 8) | lo;
        self.addr_abs += self.y as u16;

        if (self.addr_abs & 0xFF00) != (hi << 8)
        {
            return 1;
        }
        else
        {
            return 0;
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

        return 1;
    }

    // Instruction: Bitwise Logic AND
    // A = A & M
    pub fn and(&mut self) -> u8
    {
        self.fetch();
        self.a = self.a & self.fetched_data;
        self.set_flag(Flags6502::Z, self.a == 0x00); // Zero
        self.set_flag(Flags6502::N, self.a & 0x80 == 0x80); // Negative
        return 1;
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
        return 0;
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
        return 0;
    }

    // Instruction: Branch if Carry Set
    // if (C == 1) then pc = address
    pub fn bcs(&mut self) -> u8
    {
        if self.get_flag(Flags6502::C) == 1
        {
            self.branch();
        }
        return 0;
    }

    // Instruction: Branch if Equal
    // if (Z == 1) then pc = address
    pub fn beq(&mut self) -> u8
    {
        if self.get_flag(Flags6502::Z) == 1
        {
            self.branch();
        }
        return 0;
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

        return 0;
    }

    // Instruction: Branch if Negative
    // if (N == 1) then pc = address
    pub fn bmi(&mut self) -> u8
    {
        if self.get_flag(Flags6502::N) == 1
        {
            self.branch();
        }
        return 0;
    }

    // Instruction: Branch if not equal
    // if (Z == 0) then pc = address
    pub fn bne(&mut self) -> u8
    {
        if self.get_flag(Flags6502::Z) == 0
        {
            self.branch();
        }
        return 0;
    }

    // Instruction: Branch if Positive
    // if (N == 0) then pc = address
    pub fn bpl(&mut self) -> u8
    {
        if self.get_flag(Flags6502::N) == 0
        {
            self.branch();
        }
        return 0;
    }

    fn write_pc_to_stack(&mut self)
    {
        self.write(CPU6502::STACK_START_ADDRESS + self.stkp as u16, ((self.pc >> 8) & 0x00FF) as u8);
        self.stkp -= 1;
        self.write(CPU6502::STACK_START_ADDRESS + self.stkp as u16, (self.pc & 0x00FF) as u8);
        self.stkp -= 1;
    }

    fn read_pc_from_stack(&mut self)
    {
        self.stkp += 1;
        self.pc = self.read(CPU6502::STACK_START_ADDRESS + self.stkp as u16) as u16;
        self.stkp += 1;
        self.pc |= (self.read(CPU6502::STACK_START_ADDRESS + self.stkp as u16) as u16) << 8;
    }

    // Instruction: Break
    // Function: Program Sourced Interrupt
    pub fn brk(&mut self) -> u8
    {
        self.pc += 1;

        self.set_flag(Flags6502::I, true);
        self.write_pc_to_stack();

        // TODO: Not clear when exactly the B-flag should be set based on the documentation, needs investigation
        self.set_flag(Flags6502::B, true);
        self.write(CPU6502::STACK_START_ADDRESS + self.stkp as u16, self.status);
        self.stkp -= 1;
        self.set_flag(Flags6502::B, false);

        // TODO: Not clear if we should vary behavior if an NMI happens while BRK is ongoing
        // Go to the interrupt vector
        self.pc = self.read(CPU6502::INTERRUPT_VECTOR[1]) as u16 |
                    ((self.read(CPU6502::INTERRUPT_VECTOR[0]) as u16) << 8);

        return 0;
    }

    // Instruction: Branch if Overflow Clear
    // Function: if (V == 0) pc = address
    pub fn bvc(&mut self) -> u8
    {
        if self.get_flag(Flags6502::V) == 0
        {
            self.branch();
        }
        return 0;
    }

    // Instruction: Branch if Overflow Set
    // Function: if (V == 1) pc = address
    pub fn bvs(&mut self) -> u8
    {
        if self.get_flag(Flags6502::V) == 1
        {
            self.branch();
        }
        return 0;
    }

    // Instruction: Clear Carry Flag
    // Function: C = 0
    pub fn clc(&mut self) -> u8 
    {
        self.set_flag(Flags6502::C, false);
        return 0;
    }

    // Instruction: Clear Decimal Flag
    // Function: D = 0
    pub fn cld(&mut self) -> u8 
    {
        self.set_flag(Flags6502::D, false);
        return 0;
    }

    // Instruction: Disable Interrupts / Clear Interrupt Flag
    // Function: I = 0
    pub fn cli(&mut self) -> u8 
    {
        self.set_flag(Flags6502::I, false);
        return 0;
    }

    // Instruction: Clear Overflow Flag
    // Function: V = 0
    pub fn clv(&mut self) -> u8 
    {
        self.set_flag(Flags6502::V, false);
        return 0;
    }

    fn cmp_helper<F>(&mut self, closure: F) where F: Fn(&CPU6502) -> u8
    {
        self.fetch();
        let temp: u16 = closure(self) as u16 - self.fetched_data as u16;
        self.set_flag(Flags6502::C, self.a >= self.fetched_data);
        self.set_flag(Flags6502::Z, (temp & 0x00FF) == 0x0000);
        self.set_flag(Flags6502::N, temp & 0x0080 == 0x0080);
    }

    pub fn cmp(&mut self) -> u8 
    {
        self.cmp_helper(|s| -> u8 { s.a });
        return 1;
    }

    pub fn cpx(&mut self) -> u8 
    {
        self.cmp_helper(|s| -> u8 { s.x });
        return 0;
    }

    pub fn cpy(&mut self) -> u8 
    {
        self.cmp_helper(|s| -> u8 { s.y });
        return 0;
    }

    pub fn dec(&mut self) -> u8 
    {
        self.fetch();
        let temp: u16 = self.fetched_data as u16 - 1;
        self.write(self.addr_abs, (temp & 0x00FF) as u8);
        self.set_flag(Flags6502::Z, (temp & 0x00FF) == 0x0000);
        self.set_flag(Flags6502::N, (temp & 0x0080) == 0x0080);

        return 0;
    }

    // Instruction: Decrement X Register
    // Function: X = X - 1
    pub fn dex(&mut self) -> u8 
    {
        self.x -= 1;
        self.set_flag(Flags6502::Z, self.x == 0x00);
        self.set_flag(Flags6502::N, (self.x & 0x80) == 0x80);
        return 0;
    }

    // Instruction: Decrement Y Register
    // Function: Y = Y - 1
    pub fn dey(&mut self) -> u8 
    {
        self.y -= 1;
        self.set_flag(Flags6502::Z, self.y == 0x00);
        self.set_flag(Flags6502::N, (self.y & 0x80) == 0x80);
        return 0;
    }

    // Instruction: Bitwise Logic XOR
    // Function: A = A xor M
    pub fn eor(&mut self) -> u8 
    {
        self.fetch();
        self.a = self.a ^ self.fetched_data;
        self.set_flag(Flags6502::Z, self.a == 0x00);
        self.set_flag(Flags6502::N, (self.a & 0x80) == 0x80);
        return 0;
    }

    // Instruction: Increment Value at Memory Location
    // Function: M = M + 1
    pub fn inc(&mut self) -> u8 
    {
        self.fetch();
        let temp: u16 = self.fetched_data as u16 + 1;
        self.write(self.addr_abs, (temp & 0x00FF) as u8);
        self.set_flag(Flags6502::Z, (temp & 0x00FF) == 0x0000);
        self.set_flag(Flags6502::N, (temp & 0x0080) == 0x0080);
        return 0;
    }

    // Instruction: Increment X Register
    // Function: X = X + 1
    pub fn inx(&mut self) -> u8 
    {
        self.x += 1;
        self.set_flag(Flags6502::Z, self.x == 0x00);
        self.set_flag(Flags6502::N, (self.x & 0x80) == 0x80);
        return 0;
    }

    // Instruction: Increment Y Register
    // Function: Y = Y + 1
    pub fn iny(&mut self) -> u8 
    {
        self.y += 1;
        self.set_flag(Flags6502::Z, self.y == 0x00);
        self.set_flag(Flags6502::N, (self.y & 0x80) == 0x80);
        return 0;
    }

    // Instruction: Jump To Location
    // Function: pc = address
    pub fn jmp(&mut self) -> u8 
    {
        self.pc = self.addr_abs;
        return 0;
    }

    // Instruction: Jump To Sub-Routine
    // Function:    Push current pc to stack, pc = address
    pub fn jsr(&mut self) -> u8 
    {
        // Decrement PC to get back to current program counter
        self.pc -= 1;
        self.write_pc_to_stack();
        self.pc = self.addr_abs;
        return 0;
    }

    fn load_helper_write<F>(&mut self, write_closure: F) where F: Fn(&mut CPU6502)
    {
        self.fetch();
        write_closure(self);
    }

    fn load_helper_update_flags<F>(&mut self, read_closure: F) where F: Fn(&CPU6502) -> u8
    {
        self.set_flag(Flags6502::Z, read_closure(self) == 0x00);
        self.set_flag(Flags6502::N, (read_closure(self) & 0x80) == 0x80);
    }

    pub fn lda(&mut self) -> u8 
    {
        self.load_helper_write(|x| x.a = x.fetched_data);
        self.load_helper_update_flags(|x| return x.a);
        return 0;
    }

    pub fn ldx(&mut self) -> u8 
    {
        self.load_helper_write(|x| x.x = x.fetched_data);
        self.load_helper_update_flags(|x| return x.x);
        return 0;
    }

    pub fn ldy(&mut self) -> u8 
    {
        self.load_helper_write(|x| x.y = x.fetched_data);
        self.load_helper_update_flags(|x| return x.y);
        return 0;
    }

    pub fn lsr(&mut self) -> u8 
    {
        self.fetch();
        self.set_flag(Flags6502::C, (self.fetched_data & 0x0001) == 0x0001); // Carry
        let temp: u16 = (self.fetched_data as u16) >> 1;
        
        self.set_flag(Flags6502::Z, (temp & 0x00FF) == 0x0000); // Zero
        self.set_flag(Flags6502::N, temp & 0x0080 == 0x0080); // Negative

        if self.ins[self.opcode as usize].addr_mode == CPU6502::imp
        {
            self.a = (temp & 0x00FF) as u8;
        }
        else
        {
            self.write(self.addr_abs, (temp & 0x00FF) as u8);
        }

        return 0;
    }

    // Instruction: No-op
    pub fn nop(&mut self) -> u8 
    {
        // There are different kinds of NOPs depending on the opcode
        match self.opcode
        {
            0x1C => return 1,
            0x3C => return 1,
            0x5C => return 1,
            0x7C => return 1,
            0xDC => return 1,
            0xFC => return 1,
            _ => return 0
        }
    }

    // Instruction: Bitwise Logic OR
    // Function: A = A | M
    pub fn ora(&mut self) -> u8 
    {
        self.fetch();
        self.a = self.a | self.fetched_data;

        self.set_flag(Flags6502::Z, self.a == 0x00); // Zero
        self.set_flag(Flags6502::N, self.a & 0x80 == 0x80); // Negative
        return 0;
    }

    // Instruction: Push Accumulator to Stack
    // Function: Write A to stkp
    pub fn pha(&mut self) -> u8 
    {
        self.write(CPU6502::STACK_START_ADDRESS + self.stkp as u16, self.a);
        self.stkp -= 1;
        return 0;
    }
    
    // Instruction: Push Status Register to Stack
    // Function: status -> stack
    pub fn php(&mut self) -> u8 
    {
        // Note that Break and Unused flag are both set to 1 when writing
        self.write(CPU6502::STACK_START_ADDRESS + self.stkp as u16, (self.status) | Flags6502::B as u8 | Flags6502::U as u8);
        self.set_flag(Flags6502::B, false);
        self.set_flag(Flags6502::U, false);
        self.stkp -= 1;
        return 0;
    }

    // Instruction: Pop Accumulator off Stack
    // Function: Set A to stack
    pub fn pla(&mut self) -> u8 
    {
        self.stkp += 1;
        self.a = self.read(CPU6502::STACK_START_ADDRESS + self.stkp as u16);
        self.set_flag(Flags6502::Z, self.a == 0x00); // Zero
        self.set_flag(Flags6502::N, self.a & 0x80 == 0x80); // Negative
        return 0;
    }

    // Instruction: Pop Status Register off Stack
    // Function: Status <- stack
    pub fn plp(&mut self) -> u8 
    {
        self.stkp += 1;
        self.status = self.read(CPU6502::STACK_START_ADDRESS + self.stkp as u16);
        self.set_flag(Flags6502::U, true);
        return 0;
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
        if self.ins[self.opcode as usize].addr_mode == CPU6502::imp
        {
            self.a = (temp & 0x00FF) as u8;
        }
        else
        {
            self.write(self.addr_abs, (temp & 0x00FF) as u8);
        }

        return 0;
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
        if self.ins[self.opcode as usize].addr_mode == CPU6502::imp
        {
            self.a = (temp & 0x00FF) as u8;
        }
        else
        {
            self.write(self.addr_abs, (temp & 0x00FF) as u8);
        }

        return 0;
    }

    // Instruction: Return from Interrupt
    // Function: Read status and pc from stkp
    pub fn rti(&mut self) -> u8 
    {
        self.stkp += 1;
        self.status = self.read(CPU6502::STACK_START_ADDRESS + self.stkp as u16);

        // Zero out break and unused values, TODO: Why?
        self.status &= !(Flags6502::B as u8);
        self.status &= !(Flags6502::U as u8);

        self.read_pc_from_stack();
        return 0;
    }

    // Instruction: Return from subroutine
    // Function: Read pc from stack, increment PC to next instruction
    pub fn rts(&mut self) -> u8 
    {
        self.read_pc_from_stack();
        self.pc += 1;
        return 0;
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

        let v: u16 = (!(self.a as u16 ^ self.fetched_data as u16)) & ((self.a as u16 ^ temp) & 0x0080);
        self.set_flag(Flags6502::V, v != 0);

        self.set_flag(Flags6502::N, temp & 0x80 == 0x80);

        // Load the result into the accumulator
        self.a = (temp & 0x00FF) as u8;

        return 1;
    }

    // Instruction: Set Carry Flag
    // Function: C = 1
    pub fn sec(&mut self) -> u8 
    {
        self.set_flag(Flags6502::C, true);
        return 0;
    }

    // Instruction: Set Decimal Flag
    // Function: D = 1
    pub fn sed(&mut self) -> u8 
    {
        self.set_flag(Flags6502::D, true);
        return 0;
    }

    // Instruction: Set Interrupt Flag / Enable Interrupts
    // Function: I = 1
    pub fn sei(&mut self) -> u8 
    {
        self.set_flag(Flags6502::I, true);
        return 0;
    }

    // Instruction: Store Accumulator at address
    // Function: M = A
    pub fn sta(&mut self) -> u8 
    {
        self.write(self.addr_abs, self.a);
        return 0;
    }

    // Instruction: Store X at address
    // Function: M = X
    pub fn stx(&mut self) -> u8 
    {
        self.write(self.addr_abs, self.x);
        return 0;
    }

    // Instruction: Store Y at address
    // Function: M = Y
    pub fn sty(&mut self) -> u8 
    {
        self.write(self.addr_abs, self.y);
        return 0;
    }

    // Instruction: Transfer Accumulator to X Register
    // Function: X = A
    pub fn tax(&mut self) -> u8 
    {
        self.x = self.a;
        self.set_flag(Flags6502::Z, self.x == 0x00); // Zero
        self.set_flag(Flags6502::N, self.x & 0x80 == 0x80); // Negative
        return 0;
    }

    // Instruction: Transfer Accumulator to Y Register
    // Function: Y = A
    pub fn tay(&mut self) -> u8 
    {
        self.y = self.a;
        self.set_flag(Flags6502::Z, self.y == 0x00); // Zero
        self.set_flag(Flags6502::N, self.y & 0x80 == 0x80); // Negative
        return 0;
    }

    // Instruction: Transfer Stack Pointer to X
    // Function:    X = stkp
    pub fn tsx(&mut self) -> u8 
    {
        self.x = self.stkp;
        self.set_flag(Flags6502::Z, self.x == 0x00); // Zero
        self.set_flag(Flags6502::N, self.x & 0x80 == 0x80); // Negative
        return 0;
    }

    // Instruction: Transfer X to A
    // Function: A = X
    pub fn txa(&mut self) -> u8 
    {
        self.a = self.x;
        self.set_flag(Flags6502::Z, self.a == 0x00); // Zero
        self.set_flag(Flags6502::N, self.a & 0x80 == 0x80); // Negative
        return 0;
    }

    // Instruction: Transfer X to Stack pointer
    // Function: stkp = x
    pub fn txs(&mut self) -> u8 
    {
        self.stkp = self.x;
        return 0;
    }

    // Instruction: Transfer Y to A
    // Function: A = Y
    pub fn tya(&mut self) -> u8 
    {
        self.a = self.y;
        self.set_flag(Flags6502::Z, self.a == 0x00); // Zero
        self.set_flag(Flags6502::N, self.a & 0x80 == 0x80); // Negative
        return 0;
    }

    // This function captures illegal opcodes
    pub fn xxx(&mut self) -> u8 
    {
        return 0;
    }

    // Other signals
    pub fn clock_tick(&mut self)
    {
        if self.cycles == 0
        {
            self.opcode = self.read(self.pc);

            // TODO: Why?
            self.set_flag(Flags6502::U, true);

            self.pc += 1;

            self.cycles = self.ins[self.opcode as usize].cycles;
            let additional_cycle1: u8 = (self.ins[self.opcode as usize].addr_mode)(self);
            let additional_cycle2: u8 = (self.ins[self.opcode as usize].op)(self);

            // TODO: Why is this a binary AND?
            self.cycles += additional_cycle1 & additional_cycle2;

            // TODO: Why?
            self.set_flag(Flags6502::U, true);
        }

         self.cycles -= 1;
    }

    pub fn complete(&self) -> bool
    {
        return self.cycles == 0;
    }

    pub fn disassemble(&self, n_start: u16, n_end: u16) -> BTreeMap<u16, String>
    {
        let mut map = BTreeMap::new();

        let mut addr: u32 = n_start as u32;
        let mut value: u8;
        let mut lo: u8;
        let mut hi: u8;
        let mut line_addr: u16;

        while addr <= n_end as u32
        {
            line_addr = addr as u16;
            let mut instruction: String = format!("${:04x}: ", addr);
            let opcode = match &self.bus
                {
                    Some(x) => (*x.borrow()).read(addr as u16),
                    None => panic!("Error, missing bus inside CPU")
                };

            addr += 1;
            instruction += &self.ins[opcode as usize].name;

            if self.ins[opcode as usize].addr_mode == CPU6502::imp
            {
                instruction += &String::from(" {IMP}");
            }
            else if self.ins[opcode as usize].addr_mode == CPU6502::imm
            {
                value = self.read(addr as u16);
                addr += 1;
                instruction += &String::from(format!(" #${:02x} {{IMM}}", value));
            }
            else if self.ins[opcode as usize].addr_mode == CPU6502::zp0
            {
                lo = self.read(addr as u16);
                addr += 1;
                instruction += &String::from(format!(" ${:02x} {{ZP0}}", lo));
            }
            else if self.ins[opcode as usize].addr_mode == CPU6502::zpx
            {
                lo = self.read(addr as u16);
                addr += 1;
                instruction += &String::from(format!(" ${:02x}, X {{ZPX}}", lo));
            }
            else if self.ins[opcode as usize].addr_mode == CPU6502::zpy
            {
                lo = self.read(addr as u16);
                addr += 1;
                instruction += &String::from(format!(" ${:02x}, Y {{ZPY}}", lo));
            }
            else if self.ins[opcode as usize].addr_mode == CPU6502::izx
            {
                lo = self.read(addr as u16);
                addr += 1;
                instruction += &String::from(format!(" (${:02x}), X {{IZX}}", lo));
            }
            else if self.ins[opcode as usize].addr_mode == CPU6502::izy
            {
                lo = self.read(addr as u16);
                addr += 1;
                instruction += &String::from(format!(" (${:02x}), Y {{IZY}}", lo));
            }
            else if self.ins[opcode as usize].addr_mode == CPU6502::abs
            {
                lo = self.read(addr as u16);
                addr += 1;
                hi = self.read(addr as u16);
                addr += 1;
                let cur_addr = ((hi as u16) << 8) | lo as u16;
                instruction += &String::from(format!(" ${:04x} {{ABS}}", cur_addr));
            }
            else if self.ins[opcode as usize].addr_mode == CPU6502::abx
            {
                lo = self.read(addr as u16);
                addr += 1;
                hi = self.read(addr as u16);
                addr += 1;
                let cur_addr = ((hi as u16) << 8) | lo as u16;
                instruction += &String::from(format!(" ${:04x}, X {{ABX}}", cur_addr));
            }
            else if self.ins[opcode as usize].addr_mode == CPU6502::aby
            {
                lo = self.read(addr as u16);
                addr += 1;
                hi = self.read(addr as u16);
                addr += 1;
                let cur_addr = ((hi as u16) << 8) | lo as u16;
                instruction += &String::from(format!(" ${:04x}, Y {{ABY}}", cur_addr));
            }
            else if self.ins[opcode as usize].addr_mode == CPU6502::ind
            {
                lo = self.read(addr as u16);
                addr += 1;
                hi = self.read(addr as u16);
                addr += 1;
                let cur_addr = ((hi as u16) << 8) | lo as u16;
                instruction += &String::from(format!(" (${:04x}) {{IND}}", cur_addr));
            }
            else if self.ins[opcode as usize].addr_mode == CPU6502::rel
            {
                value = self.read(addr as u16);
                addr += 1;
                instruction += &String::from(format!(" ${:02x} [${:04x}] {{REL}}", value, addr as u16 + value as u16));
            }

            map.insert(line_addr, instruction);
        }

        return map;
    }

    pub fn reset(&mut self)
    {
        self.addr_abs = 0xFFFC;
        let lo: u8 = self.read(self.addr_abs + 0);
        let hi: u8 = self.read(self.addr_abs + 1);
    
        self.pc = ((hi as u16) << 8) | lo as u16;

        self.a = 0;
        self.x = 0;
        self.y = 0;
        self.stkp = 0xFD;

        self.status = Flags6502::U as u8;

        self.addr_rel = 0x0000;
        self.addr_abs = 0x0000;
        self.fetched_data = 0x00;

        self.cycles = 8;
    }

    fn perform_irq(&mut self, pc_read_addr: u16, num_cycles: u8)
    {
        // Save the PC to the stack
        self.write_pc_to_stack();

        // Save the status register to the stack
        self.set_flag(Flags6502::B, false);
        self.set_flag(Flags6502::U, true);
        self.set_flag(Flags6502::I, true);
        self.write(CPU6502::STACK_START_ADDRESS + self.stkp as u16, self.status);
        self.stkp -= 1;
        
        // Read the new program counter location from a fixed address
        self.addr_abs = pc_read_addr;
        let lo: u16 = self.read(self.addr_abs + 0) as u16;
        let hi: u16 = self.read(self.addr_abs + 1) as u16;

        self.pc = (hi << 8) | lo;

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
        if self.ins[self.opcode as usize].addr_mode != CPU6502::imp
        {
            self.fetched_data = self.read(self.addr_abs);
        }

        return self.fetched_data;
    }

}

impl<'a> ReadWrite for CPU6502<'a>
{
    fn write(&mut self, address: u16, data: u8)
    {
        match &self.bus
        {
            Some(x) => (*x.borrow_mut()).write(address, data),
            None => panic!("Error, missing bus inside CPU")
        }
    }

    fn read(&self, address: u16) -> u8
    {
        match &self.bus
        {
            Some(x) => (*x.borrow_mut()).read(address),
            None => panic!("Error, missing bus inside CPU")
        }
    }
}