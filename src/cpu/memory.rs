pub struct Memory {
    /// The memory array that stores the data.
    /// 1 Mb of memory (0x0000_0000 - 0x000F_FFFF)
    /// The memory array is indexed by a 16-bit (word) address.
    /// Stored in little-endian format. (least significant byte first)
    data: Vec<u8>,
}

impl Memory {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }
    pub fn read(&self, address: u32) -> u8 {
        self.data[address as usize]
    }

    pub fn write(&mut self, address: u32, value: u8) {
        self.data[address as usize] = value;
    }
}
