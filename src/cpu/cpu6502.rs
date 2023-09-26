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

    pub fn new(bus: &'a mut dyn ReadWrite) -> Self
    {
        let cpu = CPU6502
        {
            bus: bus,
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

    pub fn ind(&mut self) -> u8
    {
        let ptr_lo: u16 = self.read(self.pc) as u16;
        self.pc += 1;
        let ptr_hi: u16 = self.read(self.pc) as u16;
        self.pc += 1;

        let ptr: u16 = (ptr_hi << 8) | ptr_lo;

        if ptr_lo == 0x00FF
        {
            self.addr_abs = ((self.read(ptr & 0xFF00 as u16) << 8) | self.read(ptr)) as u16;
        }
        else
        {
            self.addr_abs = ((self.read(ptr + 1) << 8) | self.read(ptr + 0)) as u16;
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
    pub fn adc(&mut self) -> u8 
    {
        return 0;
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
        self.addr_abs = self.pc + self.addr_rel;

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

    // Instruction: Break
    // Function: Program Sourced Interrupt
    pub fn brk(&mut self) -> u8
    {
        self.pc += 1;

        self.set_flag(Flags6502::I, true);
        self.write(CPU6502::STACK_START_ADDRESS + self.stkp as u16, ((self.pc >> 8) & 0x00FF) as u8);
        self.stkp -= 1;
        self.write(CPU6502::STACK_START_ADDRESS + self.stkp as u16, (self.pc & 0x00FF) as u8);
        self.stkp -= 1;

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


    pub fn cmp(&mut self) -> u8 
    {
        return 0;
    }
    pub fn cpx(&mut self) -> u8 
    {
        return 0;
    }
    pub fn cpy(&mut self) -> u8 
    {
        return 0;
    }
    pub fn dec(&mut self) -> u8 
    {
        return 0;
    }
    pub fn dex(&mut self) -> u8 
    {
        return 0;
    }
    pub fn dey(&mut self) -> u8 
    {
        return 0;
    }
    pub fn eor(&mut self) -> u8 
    {
        return 0;
    }
    pub fn inc(&mut self) -> u8 
    {
        return 0;
    }
    pub fn inx(&mut self) -> u8 
    {
        return 0;
    }
    pub fn iny(&mut self) -> u8 
    {
        return 0;
    }
    pub fn jmp(&mut self) -> u8 
    {
        return 0;
    }
    pub fn jsr(&mut self) -> u8 
    {
        return 0;
    }
    pub fn lda(&mut self) -> u8 
    {
        return 0;
    }
    pub fn ldx(&mut self) -> u8 
    {
        return 0;
    }
    pub fn ldy(&mut self) -> u8 
    {
        return 0;
    }
    pub fn lsr(&mut self) -> u8 
    {
        return 0;
    }
    pub fn nop(&mut self) -> u8 
    {
        return 0;
    }
    pub fn ora(&mut self) -> u8 
    {
        return 0;
    }
    pub fn pha(&mut self) -> u8 
    {
        return 0;
    }
    pub fn php(&mut self) -> u8 
    {
        return 0;
    }
    pub fn pla(&mut self) -> u8 
    {
        return 0;
    }
    pub fn plp(&mut self) -> u8 
    {
        return 0;
    }
    pub fn rol(&mut self) -> u8 
    {
        return 0;
    }
    pub fn ror(&mut self) -> u8 
    {
        return 0;
    }
    pub fn rti(&mut self) -> u8 
    {
        return 0;
    }
    pub fn rts(&mut self) -> u8 
    {
        return 0;
    }
    pub fn sbc(&mut self) -> u8 
    {
        return 0;
    }
    pub fn sec(&mut self) -> u8 
    {
        return 0;
    }
    pub fn sed(&mut self) -> u8 
    {
        return 0;
    }
    pub fn sei(&mut self) -> u8 
    {
        return 0;
    }
    pub fn sta(&mut self) -> u8 
    {
        return 0;
    }
    pub fn stx(&mut self) -> u8 
    {
        return 0;
    }
    pub fn sty(&mut self) -> u8 
    {
        return 0;
    }
    pub fn tax(&mut self) -> u8 
    {
        return 0;
    }
    pub fn tay(&mut self) -> u8 
    {
        return 0;
    }
    pub fn tsx(&mut self) -> u8 
    {
        return 0;
    }
    pub fn txa(&mut self) -> u8 
    {
        return 0;
    }
    pub fn txs(&mut self) -> u8 
    {
        return 0;
    }
    pub fn tya(&mut self) -> u8 
    {
        return 0;
    }

    
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
        self.bus.write(address, data);
    }

    fn read(&self, address: u16) -> u8
    {
        return self.bus.read(address);
    }
}