#[derive(Debug)]
pub struct AluResult {
    pub result: u8,
    pub half_carry: bool,
    pub carry: bool,
}

impl AluResult {
    /// Create an `AluResult` from a wrapping add
    pub fn from_add(a: u8, b: u8) -> AluResult {
        AluResult::from_adc(a, b, false)
    }

    /// Create an `AluResult` from a wrapping add with carry
    pub fn from_adc(a: u8, b: u8, carry: bool) -> AluResult {
        let carry = carry as u16;
        let result = a as u16 + b as u16 + carry;

        let half_carry = (a & 0x0F) + (b & 0x0F) + carry as u8 > 0x0F;
        let carry_flag = result > u8::MAX as u16;

        AluResult {
            result: result as u8,
            half_carry,
            carry: carry_flag,
        }
    }

    /// Create an `AluResult` from a wrapping subtraction
    pub fn from_sub(a: u8, b: u8) -> AluResult {
        AluResult::from_sbc(a, b, false)
    }

    /// Create an `AluResult` from a wrapping subtraction with carry
    pub fn from_sbc(a: u8, b: u8, carry: bool) -> AluResult {
        let a = a as i16;
        let b = b as i16;
        let carry = carry as i16;
        let result = (a & 0xff) - (b & 0xff) - carry;

        let lower_sbc = (a & 0xf) - (b & 0xf) - carry;

        let half_carry = (lower_sbc & 0x1f) > 0xf;
        let carry = (b + carry) > a;

        AluResult {
            result: result as u8,
            half_carry,
            carry,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let alu = AluResult::from_add(5, 10);

        assert_eq!(alu.result, 15);
        assert_eq!(alu.half_carry, false);
        assert_eq!(alu.carry, false);
    }

    #[test]
    fn test_add_half_carry() {
        let alu = AluResult::from_add(0x0F, 1);

        assert_eq!(alu.result, 0x10);
        assert_eq!(alu.half_carry, true);
        assert_eq!(alu.carry, false);
    }

    #[test]
    fn test_add_carry() {
        let alu = AluResult::from_add(u8::MAX, 10);

        assert_eq!(alu.result, 9);
        assert_eq!(alu.half_carry, true);
        assert_eq!(alu.carry, true);
    }

    #[test]
    fn test_sub() {
        let alu = AluResult::from_sub(15, 10);

        assert_eq!(alu.result, 5);
        assert_eq!(alu.half_carry, false);
        assert_eq!(alu.carry, false);
    }

    #[test]
    fn test_sub_half_carry() {
        let alu = AluResult::from_sub(0x10, 1);

        assert_eq!(alu.result, 0x0F);
        assert_eq!(alu.half_carry, true);
        assert_eq!(alu.carry, false);
    }

    #[test]
    fn test_sub_carry() {
        let alu = AluResult::from_sub(0, 10);

        assert_eq!(alu.result, 246);
        assert_eq!(alu.half_carry, true);
        assert_eq!(alu.carry, true);
    }
}
