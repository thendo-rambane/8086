use super::bus;
// use crate::bus::AddressBus;

/// Represents the Bus Interface Unit (BIU) of the CPU, which is responsible for interfacing with the system bus.
pub struct BusInterfaceUnit<'bus> {
    /// Extra Segment; points to an additional segment of memory
    es: u16,
    /// Code Segment; points to the segment of memory containing the current program
    cs: u16,
    /// Stack Segment; points to the segment of memory containing the current stack
    ss: u16,
    /// Data Segment; points to the segment of memory containing the current data
    ds: u16,
    /// Instruction Pointer; points to the next instruction to be executed
    ip: u16,

    /// Queue of bytes to be read from memory
    instruction_queue: Vec<u8>,
    bus: &'bus mut bus::AddressBus,
}

impl<'a> BusInterfaceUnit<'a> {
    pub fn new(
        es: u16,
        cs: u16,
        ss: u16,
        ds: u16,
        ip: u16,
        instruction_queue: Vec<u8>,
        bus: &'a mut bus::AddressBus,
    ) -> Self {
        Self {
            es,
            cs,
            ss,
            ds,
            ip,
            instruction_queue,
            bus,
        }
    }

    pub fn set_extra_segment_address(&mut self, value: u16) {
        self.es = value;
    }
    pub fn get_extra_segment_address(&self) -> u16 {
        self.es
    }

    pub fn set_code_segment_address(&mut self, value: u16) {
        self.cs = value;
    }
    pub fn get_code_segment_address(&self) -> u16 {
        self.cs
    }

    pub fn set_stack_segment_address(&mut self, value: u16) {
        self.ss = value;
    }
    pub fn get_stack_segment_address(&self) -> u16 {
        self.ss
    }

    pub fn set_data_segment_address(&mut self, value: u16) {
        self.ds = value;
    }
    pub fn get_data_segment_address(&self) -> u16 {
        self.ds
    }

    pub fn set_instruction_pointer(&mut self, value: u16) {
        self.ip = value;
    }
    pub fn get_instruction_pointer(&self) -> u16 {
        self.ip
    }

    pub fn push_instruction(&mut self, instruction: u8) {
        self.instruction_queue.push(instruction);
    }
    pub fn pop_instruction(&mut self) -> Option<u8> {
        self.instruction_queue.pop()
    }

    pub fn get_fetch_address(&self) -> u32 {
        ((self.cs as u32) << 4) + self.ip as u32
    }

    pub fn get_stack_address(&self, sp_offset: u16) -> u32 {
        ((self.ss as u32) << 4) + sp_offset as u32
    }

    pub fn get_string_source_address(&self, si_offset: u16, alt_base: Option<u16>) -> u32 {
        let base = alt_base.unwrap_or(self.ds);
        ((base as u32) << 4) + si_offset as u32
    }

    pub fn get_string_destination_address(&self, di_offset: u16) -> u32 {
        ((self.es as u32) << 4) + di_offset as u32
    }

    pub fn get_data_address(&self, eu_offset: u16, alt_base: Option<u16>) -> u32 {
        let base = alt_base.unwrap_or(self.ds);
        ((base as u32) << 4) + eu_offset as u32
    }

    pub fn get_bp_address(&self, eu_bp_offset: u16, alt_base: Option<u16>) -> u32 {
        let base = alt_base.unwrap_or(self.ss);
        ((base as u32) << 4) + eu_bp_offset as u32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_and_get_extra_segment_address() {
        let mut bus = bus::AddressBus::new();
        let mut biu = BusInterfaceUnit::new(0, 0, 0, 0, 0, vec![], &mut bus);
        biu.set_extra_segment_address(0x1234);
        assert_eq!(biu.get_extra_segment_address(), 0x1234);
    }

    #[test]
    fn test_set_and_get_code_segment_address() {
        let mut bus = bus::AddressBus::new();
        let mut biu = BusInterfaceUnit::new(0, 0, 0, 0, 0, vec![], &mut bus);
        biu.set_code_segment_address(0x5678);
        assert_eq!(biu.get_code_segment_address(), 0x5678);
    }

    #[test]
    fn test_set_and_get_stack_segment_address() {
        let mut bus = bus::AddressBus::new();
        let mut biu = BusInterfaceUnit::new(0, 0, 0, 0, 0, vec![], &mut bus);
        biu.set_stack_segment_address(0x9ABC);
        assert_eq!(biu.get_stack_segment_address(), 0x9ABC);
    }

    #[test]
    fn test_set_and_get_data_segment_address() {
        let mut bus = bus::AddressBus::new();
        let mut biu = BusInterfaceUnit::new(0, 0, 0, 0, 0, vec![], &mut bus);
        biu.set_data_segment_address(0xDEF0);
        assert_eq!(biu.get_data_segment_address(), 0xDEF0);
    }

    #[test]
    fn test_set_and_get_instruction_pointer() {
        let mut bus = bus::AddressBus::new();
        let mut biu = BusInterfaceUnit::new(0, 0, 0, 0, 0, vec![], &mut bus);
        biu.set_instruction_pointer(0x1357);
        assert_eq!(biu.get_instruction_pointer(), 0x1357);
    }

    #[test]
    fn test_push_and_pop_instruction() {
        let mut bus = bus::AddressBus::new();
        let mut biu = BusInterfaceUnit::new(0, 0, 0, 0, 0, vec![], &mut bus);
        biu.push_instruction(0x42);
        assert_eq!(biu.pop_instruction(), Some(0x42));
    }

    #[test]
    fn test_get_fetch_address() {
        // Given
        let cs = 0x1005;
        let ip = 0x5555;
        let mut bus = bus::AddressBus::new();
        let biu = BusInterfaceUnit::new(0, cs, 0, 0, ip, vec![], &mut bus);
        // When
        let expected = 0x1_55A5; // ((cs << 4) + ip) as u32;
        let actual = biu.get_fetch_address();
        // Then
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_get_stack_address() {
        let mut bus = bus::AddressBus::new();
        let biu = BusInterfaceUnit::new(0, 0, 0x2000, 0, 0, vec![], &mut bus);
        assert_eq!(biu.get_stack_address(0x300), 0x20300);
    }

    #[test]
    fn test_get_string_source_address() {
        let mut bus = bus::AddressBus::new();
        let biu = BusInterfaceUnit::new(0, 0, 0, 0x3000, 0, vec![], &mut bus);
        assert_eq!(biu.get_string_source_address(0x400, None), 0x30400);
    }

    #[test]
    fn test_get_string_destination_address() {
        let mut bus = bus::AddressBus::new();
        let biu = BusInterfaceUnit::new(0x4000, 0, 0, 0, 0, vec![], &mut bus);
        assert_eq!(biu.get_string_destination_address(0x500), 0x40500);
    }

    #[test]
    fn test_get_data_address() {
        let mut bus = bus::AddressBus::new();
        let biu = BusInterfaceUnit::new(0, 0, 0, 0x5000, 0, vec![], &mut bus);
        assert_eq!(biu.get_data_address(0x600, None), 0x50600);
    }

    #[test]
    fn test_get_bp_address() {
        let mut bus = bus::AddressBus::new();
        let biu = BusInterfaceUnit::new(0, 0, 0x7000, 0, 0, vec![], &mut bus);
        assert_eq!(biu.get_bp_address(0x800, None), 0x70800);
    }
}
