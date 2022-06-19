#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConditionalFlag {
    NZ,
    Z,
    NC,
    C,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Register {
    B,
    C,
    D,
    E,
    H,
    L,
    A,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DoubleRegister {
    BC,
    DE,
    HL,
    AF,
    SP,
}

#[derive(Debug, PartialEq, Eq)]
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
    /// 0xA{0-7} - AND R
    AndAReg {
        register: Register,
    },
    /// 0xE6 - AND d8
    AndA,
    /// 0xA6 - AND (HL)
    AndAHL,
    /// 0xA{8-F} - XOR R
    XorAReg {
        register: Register,
    },
    /// 0xEE - XOR d8
    XorA,
    /// 0xAE - XOR (HL)
    XorAHL,
    /// 0xB{0-7} - OR R
    OrAReg {
        register: Register,
    },
    /// 0xF6 - OR d8
    OrA,
    /// 0xB6 - OR (HL)
    OrAHL,
    /// 0xB{8-F} - CP R
    CompareAReg {
        register: Register,
    },
    /// 0xFE - CP d8
    CompareA,
    /// 0xBE - CP (HL)
    CompareAHL,
    /// 0x{0-2}{4,C} - INC R
    IncrementReg {
        register: Register,
    },
    /// 0x34 - INC (HL)
    IncrementHL,
    /// 0x{0-2}{5,D} - DEC R
    DecrementReg {
        register: Register,
    },
    /// 0x35 - DEC (HL)
    DecrementHL,
    /// 0x27 - DAA
    DecimalAdjustA,
    /// 0x2F - CPL
    Complement,

    // 16-bit Arithmetic/Logic instructions
    /// 0x{0-3}9 - ADD HL, RR
    AddHLReg {
        register: DoubleRegister,
    },
    /// 0x{0-3}3 - INC RR
    IncrementReg16 {
        register: DoubleRegister,
    },
    /// 0x{0-4}B - DEC RR
    DecrementReg16 {
        register: DoubleRegister,
    },
    /// 0xE8 - ADD SP,r8
    AddSPOffset,
    /// 0xF8 - LD HL,SP+r8
    LoadHLSPOffset,

    // Rotate and Shift instructions
    /// 0x07 - RLCA
    RotateALeft,
    /// 0x17 - RLA
    RotateALeftThroughCarry,
    /// 0x0F - RRCA
    RotateARight,
    /// 0x1F - RRA
    RotateARightThroughCarry,
    /// 0xCB 0x - RLC R
    RotateLeft {
        register: Register,
    },
    /// 0xCB 06 - RLC (HL)
    RotateHLLeft,
    /// 0xCB 1x - RL R
    RotateLeftThroughCarry {
        register: Register,
    },
    /// 0xCB 16 - RL (HL)
    RotateHLLeftThroughCarry,
    /// 0xCB 0x RRC R
    RotateRight {
        register: Register,
    },
    /// 0xCB 0E RRC (HL)
    RotateHLRight,
    /// 0xCB 1x - RR R
    RotateRightThroughCarry {
        register: Register,
    },
    /// 0xCB 1E - RR (HL)
    RotateHLRightThroughCarry,
    /// 0xCB 2x - SLA R
    ShiftLeftArithmetic {
        register: Register,
    },
    /// 0xCB 26 - SLA (HL)
    ShiftHLLeftArithmetic,
    /// 0xCB 3x - SWAP R
    Swap {
        register: Register,
    },
    /// 0xCB 36 - SWAP (HL)
    SwapHL,
    /// 0xCB 2x - SRA R
    ShiftRightArithmetic {
        register: Register,
    },
    /// 0xCB 2E - SRA (HL)
    ShiftHLRightArithmetic,
    /// 0xCB 3x - SRL R
    ShiftRightLogical {
        register: Register,
    },
    /// 0xCB 3E - SRL (HL)
    ShiftHLRightLogical,

    // Single-bit operation instructions
    /// 0xCB {4-7}x - BIT x,R
    TestBit {
        bit: u8,
        register: Register,
    },
    /// 0xCB {4-7}{6,E} - BIT x, (HL)
    TestHLBit {
        bit: u8,
    },
    /// 0xCB {C-F}x - SET x, R
    SetBit {
        bit: u8,
        register: Register,
    },
    /// 0xCB {C-F}{6,E} - SET x, (HL)
    SetHLBit {
        bit: u8,
    },
    /// 0xCB {8-B}x - RES x, R
    ResetBit {
        bit: u8,
        register: Register,
    },
    /// 0xCB {8-B}{6,E} - RES x, (HL)
    ResetHLBit {
        bit: u8,
    },

    // CPU Control instructions
    /// 0x3F - CCF
    FlipCarryFlag,
    /// 0x37 - SCF
    SetCarryFlag,
    /// 0x00 - NOP
    Nop,
    /// 0x76 - HALT
    Halt,
    /// 0x10 - STOP 0
    Stop,
    /// 0xF3 - DI
    DisableInterrupts,
    /// 0xFB - EI
    EnableInterrupts,

    // Jump instructions
    /// 0xC3 - JP a16
    Jump,
    /// 0xE9 - JP (HL)
    JumpHL,
    /// JP {f}, a16
    JumpConditional {
        flag: ConditionalFlag,
    },
    /// 0x18 - JR r8
    JumpRelative,
    /// JR f,r8
    JumpRelativeConditional {
        flag: ConditionalFlag,
    },
    /// 0xCD - CALL a16
    Call,
    /// CALL f, a16
    CallConditinal {
        flag: ConditionalFlag
    },
    /// 0xC9 - RET
    Return,
    /// RET f
    ReturnConditional {
        flag: ConditionalFlag
    },
    /// 0xD9 - RETI
    ReturnAndEnableInterrupts,
    /// 0x{C-F}6 - RST x0H
    Reset0 {
        location: u8,
    },
    /// 0x{C-F}F - RST x8H
    Reset8 {
        location: u8,
    },
}
