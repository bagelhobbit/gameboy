#[derive(Debug, Clone, Copy)]
pub enum ConditionalFlag {
    None,
    NZ,
    Z,
    NC,
    C,
}

#[derive(Debug, Clone, Copy)]
pub enum Register {
    B,
    C,
    D,
    E,
    H,
    L,
    A,
}

#[derive(Debug, Clone, Copy)]
pub enum DoubleRegister {
    BC,
    DE,
    HL,
    AF,
}

#[derive(Debug)]
pub enum Instruction {
    Invalid,
    // 8-bit load instructions
    /// LD R,R
    LoadReg {
        load_from: u8,
        load_into: Register,
    },
    /// LD R,d8
    LoadReg8 {
        register: Register,
    },
    /// LD R,(HL)
    LoadRegHL {
        register: Register,
    },
    /// 0x7_ - LD (HL),R
    LoadHLReg {
        register: Register,
    },
    /// 0x36 - LD (HL),d8
    LoadHL8,
    /// 0x0A - LD A,(BC)
    LoadABC,
    /// 0x1A -  LD A,(DE)
    LoadADE,
    /// 0xFA - LD A,(a16)
    LoadAAddress,
    /// 0x02 - LD (BC),A
    LoadBCA,
    /// 0x12 - LD (DE),A
    LoadDEA,
    /// 0xEA - LD (a16),A
    LoadAddressA,
    /// 0xF0 - LDH A,(a8)
    LoadAOffset,
    /// 0xE0 - LDH (a8),A
    LoadOffsetA,
    /// 0xF2 - LDH A,(C)
    LoadAOffsetC,
    /// 0xE2 - LD (C),A
    LoadOffsetCA,
    /// 0x22 - LDI (HL),A
    LoadIncrementHLA,
    /// 0x2A - LDI A,(HL)
    LoadIncrementAHL,
    /// 0x32 LDD (HL),A
    LoadDecrementHLA,
    /// 0x3A - LDD A,(HL)
    LoadDecrementAHL,

    // 16-bit load instructions
    /// 0x_1 - LD RR,d16
    LoadReg16 {
        register: DoubleRegister,
    },
    /// 0x08 - LD (a16),SP
    LoadAddressSP,
    /// 0xF9 - LD SP,HL
    LoadSPHL,
    /// 0x_5 - PUSH RR
    PushReg {
        register: DoubleRegister,
    },
    /// 0x_1 - POP RR
    PopReg {
        register: DoubleRegister,
    },

    // 8-bit Arithmetic/Logic instructions
    /// 0x8{0-7} - ADD A,R
    AddAReg {
        register: Register,
    },
    /// 0xC6 - ADD A, d8
    AddA,
    /// 0x86 - ADD A,(HL)
    AddAHL,
    /// 0x8{8-F} - ADC A,R
    AddCarryAReg {
        register: Register,
    },
    /// 0xCE - ADC A,d8
    AddCarryA,
    /// 0x8E - ADC A,(HL)
    AddCarryAHL,
    /// 0x9{0-7} - SUB R
    SubtractAReg {
        register: Register,
    },
    /// 0xD6 - SUB d8
    SubtractA,
    /// 0x96 - SUB (HL)
    SubtractAHL,
    /// 0x9{8-F} - SBC A,R
    SubtractARegCarry {
        register: Register,
    },
    /// 0xDE - SBC A,d8
    SubtractACarry,
    /// 0x9E - SBC A,(HL)
    SubtractAHLCarry,

    //--------------------
    /// 0x00 - NOP
    Nop,
    /// 0x07 - RLCA
    RotateALeft,
    /// 0x0F - RRCA
    RotateARight,
    /// 0x10 - STOP 0
    Stop,
    /// 0x17 - RLA
    RotateALeftThroughCarry,
    /// JR {f},r8
    JumpRelative {
        flag: ConditionalFlag,
    },
    /// 0x1F - RRA
    RotateARightThroughCarry,
    /// 0x27 - DAA
    DecimalAdjustA,
    /// 0x2F - CPL
    /// * stands for ComPLement?
    Cpl,
    /// 0x34 - INC (HL)
    IncrementHL,
    /// 0x35 - DEC (HL)
    DecrementHL,
    /// 0x37 - SCF
    Scf,
    /// 0x3F - CCF
    Ccf,
    /// 0x76 - HALT
    Halt,
    /// 0xA6 - AND (HL)
    AndAHL,
    /// 0xAE - XOR (HL)
    XorAHL,
    /// 0xA{0-7} - AND R
    AndAReg {
        reg: u8,
    },
    /// 0xA{8-F} - XOR R
    XorAReg {
        reg: u8,
    },
    /// 0xB6 - OR (HL)
    OrAHL,
    /// 0xBE - CP (HL)
    CompareAHL,
    /// 0xB{0-7} - OR R
    OrAReg {
        reg: u8,
    },
    /// 0xB{8-F} - CP R
    CompareAReg {
        reg: u8,
    },
    /// 0xC0 - RET NZ
    ReturnIfNotZero,
    /// 0xC2 - JP NZ,a16
    JumpIfNotZero {
        address: u16,
    },
    /// 0xC3 - JP a16
    Jump {
        address: u16,
    },
    /// 0xC4 - CALL NZ,a16
    CallIfNotZero {
        address: u16,
    },
    /// 0xC8 - RET Z
    ReturnIfZero,
    /// 0xC9 - RET
    Return,
    /// 0xCB - PREFIX CB
    Prefix {
        op: u8,
    },
    /// 0xCA - JP Z,a16
    JumpIfZero {
        address: u16,
    },
    ///0xCC - CALL Z,a16
    CallIfZero {
        address: u16,
    },
    /// 0xCD - CALL a16
    Call {
        address: u16,
    },
    /// 0xD0 - RET NC
    ReturnIfNotCarry,
    /// 0xD2 - JP NC,a16
    JumpIfNotCarry {
        address: u16,
    },
    /// 0xD3 - CALL NC,a16
    CallIfNotCarry {
        address: u16,
    },
    /// 0xD8 - RET C
    ReturnIfCarry,
    /// 0xD9 - RETI
    ReturnAndEnableInterrupts,
    /// 0xDA - JP C,a16
    JumpIfCarry {
        address: u16,
    },
    /// 0xDC - CALL C,a16
    CallIfCarry {
        address: u16,
    },
    /// 0xE6 - AND d8
    AndA {
        data: u8,
    },
    /// 0xE8 - ADD SP,r8
    AddSPOffset {
        offset: i8,
    },
    /// 0xE9 - JP (HL)
    JumpHL,
    /// 0xEE - XOR d8
    XorA,
    /// 0xF3 - DI
    DisableInterrupts,
    /// 0xF6 - OR d8
    OrA {
        data: u8,
    },
    /// 0xF8 - LD HL,SP+r8
    LoadHLSPOffset {
        offset: i8,
    },
    /// 0xFB - EI
    EnableInterrupts,
    /// 0xFE - CP d8
    CompareA {
        data: u8,
    },
    /// 0x{0-3}3 - INC RR
    IncrementReg16 {
        reg: u8,
    },
    /// 0x{0-3}4 - INC R
    IncrementHighReg {
        reg: u8,
    },
    /// 0x{0-3}5 - DEC R
    DecrementHighReg {
        reg: u8,
    },
    /// 0x{C-F}6 - RST x0H
    Reset0 {
        location: u8,
    },
    /// 0x{0-3}9 - ADD HL, RR
    AddHLReg {
        reg: u8,
    },
    /// 0x{0-4}B - DEC RR
    DecrementReg16 {
        reg: u8,
    },
    /// 0x{0-3}C - INC R
    IncrementLowReg {
        reg: u8,
    },
    /// 0x{0-3}D - DEC R
    DecrementLowReg {
        reg: u8,
    },
    /// 0x{C-F}F - RST x8H
    Reset8 {
        location: u8,
    },
}
