/// Represents the flags register of the CPU, which holds various status flags.
#[derive(Default)]
pub struct Flags {
    /// Set if the last operation resulted in a carry
    /// There has been a carry from the low nibble to the high nibble,
    /// or a borrow from the high nibble to the low nibble.
    ///
    /// Used by instructions that add or subtract multibyte numbers.
    /// Rotate instructions use this flag to isolate a bit.
    carry: bool,
    /// Set if the last operation resulted in a parity.
    /// The number of set bits in the result is even.
    parity: bool,
    /// Set if the last operation resulted in an auxiliary carry;
    /// there has been a carry from the low nibble to the high nibble,
    /// or a borrow from the high nibble to the low nibble is used by
    /// decimal arithmetic instructions
    auxiliary_carry: bool,
    /// Set if the last operation resulted in a zero
    zero: bool,
    /// Set if the last operation resulted in a negative number.
    /// The most significant bit of the result is set. true if the result is negative.
    sign: bool,
    /// Set if the last operation resulted in an overflow.
    /// That is a significant bit has been lost because the
    /// result was too large to fit in the destination location.
    ///
    /// An interrupt on overflow is used to trap the overflow condition.
    overflow: bool,
    /// Set if interrupts are enabled.
    /// If this flag is set, the CPU will recognize external/maskable
    /// interrupts.
    /// If this flag is cleared, the CPU will ignore interrupt requests.
    ///
    /// Has no effect on non-maskable interrupts, and internal interrupts.
    interrupt_enable: bool,
    /// Set if string operations should decrement the index registers.
    /// Used to determine the direction of string operations.
    /// true if the string operations should decrement the index registers (read from right to left).
    /// false if the string operations should increment the index registers (read from left to right)
    /// the normal direction
    direction: bool,
    /// Set if the CPU should enter a single-step mode
    ///
    /// When set, the CPU will generate an interrupt
    /// after the execution of each instruction.
    /// This is used by debuggers to step through a program one instruction at a time.
    trap: bool,
}

impl Flags {
    fn new(
        carry: bool,
        parity: bool,
        auxiliary_carry: bool,
        zero: bool,
        sign: bool,
        overflow: bool,
        interrupt_enable: bool,
        direction: bool,
        trap: bool,
    ) -> Self {
        Self {
            carry,
            parity,
            auxiliary_carry,
            zero,
            sign,
            overflow,
            interrupt_enable,
            direction,
            trap,
        }
    }

    pub fn set_carry(&mut self, value: bool) {
        self.carry = value;
    }
    pub fn set_parity(&mut self, value: bool) {
        self.parity = value;
    }
    pub fn set_auxiliary_carry(&mut self, value: bool) {
        self.auxiliary_carry = value;
    }
    pub fn set_zero(&mut self, value: bool) {
        self.zero = value;
    }
    pub fn set_sign(&mut self, value: bool) {
        self.sign = value;
    }
    pub fn set_overflow(&mut self, value: bool) {
        self.overflow = value;
    }
    pub fn set_interrupt_enable(&mut self, value: bool) {
        self.interrupt_enable = value;
    }
    pub fn set_direction(&mut self, value: bool) {
        self.direction = value;
    }
    pub fn set_trap(&mut self, value: bool) {
        self.trap = value;
    }

    pub fn get_carry(&self) -> bool {
        self.carry
    }
    pub fn get_parity(&self) -> bool {
        self.parity
    }
    pub fn get_auxiliary_carry(&self) -> bool {
        self.auxiliary_carry
    }
    pub fn get_zero(&self) -> bool {
        self.zero
    }
    pub fn get_sign(&self) -> bool {
        self.sign
    }
    pub fn get_overflow(&self) -> bool {
        self.overflow
    }
    pub fn get_interrupt_enable(&self) -> bool {
        self.interrupt_enable
    }
    pub fn get_direction(&self) -> bool {
        self.direction
    }
    pub fn get_trap(&self) -> bool {
        self.trap
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flags_initialization() {
        let flags = Flags::new(true, false, true, false, true, false, true, false, true);
        assert!(flags.get_carry());
        assert!(!flags.get_parity());
        assert!(flags.get_auxiliary_carry());
        assert!(!flags.get_zero());
        assert!(flags.get_sign());
        assert!(!flags.get_overflow());
        assert!(flags.get_interrupt_enable());
        assert!(!flags.get_direction());
        assert!(flags.get_trap());
    }

    #[test]
    fn test_set_and_get_carry() {
        let mut flags = Flags::new(
            false, false, false, false, false, false, false, false, false,
        );
        flags.set_carry(true);
        assert!(flags.get_carry());
    }

    #[test]
    fn test_set_and_get_parity() {
        let mut flags = Flags::new(
            false, false, false, false, false, false, false, false, false,
        );
        flags.set_parity(true);
        assert!(flags.get_parity());
    }

    #[test]
    fn test_set_and_get_auxiliary_carry() {
        let mut flags = Flags::new(
            false, false, false, false, false, false, false, false, false,
        );
        flags.set_auxiliary_carry(true);
        assert!(flags.get_auxiliary_carry());
    }

    #[test]
    fn test_set_and_get_zero() {
        let mut flags = Flags::new(
            false, false, false, false, false, false, false, false, false,
        );
        flags.set_zero(true);
        assert!(flags.get_zero());
    }

    #[test]
    fn test_set_and_get_sign() {
        let mut flags = Flags::new(
            false, false, false, false, false, false, false, false, false,
        );
        flags.set_sign(true);
        assert!(flags.get_sign());
    }

    #[test]
    fn test_set_and_get_overflow() {
        let mut flags = Flags::new(
            false, false, false, false, false, false, false, false, false,
        );
        flags.set_overflow(true);
        assert!(flags.get_overflow());
    }

    #[test]
    fn test_set_and_get_interrupt_enable() {
        let mut flags = Flags::new(
            false, false, false, false, false, false, false, false, false,
        );
        flags.set_interrupt_enable(true);
        assert!(flags.get_interrupt_enable());
    }

    #[test]
    fn test_set_and_get_direction() {
        let mut flags = Flags::new(
            false, false, false, false, false, false, false, false, false,
        );
        flags.set_direction(true);
        assert!(flags.get_direction());
    }

    #[test]
    fn test_set_and_get_trap() {
        let mut flags = Flags::new(
            false, false, false, false, false, false, false, false, false,
        );
        flags.set_trap(true);
        assert!(flags.get_trap());
    }
}
