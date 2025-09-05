/// // the Variability enum must be kept in synchronization with the
///     // the com.sun.hotspot.perfdata.Variability class
///     enum Variability {
///         V_Constant = 1,
///         V_Monotonic = 2,
///         V_Variable = 3,
///         V_last = V_Variable
///       };
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Variability {
    Invalid = 0,
    Constant = 1,
    Monotonic = 2,
    Variable = 3,
}
