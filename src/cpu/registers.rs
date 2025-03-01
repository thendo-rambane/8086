#[derive(Default)]
pub struct Register {
    x: u16,
}

impl Register {
    /// Creates a new `Register` with an initial value of 0x0000.
    fn new() -> Register {
        Register { x: 0x0000 }
    }

    /// Returns the low byte of the register.
    pub fn low(&self) -> u8 {
        (self.x & 0x00FF) as u8
    }

    /// Returns the high byte of the register.
    pub fn high(&self) -> u8 {
        ((self.x & 0xFF00) >> 8) as u8
    }

    /// Sets the low byte of the register.
    pub fn set_low(&mut self, value: u8) {
        self.x = (self.x & 0xFF00) | (value as u16);
    }

    /// Sets the high byte of the register.
    pub fn set_high(&mut self, value: u8) {
        self.x = (self.x & 0x00FF) | ((value as u16) << 8);
    }

    /// Sets the entire 16-bit value of the register.
    pub fn set(&mut self, value: u16) {
        self.x = value;
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_register() {
        let reg = Register::new();
        assert_eq!(reg.x, 0x0000);
    }

    #[test]
    fn test_low() {
        let reg = Register { x: 0x1234 };
        assert_eq!(reg.low(), 0x34);
    }

    #[test]
    fn test_high() {
        let reg = Register { x: 0x1234 };
        assert_eq!(reg.high(), 0x12);
    }

    #[test]
    fn test_set_low() {
        let mut reg = Register { x: 0x1234 };
        reg.set_low(0x56);
        assert_eq!(reg.x, 0x1256);
    }

    #[test]
    fn test_set_high() {
        let mut reg = Register { x: 0x1234 };
        reg.set_high(0x78);
        assert_eq!(reg.x, 0x7834);
    }

    #[test]
    fn test_set() {
        let mut reg = Register::new();
        reg.set(0x9ABC);
        assert_eq!(reg.x, 0x9ABC);
    }
}
