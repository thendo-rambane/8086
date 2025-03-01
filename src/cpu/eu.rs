use super::{flags, registers};

/// Represents the Execution Unit (EU) of the CPU, which is responsible for executing instructions.
pub struct ExecutionUnit {
    /// Accumulator register
    a: registers::Register,
    /// Base register
    b: registers::Register,
    /// Count register
    c: registers::Register,
    /// Data register
    d: registers::Register,

    /// Stack Pointer; points to the top of the stack
    sp: u16,
    /// Base Pointer; points to the base of the current stack frame
    bp: u16,
    /// Source Index; used for string operations
    si: u16,
    /// Destination Index; used for string operations
    di: u16,

    /// Flags register
    /// The flags register contains various status flags that are set or cleared based on the result of an operation.
    flags: flags::Flags,
}

impl ExecutionUnit {
    fn new(
        a: registers::Register,
        b: registers::Register,
        c: registers::Register,
        d: registers::Register,
        sp: u16,
        bp: u16,
        si: u16,
        di: u16,
        flags: flags::Flags,
    ) -> Self {
        Self {
            a,
            b,
            c,
            d,
            sp,
            bp,
            si,
            di,
            flags,
        }
    }

    pub fn set_sp(&mut self, value: u16) {
        self.sp = value;
    }
    pub fn set_bp(&mut self, value: u16) {
        self.bp = value;
    }
    pub fn set_si(&mut self, value: u16) {
        self.si = value;
    }
    pub fn set_di(&mut self, value: u16) {
        self.di = value;
    }

    pub fn get_sp(&self) -> u16 {
        self.sp
    }
    pub fn get_bp(&self) -> u16 {
        self.bp
    }
    pub fn get_si(&self) -> u16 {
        self.si
    }
    pub fn get_di(&self) -> u16 {
        self.di
    }
}
#[cfg(test)]
mod tests {
    use super::super::flags::Flags;
    use super::super::registers::Register;
    use super::*;

    #[test]
    fn test_set_and_get_sp() {
        let mut eu = ExecutionUnit::new(
            Register::default(),
            Register::default(),
            Register::default(),
            Register::default(),
            0,
            0,
            0,
            0,
            Flags::default(),
        );
        eu.set_sp(0x1234);
        assert_eq!(eu.get_sp(), 0x1234);
    }

    #[test]
    fn test_set_and_get_bp() {
        let mut eu = ExecutionUnit::new(
            Register::default(),
            Register::default(),
            Register::default(),
            Register::default(),
            0,
            0,
            0,
            0,
            Flags::default(),
        );
        eu.set_bp(0x5678);
        assert_eq!(eu.get_bp(), 0x5678);
    }

    #[test]
    fn test_set_and_get_si() {
        let mut eu = ExecutionUnit::new(
            Register::default(),
            Register::default(),
            Register::default(),
            Register::default(),
            0,
            0,
            0,
            0,
            Flags::default(),
        );
        eu.set_si(0x9ABC);
        assert_eq!(eu.get_si(), 0x9ABC);
    }

    #[test]
    fn test_set_and_get_di() {
        let mut eu = ExecutionUnit::new(
            Register::default(),
            Register::default(),
            Register::default(),
            Register::default(),
            0,
            0,
            0,
            0,
            Flags::default(),
        );
        eu.set_di(0xDEF0);
        assert_eq!(eu.get_di(), 0xDEF0);
    }
}
