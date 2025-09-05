#[repr(u8)]
#[derive(Debug)]
#[allow(dead_code)]
pub(crate) enum DataType {
    Boolean = 'Z' as u8,
    Byte = 'B' as u8,
    Char = 'C' as u8,
    Short = 'S' as u8,
    Int = 'I' as u8,
    Long = 'J' as u8,
    Float = 'F' as u8,
    Double = 'D' as u8,
}
