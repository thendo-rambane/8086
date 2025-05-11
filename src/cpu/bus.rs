use super::memory::Memory;

#[derive(Default, Debug)]
pub struct AddressBus {
    address: u32,
    memory: Memory,
}

impl AddressBus {
    pub fn new() -> Self {
        Self {
            address: 0,
            memory: Memory::new(),
        }
    }

    pub fn set_address(&mut self, address: u32) {
        self.address = address;
    }

    pub fn read(&self) -> u8 {
        self.memory.read(self.address)
    }

    pub fn write(&mut self, value: u8) {
        self.memory.write(self.address, value);
    }
}
