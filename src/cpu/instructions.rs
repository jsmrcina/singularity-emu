#[macro_export]
macro_rules! create_instruction_array {
    () => {
        [
        // 00 - 0F
        Instruction { name: String::from("BRK"), op: Cpu6502::brk, addr_mode: Cpu6502::imm, cycles: 7 },
        Instruction { name: String::from("ORA"), op: Cpu6502::ora, addr_mode: Cpu6502::izx, cycles: 6 },
        Instruction { name: String::from("KIL"), op: Cpu6502::kil, addr_mode: Cpu6502::imp, cycles: 2 },
        Instruction { name: String::from("*SLO"), op: Cpu6502::slo, addr_mode: Cpu6502::izx, cycles: 8 },
        Instruction { name: String::from("*NOP"), op: Cpu6502::nop, addr_mode: Cpu6502::zp0, cycles: 3 },
        Instruction { name: String::from("ORA"), op: Cpu6502::ora, addr_mode: Cpu6502::zp0, cycles: 3 },
        Instruction { name: String::from("ASL"), op: Cpu6502::asl, addr_mode: Cpu6502::zp0, cycles: 5 },
        Instruction { name: String::from("*SLO"), op: Cpu6502::slo, addr_mode: Cpu6502::zp0, cycles: 5 },
        Instruction { name: String::from("PHP"), op: Cpu6502::php, addr_mode: Cpu6502::imp, cycles: 3 },
        Instruction { name: String::from("ORA"), op: Cpu6502::ora, addr_mode: Cpu6502::imm, cycles: 2 },
        Instruction { name: String::from("ASL A"), op: Cpu6502::asl, addr_mode: Cpu6502::imp, cycles: 2 },
        Instruction { name: String::from("*ANC"), op: Cpu6502::anc, addr_mode: Cpu6502::imm, cycles: 2 },
        Instruction { name: String::from("*NOP"), op: Cpu6502::nop, addr_mode: Cpu6502::abs, cycles: 4 },
        Instruction { name: String::from("ORA"), op: Cpu6502::ora, addr_mode: Cpu6502::abs, cycles: 4 },
        Instruction { name: String::from("ASL"), op: Cpu6502::asl, addr_mode: Cpu6502::abs, cycles: 6 },
        Instruction { name: String::from("*SLO"), op: Cpu6502::slo, addr_mode: Cpu6502::abs, cycles: 6 },

        // 10 - 1F
        Instruction { name: String::from("BPL"), op: Cpu6502::bpl, addr_mode: Cpu6502::rel, cycles: 2 },
        Instruction { name: String::from("ORA"), op: Cpu6502::ora, addr_mode: Cpu6502::izy, cycles: 5 },
        Instruction { name: String::from("KIL"), op: Cpu6502::kil, addr_mode: Cpu6502::imp, cycles: 2 },
        Instruction { name: String::from("*SLO"), op: Cpu6502::slo, addr_mode: Cpu6502::izy, cycles: 8 },
        Instruction { name: String::from("*NOP"), op: Cpu6502::nop, addr_mode: Cpu6502::zpx, cycles: 4 },
        Instruction { name: String::from("ORA"), op: Cpu6502::ora, addr_mode: Cpu6502::zpx, cycles: 4 },
        Instruction { name: String::from("ASL"), op: Cpu6502::asl, addr_mode: Cpu6502::zpx, cycles: 6 },
        Instruction { name: String::from("*SLO"), op: Cpu6502::slo, addr_mode: Cpu6502::zpx, cycles: 6 },
        Instruction { name: String::from("CLC"), op: Cpu6502::clc, addr_mode: Cpu6502::imp, cycles: 2 },
        Instruction { name: String::from("ORA"), op: Cpu6502::ora, addr_mode: Cpu6502::aby, cycles: 4 },
        Instruction { name: String::from("*NOP"), op: Cpu6502::nop, addr_mode: Cpu6502::imp, cycles: 2 },
        Instruction { name: String::from("*SLO"), op: Cpu6502::slo, addr_mode: Cpu6502::aby, cycles: 7 },
        Instruction { name: String::from("*NOP"), op: Cpu6502::nop, addr_mode: Cpu6502::abx, cycles: 4 },
        Instruction { name: String::from("ORA"), op: Cpu6502::ora, addr_mode: Cpu6502::abx, cycles: 4 },
        Instruction { name: String::from("ASL"), op: Cpu6502::asl, addr_mode: Cpu6502::abx, cycles: 7 },
        Instruction { name: String::from("*SLO"), op: Cpu6502::slo, addr_mode: Cpu6502::abx, cycles: 7 },

        // 20 - 2F
        Instruction { name: String::from("JSR"), op: Cpu6502::jsr, addr_mode: Cpu6502::abs, cycles: 6 },
        Instruction { name: String::from("AND"), op: Cpu6502::and, addr_mode: Cpu6502::izx, cycles: 6 },
        Instruction { name: String::from("KIL"), op: Cpu6502::kil, addr_mode: Cpu6502::imp, cycles: 2 },
        Instruction { name: String::from("*RLA"), op: Cpu6502::rla, addr_mode: Cpu6502::izx, cycles: 8 },
        Instruction { name: String::from("BIT"), op: Cpu6502::bit, addr_mode: Cpu6502::zp0, cycles: 3 },
        Instruction { name: String::from("AND"), op: Cpu6502::and, addr_mode: Cpu6502::zp0, cycles: 3 },
        Instruction { name: String::from("ROL"), op: Cpu6502::rol, addr_mode: Cpu6502::zp0, cycles: 5 },
        Instruction { name: String::from("*RLA"), op: Cpu6502::rla, addr_mode: Cpu6502::zp0, cycles: 5 },
        Instruction { name: String::from("PLP"), op: Cpu6502::plp, addr_mode: Cpu6502::imp, cycles: 4 },
        Instruction { name: String::from("AND"), op: Cpu6502::and, addr_mode: Cpu6502::imm, cycles: 2 },
        Instruction { name: String::from("ROL A"), op: Cpu6502::rol, addr_mode: Cpu6502::imp, cycles: 2 },
        Instruction { name: String::from("*ANC"), op: Cpu6502::anc, addr_mode: Cpu6502::imm, cycles: 2 },
        Instruction { name: String::from("BIT"), op: Cpu6502::bit, addr_mode: Cpu6502::abs, cycles: 4 },
        Instruction { name: String::from("AND"), op: Cpu6502::and, addr_mode: Cpu6502::abs, cycles: 4 },
        Instruction { name: String::from("ROL"), op: Cpu6502::rol, addr_mode: Cpu6502::abs, cycles: 6 },
        Instruction { name: String::from("*RLA"), op: Cpu6502::rla, addr_mode: Cpu6502::abs, cycles: 6 },

        // 30 - 3F
        Instruction { name: String::from("BMI"), op: Cpu6502::bmi, addr_mode: Cpu6502::rel, cycles: 2 },
        Instruction { name: String::from("AND"), op: Cpu6502::and, addr_mode: Cpu6502::izy, cycles: 5 },
        Instruction { name: String::from("KIL"), op: Cpu6502::kil, addr_mode: Cpu6502::imp, cycles: 2 },
        Instruction { name: String::from("*RLA"), op: Cpu6502::rla, addr_mode: Cpu6502::izy, cycles: 8 },
        Instruction { name: String::from("*NOP"), op: Cpu6502::nop, addr_mode: Cpu6502::zpx, cycles: 4 },
        Instruction { name: String::from("AND"), op: Cpu6502::and, addr_mode: Cpu6502::zpx, cycles: 4 },
        Instruction { name: String::from("ROL"), op: Cpu6502::rol, addr_mode: Cpu6502::zpx, cycles: 6 },
        Instruction { name: String::from("*RLA"), op: Cpu6502::rla, addr_mode: Cpu6502::zpx, cycles: 6 },
        Instruction { name: String::from("SEC"), op: Cpu6502::sec, addr_mode: Cpu6502::imp, cycles: 2 },
        Instruction { name: String::from("AND"), op: Cpu6502::and, addr_mode: Cpu6502::aby, cycles: 4 },
        Instruction { name: String::from("*NOP"), op: Cpu6502::nop, addr_mode: Cpu6502::imp, cycles: 2 },
        Instruction { name: String::from("*RLA"), op: Cpu6502::rla, addr_mode: Cpu6502::aby, cycles: 7 },
        Instruction { name: String::from("*NOP"), op: Cpu6502::nop, addr_mode: Cpu6502::abx, cycles: 4 },
        Instruction { name: String::from("AND"), op: Cpu6502::and, addr_mode: Cpu6502::abx, cycles: 4 },
        Instruction { name: String::from("ROL"), op: Cpu6502::rol, addr_mode: Cpu6502::abx, cycles: 7 },
        Instruction { name: String::from("*RLA"), op: Cpu6502::rla, addr_mode: Cpu6502::abx, cycles: 7 },

        // 40 - 4F
        Instruction { name: String::from("RTI"), op: Cpu6502::rti, addr_mode: Cpu6502::imp, cycles: 6 },
        Instruction { name: String::from("EOR"), op: Cpu6502::eor, addr_mode: Cpu6502::izx, cycles: 6 },
        Instruction { name: String::from("KIL"), op: Cpu6502::kil, addr_mode: Cpu6502::imp, cycles: 2 },
        Instruction { name: String::from("*SRE"), op: Cpu6502::sre, addr_mode: Cpu6502::izx, cycles: 8 },
        Instruction { name: String::from("*NOP"), op: Cpu6502::nop, addr_mode: Cpu6502::zp0, cycles: 3 },
        Instruction { name: String::from("EOR"), op: Cpu6502::eor, addr_mode: Cpu6502::zp0, cycles: 3 },
        Instruction { name: String::from("LSR"), op: Cpu6502::lsr, addr_mode: Cpu6502::zp0, cycles: 5 },
        Instruction { name: String::from("*SRE"), op: Cpu6502::sre, addr_mode: Cpu6502::zp0, cycles: 5 },
        Instruction { name: String::from("PHA"), op: Cpu6502::pha, addr_mode: Cpu6502::imp, cycles: 3 },
        Instruction { name: String::from("EOR"), op: Cpu6502::eor, addr_mode: Cpu6502::imm, cycles: 2 },
        Instruction { name: String::from("LSR A"), op: Cpu6502::lsr, addr_mode: Cpu6502::imp, cycles: 2 },
        Instruction { name: String::from("ALR"), op: Cpu6502::alr, addr_mode: Cpu6502::imm, cycles: 2 },
        Instruction { name: String::from("JMP"), op: Cpu6502::jmp, addr_mode: Cpu6502::abs, cycles: 3 },
        Instruction { name: String::from("EOR"), op: Cpu6502::eor, addr_mode: Cpu6502::abs, cycles: 4 },
        Instruction { name: String::from("LSR"), op: Cpu6502::lsr, addr_mode: Cpu6502::abs, cycles: 6 },
        Instruction { name: String::from("*SRE"), op: Cpu6502::sre, addr_mode: Cpu6502::abs, cycles: 6 },

        // 50 - 5F
        Instruction { name: String::from("BVC"), op: Cpu6502::bvc, addr_mode: Cpu6502::rel, cycles: 2 },
        Instruction { name: String::from("EOR"), op: Cpu6502::eor, addr_mode: Cpu6502::izy, cycles: 5 },
        Instruction { name: String::from("KIL"), op: Cpu6502::kil, addr_mode: Cpu6502::imp, cycles: 2 },
        Instruction { name: String::from("*SRE"), op: Cpu6502::sre, addr_mode: Cpu6502::izy, cycles: 8 },
        Instruction { name: String::from("*NOP"), op: Cpu6502::nop, addr_mode: Cpu6502::zpx, cycles: 4 },
        Instruction { name: String::from("EOR"), op: Cpu6502::eor, addr_mode: Cpu6502::zpx, cycles: 4 },
        Instruction { name: String::from("LSR"), op: Cpu6502::lsr, addr_mode: Cpu6502::zpx, cycles: 6 },
        Instruction { name: String::from("*SRE"), op: Cpu6502::sre, addr_mode: Cpu6502::zpx, cycles: 6 },
        Instruction { name: String::from("CLI"), op: Cpu6502::cli, addr_mode: Cpu6502::imp, cycles: 2 },
        Instruction { name: String::from("EOR"), op: Cpu6502::eor, addr_mode: Cpu6502::aby, cycles: 4 },
        Instruction { name: String::from("*NOP"), op: Cpu6502::nop, addr_mode: Cpu6502::imp, cycles: 2 },
        Instruction { name: String::from("*SRE"), op: Cpu6502::sre, addr_mode: Cpu6502::aby, cycles: 7 },
        Instruction { name: String::from("*NOP"), op: Cpu6502::nop, addr_mode: Cpu6502::abx, cycles: 4 },
        Instruction { name: String::from("EOR"), op: Cpu6502::eor, addr_mode: Cpu6502::abx, cycles: 4 },
        Instruction { name: String::from("LSR"), op: Cpu6502::lsr, addr_mode: Cpu6502::abx, cycles: 7 },
        Instruction { name: String::from("*SRE"), op: Cpu6502::sre, addr_mode: Cpu6502::abx, cycles: 7 },

        // 60 - 6F
        Instruction { name: String::from("RTS"), op: Cpu6502::rts, addr_mode: Cpu6502::imp, cycles: 6 },
        Instruction { name: String::from("ADC"), op: Cpu6502::adc, addr_mode: Cpu6502::izx, cycles: 6 },
        Instruction { name: String::from("KIL"), op: Cpu6502::kil, addr_mode: Cpu6502::imp, cycles: 2 },
        Instruction { name: String::from("*RRA"), op: Cpu6502::rra, addr_mode: Cpu6502::izx, cycles: 8 },
        Instruction { name: String::from("*NOP"), op: Cpu6502::nop, addr_mode: Cpu6502::zp0, cycles: 3 },
        Instruction { name: String::from("ADC"), op: Cpu6502::adc, addr_mode: Cpu6502::zp0, cycles: 3 },
        Instruction { name: String::from("ROR"), op: Cpu6502::ror, addr_mode: Cpu6502::zp0, cycles: 5 },
        Instruction { name: String::from("*RRA"), op: Cpu6502::rra, addr_mode: Cpu6502::zp0, cycles: 5 },
        Instruction { name: String::from("PLA"), op: Cpu6502::pla, addr_mode: Cpu6502::imp, cycles: 4 },
        Instruction { name: String::from("ADC"), op: Cpu6502::adc, addr_mode: Cpu6502::imm, cycles: 2 },
        Instruction { name: String::from("ROR A"), op: Cpu6502::ror, addr_mode: Cpu6502::imp, cycles: 2 },
        Instruction { name: String::from("ARR"), op: Cpu6502::arr, addr_mode: Cpu6502::imm, cycles: 2 },
        Instruction { name: String::from("JMP"), op: Cpu6502::jmp, addr_mode: Cpu6502::ind, cycles: 5 },
        Instruction { name: String::from("ADC"), op: Cpu6502::adc, addr_mode: Cpu6502::abs, cycles: 4 },
        Instruction { name: String::from("ROR"), op: Cpu6502::ror, addr_mode: Cpu6502::abs, cycles: 6 },
        Instruction { name: String::from("*RRA"), op: Cpu6502::rra, addr_mode: Cpu6502::abs, cycles: 6 },

        // 70 - 7F
        Instruction { name: String::from("BVS"), op: Cpu6502::bvs, addr_mode: Cpu6502::rel, cycles: 2 },
        Instruction { name: String::from("ADC"), op: Cpu6502::adc, addr_mode: Cpu6502::izy, cycles: 5 },
        Instruction { name: String::from("KIL"), op: Cpu6502::kil, addr_mode: Cpu6502::imp, cycles: 2 },
        Instruction { name: String::from("*RRA"), op: Cpu6502::rra, addr_mode: Cpu6502::izy, cycles: 8 },
        Instruction { name: String::from("*NOP"), op: Cpu6502::nop, addr_mode: Cpu6502::zpx, cycles: 4 },
        Instruction { name: String::from("ADC"), op: Cpu6502::adc, addr_mode: Cpu6502::zpx, cycles: 4 },
        Instruction { name: String::from("ROR"), op: Cpu6502::ror, addr_mode: Cpu6502::zpx, cycles: 6 },
        Instruction { name: String::from("*RRA"), op: Cpu6502::rra, addr_mode: Cpu6502::zpx, cycles: 6 },
        Instruction { name: String::from("SEI"), op: Cpu6502::sei, addr_mode: Cpu6502::imp, cycles: 2 },
        Instruction { name: String::from("ADC"), op: Cpu6502::adc, addr_mode: Cpu6502::aby, cycles: 4 },
        Instruction { name: String::from("*NOP"), op: Cpu6502::nop, addr_mode: Cpu6502::imp, cycles: 2 },
        Instruction { name: String::from("*RRA"), op: Cpu6502::rra, addr_mode: Cpu6502::aby, cycles: 7 },
        Instruction { name: String::from("*NOP"), op: Cpu6502::nop, addr_mode: Cpu6502::abx, cycles: 4 },
        Instruction { name: String::from("ADC"), op: Cpu6502::adc, addr_mode: Cpu6502::abx, cycles: 4 },
        Instruction { name: String::from("ROR"), op: Cpu6502::ror, addr_mode: Cpu6502::abx, cycles: 7 },
        Instruction { name: String::from("*RRA"), op: Cpu6502::rra, addr_mode: Cpu6502::abx, cycles: 7 },

        // 80 - 8F
        Instruction { name: String::from("*NOP"), op: Cpu6502::nop, addr_mode: Cpu6502::imm, cycles: 2 },
        Instruction { name: String::from("STA"), op: Cpu6502::sta, addr_mode: Cpu6502::izx, cycles: 6 },
        Instruction { name: String::from("*NOP"), op: Cpu6502::nop, addr_mode: Cpu6502::imp, cycles: 2 },
        Instruction { name: String::from("*SAX"), op: Cpu6502::sax, addr_mode: Cpu6502::izx, cycles: 6 },
        Instruction { name: String::from("STY"), op: Cpu6502::sty, addr_mode: Cpu6502::zp0, cycles: 3 },
        Instruction { name: String::from("STA"), op: Cpu6502::sta, addr_mode: Cpu6502::zp0, cycles: 3 },
        Instruction { name: String::from("STX"), op: Cpu6502::stx, addr_mode: Cpu6502::zp0, cycles: 3 },
        Instruction { name: String::from("*SAX"), op: Cpu6502::sax, addr_mode: Cpu6502::zp0, cycles: 3 },
        Instruction { name: String::from("DEY"), op: Cpu6502::dey, addr_mode: Cpu6502::imp, cycles: 2 },
        Instruction { name: String::from("*NOP"), op: Cpu6502::nop, addr_mode: Cpu6502::imp, cycles: 2 },
        Instruction { name: String::from("TXA"), op: Cpu6502::txa, addr_mode: Cpu6502::imp, cycles: 2 },
        Instruction { name: String::from("**XAA"), op: Cpu6502::xaa, addr_mode: Cpu6502::imm, cycles: 2 },
        Instruction { name: String::from("STY"), op: Cpu6502::sty, addr_mode: Cpu6502::abs, cycles: 4 },
        Instruction { name: String::from("STA"), op: Cpu6502::sta, addr_mode: Cpu6502::abs, cycles: 4 },
        Instruction { name: String::from("STX"), op: Cpu6502::stx, addr_mode: Cpu6502::abs, cycles: 4 },
        Instruction { name: String::from("*SAX"), op: Cpu6502::sax, addr_mode: Cpu6502::abs, cycles: 4 },

        // 90 - 9F
        Instruction { name: String::from("BCC"), op: Cpu6502::bcc, addr_mode: Cpu6502::rel, cycles: 2 },
        Instruction { name: String::from("STA"), op: Cpu6502::sta, addr_mode: Cpu6502::izy, cycles: 6 },
        Instruction { name: String::from("KIL"), op: Cpu6502::kil, addr_mode: Cpu6502::imp, cycles: 2 },
        Instruction { name: String::from("**AHX"), op: Cpu6502::ahx, addr_mode: Cpu6502::imp, cycles: 6 },
        Instruction { name: String::from("STY"), op: Cpu6502::sty, addr_mode: Cpu6502::zpx, cycles: 4 },
        Instruction { name: String::from("STA"), op: Cpu6502::sta, addr_mode: Cpu6502::zpx, cycles: 4 },
        Instruction { name: String::from("STX"), op: Cpu6502::stx, addr_mode: Cpu6502::zpy, cycles: 4 },
        Instruction { name: String::from("*SAX"), op: Cpu6502::sax, addr_mode: Cpu6502::zpy, cycles: 4 },
        Instruction { name: String::from("TYA"), op: Cpu6502::tya, addr_mode: Cpu6502::imp, cycles: 2 },
        Instruction { name: String::from("STA"), op: Cpu6502::sta, addr_mode: Cpu6502::aby, cycles: 5 },
        Instruction { name: String::from("TXS"), op: Cpu6502::txs, addr_mode: Cpu6502::imp, cycles: 2 },
        Instruction { name: String::from("**TAS"), op: Cpu6502::tas, addr_mode: Cpu6502::aby, cycles: 5 },
        Instruction { name: String::from("*NOP"), op: Cpu6502::nop, addr_mode: Cpu6502::abx, cycles: 5 },
        Instruction { name: String::from("STA"), op: Cpu6502::sta, addr_mode: Cpu6502::abx, cycles: 5 },
        Instruction { name: String::from("**SHX"), op: Cpu6502::shx, addr_mode: Cpu6502::aby, cycles: 5 },
        Instruction { name: String::from("**AHX"), op: Cpu6502::ahx, addr_mode: Cpu6502::aby, cycles: 5 },

        // A0 - AF
        Instruction { name: String::from("LDY"), op: Cpu6502::ldy, addr_mode: Cpu6502::imm, cycles: 2 },
        Instruction { name: String::from("LDA"), op: Cpu6502::lda, addr_mode: Cpu6502::izx, cycles: 6 },
        Instruction { name: String::from("LDX"), op: Cpu6502::ldx, addr_mode: Cpu6502::imm, cycles: 2 },
        Instruction { name: String::from("*LAX"), op: Cpu6502::lax, addr_mode: Cpu6502::izx, cycles: 6 },
        Instruction { name: String::from("LDY"), op: Cpu6502::ldy, addr_mode: Cpu6502::zp0, cycles: 3 },
        Instruction { name: String::from("LDA"), op: Cpu6502::lda, addr_mode: Cpu6502::zp0, cycles: 3 },
        Instruction { name: String::from("LDX"), op: Cpu6502::ldx, addr_mode: Cpu6502::zp0, cycles: 3 },
        Instruction { name: String::from("*LAX"), op: Cpu6502::lax, addr_mode: Cpu6502::zp0, cycles: 3 },
        Instruction { name: String::from("TAY"), op: Cpu6502::tay, addr_mode: Cpu6502::imp, cycles: 2 },
        Instruction { name: String::from("LDA"), op: Cpu6502::lda, addr_mode: Cpu6502::imm, cycles: 2 },
        Instruction { name: String::from("TAX"), op: Cpu6502::tax, addr_mode: Cpu6502::imp, cycles: 2 },
        Instruction { name: String::from("**LAX"), op: Cpu6502::lax, addr_mode: Cpu6502::imm, cycles: 2 },
        Instruction { name: String::from("LDY"), op: Cpu6502::ldy, addr_mode: Cpu6502::abs, cycles: 4 },
        Instruction { name: String::from("LDA"), op: Cpu6502::lda, addr_mode: Cpu6502::abs, cycles: 4 },
        Instruction { name: String::from("LDX"), op: Cpu6502::ldx, addr_mode: Cpu6502::abs, cycles: 4 },
        Instruction { name: String::from("*LAX"), op: Cpu6502::lax, addr_mode: Cpu6502::abs, cycles: 4 },

        // B0 - BF
        Instruction { name: String::from("BCS"), op: Cpu6502::bcs, addr_mode: Cpu6502::rel, cycles: 2 },
        Instruction { name: String::from("LDA"), op: Cpu6502::lda, addr_mode: Cpu6502::izy, cycles: 5 },
        Instruction { name: String::from("KIL"), op: Cpu6502::kil, addr_mode: Cpu6502::imp, cycles: 2 },
        Instruction { name: String::from("*LAX"), op: Cpu6502::lax, addr_mode: Cpu6502::izy, cycles: 5 },
        Instruction { name: String::from("LDY"), op: Cpu6502::ldy, addr_mode: Cpu6502::zpx, cycles: 4 },
        Instruction { name: String::from("LDA"), op: Cpu6502::lda, addr_mode: Cpu6502::zpx, cycles: 4 },
        Instruction { name: String::from("LDX"), op: Cpu6502::ldx, addr_mode: Cpu6502::zpy, cycles: 4 },
        Instruction { name: String::from("*LAX"), op: Cpu6502::lax, addr_mode: Cpu6502::zpy, cycles: 4 },
        Instruction { name: String::from("CLV"), op: Cpu6502::clv, addr_mode: Cpu6502::imp, cycles: 2 },
        Instruction { name: String::from("LDA"), op: Cpu6502::lda, addr_mode: Cpu6502::aby, cycles: 4 },
        Instruction { name: String::from("TSX"), op: Cpu6502::tsx, addr_mode: Cpu6502::imp, cycles: 2 },
        Instruction { name: String::from("*LAS"), op: Cpu6502::las, addr_mode: Cpu6502::aby, cycles: 4 },
        Instruction { name: String::from("LDY"), op: Cpu6502::ldy, addr_mode: Cpu6502::abx, cycles: 4 },
        Instruction { name: String::from("LDA"), op: Cpu6502::lda, addr_mode: Cpu6502::abx, cycles: 4 },
        Instruction { name: String::from("LDX"), op: Cpu6502::ldx, addr_mode: Cpu6502::aby, cycles: 4 },
        Instruction { name: String::from("*LAX"), op: Cpu6502::lax, addr_mode: Cpu6502::aby, cycles: 4 },

        // C0 - CF
        Instruction { name: String::from("CPY"), op: Cpu6502::cpy, addr_mode: Cpu6502::imm, cycles: 2 },
        Instruction { name: String::from("CMP"), op: Cpu6502::cmp, addr_mode: Cpu6502::izx, cycles: 6 },
        Instruction { name: String::from("*NOP"), op: Cpu6502::nop, addr_mode: Cpu6502::imp, cycles: 2 },
        Instruction { name: String::from("*DCP"), op: Cpu6502::dcp, addr_mode: Cpu6502::izx, cycles: 8 },
        Instruction { name: String::from("CPY"), op: Cpu6502::cpy, addr_mode: Cpu6502::zp0, cycles: 3 },
        Instruction { name: String::from("CMP"), op: Cpu6502::cmp, addr_mode: Cpu6502::zp0, cycles: 3 },
        Instruction { name: String::from("DEC"), op: Cpu6502::dec, addr_mode: Cpu6502::zp0, cycles: 5 },
        Instruction { name: String::from("*DCP"), op: Cpu6502::dcp, addr_mode: Cpu6502::zp0, cycles: 5 },
        Instruction { name: String::from("INY"), op: Cpu6502::iny, addr_mode: Cpu6502::imp, cycles: 2 },
        Instruction { name: String::from("CMP"), op: Cpu6502::cmp, addr_mode: Cpu6502::imm, cycles: 2 },
        Instruction { name: String::from("DEX"), op: Cpu6502::dex, addr_mode: Cpu6502::imp, cycles: 2 },
        Instruction { name: String::from("*AXS"), op: Cpu6502::axs, addr_mode: Cpu6502::imm, cycles: 2 },
        Instruction { name: String::from("CPY"), op: Cpu6502::cpy, addr_mode: Cpu6502::abs, cycles: 4 },
        Instruction { name: String::from("CMP"), op: Cpu6502::cmp, addr_mode: Cpu6502::abs, cycles: 4 },
        Instruction { name: String::from("DEC"), op: Cpu6502::dec, addr_mode: Cpu6502::abs, cycles: 6 },
        Instruction { name: String::from("*DCP"), op: Cpu6502::dcp, addr_mode: Cpu6502::abs, cycles: 6 },

        // D0 - DF
        Instruction { name: String::from("BNE"), op: Cpu6502::bne, addr_mode: Cpu6502::rel, cycles: 2 },
        Instruction { name: String::from("CMP"), op: Cpu6502::cmp, addr_mode: Cpu6502::izy, cycles: 5 },
        Instruction { name: String::from("KIL"), op: Cpu6502::kil, addr_mode: Cpu6502::imp, cycles: 2 },
        Instruction { name: String::from("*DCP"), op: Cpu6502::dcp, addr_mode: Cpu6502::izy, cycles: 8 },
        Instruction { name: String::from("*NOP"), op: Cpu6502::nop, addr_mode: Cpu6502::zpx, cycles: 4 },
        Instruction { name: String::from("CMP"), op: Cpu6502::cmp, addr_mode: Cpu6502::zpx, cycles: 4 },
        Instruction { name: String::from("DEC"), op: Cpu6502::dec, addr_mode: Cpu6502::zpx, cycles: 6 },
        Instruction { name: String::from("*DCP"), op: Cpu6502::dcp, addr_mode: Cpu6502::zpx, cycles: 6 },
        Instruction { name: String::from("CLD"), op: Cpu6502::cld, addr_mode: Cpu6502::imp, cycles: 2 },
        Instruction { name: String::from("CMP"), op: Cpu6502::cmp, addr_mode: Cpu6502::aby, cycles: 4 },
        Instruction { name: String::from("*NOP"), op: Cpu6502::nop, addr_mode: Cpu6502::imp, cycles: 2 },
        Instruction { name: String::from("*DCP"), op: Cpu6502::dcp, addr_mode: Cpu6502::aby, cycles: 7 },
        Instruction { name: String::from("*NOP"), op: Cpu6502::nop, addr_mode: Cpu6502::abx, cycles: 4 },
        Instruction { name: String::from("CMP"), op: Cpu6502::cmp, addr_mode: Cpu6502::abx, cycles: 4 },
        Instruction { name: String::from("DEC"), op: Cpu6502::dec, addr_mode: Cpu6502::abx, cycles: 7 },
        Instruction { name: String::from("*DCP"), op: Cpu6502::dcp, addr_mode: Cpu6502::abx, cycles: 7 },

        // E0 - EF
        Instruction { name: String::from("CPX"), op: Cpu6502::cpx, addr_mode: Cpu6502::imm, cycles: 2 },
        Instruction { name: String::from("SBC"), op: Cpu6502::sbc, addr_mode: Cpu6502::izx, cycles: 6 },
        Instruction { name: String::from("*NOP"), op: Cpu6502::nop, addr_mode: Cpu6502::imp, cycles: 2 },
        Instruction { name: String::from("*ISB"), op: Cpu6502::isb, addr_mode: Cpu6502::izx, cycles: 8 },
        Instruction { name: String::from("CPX"), op: Cpu6502::cpx, addr_mode: Cpu6502::zp0, cycles: 3 },
        Instruction { name: String::from("SBC"), op: Cpu6502::sbc, addr_mode: Cpu6502::zp0, cycles: 3 },
        Instruction { name: String::from("INC"), op: Cpu6502::inc, addr_mode: Cpu6502::zp0, cycles: 5 },
        Instruction { name: String::from("*ISB"), op: Cpu6502::isb, addr_mode: Cpu6502::zp0, cycles: 5 },
        Instruction { name: String::from("INX"), op: Cpu6502::inx, addr_mode: Cpu6502::imp, cycles: 2 },
        Instruction { name: String::from("SBC"), op: Cpu6502::sbc, addr_mode: Cpu6502::imm, cycles: 2 },
        Instruction { name: String::from("NOP"), op: Cpu6502::nop, addr_mode: Cpu6502::imp, cycles: 2 },
        Instruction { name: String::from("*SBC"), op: Cpu6502::sbc, addr_mode: Cpu6502::imm, cycles: 2 },
        Instruction { name: String::from("CPX"), op: Cpu6502::cpx, addr_mode: Cpu6502::abs, cycles: 4 },
        Instruction { name: String::from("SBC"), op: Cpu6502::sbc, addr_mode: Cpu6502::abs, cycles: 4 },
        Instruction { name: String::from("INC"), op: Cpu6502::inc, addr_mode: Cpu6502::abs, cycles: 6 },
        Instruction { name: String::from("*ISB"), op: Cpu6502::isb, addr_mode: Cpu6502::abs, cycles: 6 },

        // F0 - FF
        Instruction { name: String::from("BEQ"), op: Cpu6502::beq, addr_mode: Cpu6502::rel, cycles: 2 },
        Instruction { name: String::from("SBC"), op: Cpu6502::sbc, addr_mode: Cpu6502::izy, cycles: 5 },
        Instruction { name: String::from("KIL"), op: Cpu6502::kil, addr_mode: Cpu6502::imp, cycles: 2 },
        Instruction { name: String::from("*ISB"), op: Cpu6502::isb, addr_mode: Cpu6502::izy, cycles: 8 },
        Instruction { name: String::from("*NOP"), op: Cpu6502::nop, addr_mode: Cpu6502::zpx, cycles: 4 },
        Instruction { name: String::from("SBC"), op: Cpu6502::sbc, addr_mode: Cpu6502::zpx, cycles: 4 },
        Instruction { name: String::from("INC"), op: Cpu6502::inc, addr_mode: Cpu6502::zpx, cycles: 6 },
        Instruction { name: String::from("*ISB"), op: Cpu6502::isb, addr_mode: Cpu6502::zpx, cycles: 6 },
        Instruction { name: String::from("SED"), op: Cpu6502::sed, addr_mode: Cpu6502::imp, cycles: 2 },
        Instruction { name: String::from("SBC"), op: Cpu6502::sbc, addr_mode: Cpu6502::aby, cycles: 4 },
        Instruction { name: String::from("*NOP"), op: Cpu6502::nop, addr_mode: Cpu6502::imp, cycles: 2 },
        Instruction { name: String::from("*ISB"), op: Cpu6502::isb, addr_mode: Cpu6502::aby, cycles: 7 },
        Instruction { name: String::from("*NOP"), op: Cpu6502::nop, addr_mode: Cpu6502::abx, cycles: 4 },
        Instruction { name: String::from("SBC"), op: Cpu6502::sbc, addr_mode: Cpu6502::abx, cycles: 4 },
        Instruction { name: String::from("INC"), op: Cpu6502::inc, addr_mode: Cpu6502::abx, cycles: 7 },
        Instruction { name: String::from("*ISB"), op: Cpu6502::isb, addr_mode: Cpu6502::abx, cycles: 7 }
        ]
    };
}