pub mod biu;
pub mod bus;
pub mod eu;
pub mod flags;
pub mod memory;
pub mod registers;
/// Represents a 16-bit register with methods to access and modify the low and high bytes.

enum CPUModes {
    /// The cpu provide bus control signals needed for memory and I/O operations.
    Minimum,

    /// The cpu encodes control signals on 3 lines. An 8288 bus controller
    /// is required to convert these signals into the proper control signals.
    ///
    /// The remaining lines are used for a new set of signals to help coordinate
    /// multiple processors in a multiprocessor system.
    Maximum,
}

/// Represents the Intel 8086 CPU with its registers and segments.
struct CPU<'a> {
    /// Mode of the CPU
    /// The mode of the CPU determines the number of control lines used to interface with the system bus.
    ///
    /// The 8086 CPU can operate in two modes: Minimum mode and Maximum mode.
    /// In Minimum mode, the CPU provides the bus control signals needed for memory and I/O operations.
    /// In Maximum mode, the CPU encodes control signals on 3 lines, and an 8288 bus controller
    /// is required to convert these signals into the proper control signals.
    /// The remaining lines are used for a new set of signals to help coordinate
    /// multiple processors in a multiprocessor system.
    mode: CPUModes,

    eu: eu::ExecutionUnit,
    biu: biu::BusInterfaceUnit<'a>,
}
