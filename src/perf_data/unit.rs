///     // the Units enum must be kept in synchronization with the
///     // the com.sun.hotspot.perfdata.Units class
///     enum Units {
///         U_None = 1,
///         U_Bytes = 2,
///         U_Ticks = 3,
///         U_Events = 4,
///         U_String = 5,
///         U_Hertz = 6,
///         U_Last = U_Hertz
///       };
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Unit {
    Invalid = 0,
    None = 1,
    Bytes = 2,
    Ticks = 3,
    Events = 4,
    String = 5,
    Hertz = 6,
}
