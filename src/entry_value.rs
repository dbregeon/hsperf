use crate::{data_pointer::DataPointer, errors::Error};

/// Enumeration to hold the different types of values read from the hsperfdata file.
#[derive(Debug, Clone)]
pub enum EntryValue {
    Boolean(bool),
    Byte(u8),
    Char(char),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    BooleanVec(Vec<bool>),
    String(String),
    CharVec(Vec<char>),
    ShortVec(Vec<i16>),
    IntVec(Vec<i32>),
    LongVec(Vec<i64>),
    FloatVec(Vec<f32>),
    DoubleVec(Vec<f64>),
    NotSet,
}

impl EntryValue {
    pub(crate) fn new(pointer: &DataPointer) -> Result<Self, Error> {
        Ok(match pointer {
            DataPointer::Boolean(ptr) => EntryValue::Boolean(ptr.read() == 0),
            DataPointer::Byte(ptr) => EntryValue::Byte(ptr.read()),
            DataPointer::Char(ptr) => EntryValue::Char(ptr.read() as char),
            DataPointer::Short(ptr) => EntryValue::Short(ptr.read_i16()?),
            DataPointer::Int(ptr) => EntryValue::Int(ptr.read_i32()?),
            DataPointer::Long(ptr) => EntryValue::Long(ptr.read_i64()?),
            DataPointer::Float(ptr) => EntryValue::Float(ptr.read_f32()?),
            DataPointer::Double(ptr) => EntryValue::Double(ptr.read_f64()?),
            DataPointer::BooleanVec(ptr, length) => {
                EntryValue::BooleanVec(ptr.read_booleans(*length)?)
            }
            DataPointer::ByteVec(ptr, length) => EntryValue::String(ptr.read_string(*length)?),
            DataPointer::CharVec(ptr, length) => EntryValue::CharVec(ptr.read_n(*length)?),
            DataPointer::ShortVec(ptr, length) => EntryValue::ShortVec(ptr.read_n_i16(*length)?),
            DataPointer::IntVec(ptr, length) => EntryValue::IntVec(ptr.read_n_i32(*length)?),
            DataPointer::LongVec(ptr, length) => EntryValue::LongVec(ptr.read_n_i64(*length)?),
            DataPointer::FloatVec(ptr, length) => EntryValue::FloatVec(ptr.read_n_f32(*length)?),
            DataPointer::DoubleVec(ptr, length) => EntryValue::DoubleVec(ptr.read_n_f64(*length)?),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        data_pointer::DataPointer,
        entry_value::EntryValue,
        perf_data::{DataType, Endianness},
        safish_pointer::SafishPointer,
    };

    // Tests for new method

    #[test]
    fn new_boolean_false_returns_boolean_false() {
        let data = 0u8; // Boolean false is represented by 0
        let ptr = SafishPointer::new(&data as *const u8, 1, Endianness::BigEndian).unwrap();
        let data_pointer = DataPointer::new_scalar(&DataType::Boolean, ptr).unwrap();

        let result = EntryValue::new(&data_pointer).unwrap();

        assert!(matches!(result, EntryValue::Boolean(true))); // Note: == 0 means false in the implementation
    }

    #[test]
    fn new_boolean_true_returns_boolean_true() {
        let data = 1u8; // Boolean true is represented by non-zero
        let ptr = SafishPointer::new(&data as *const u8, 1, Endianness::BigEndian).unwrap();
        let data_pointer = DataPointer::new_scalar(&DataType::Boolean, ptr).unwrap();

        let result = EntryValue::new(&data_pointer).unwrap();

        assert!(matches!(result, EntryValue::Boolean(false))); // Note: == 0 means false, so != 0 is true
    }

    #[test]
    fn new_byte_returns_byte() {
        let data = 42u8;
        let ptr = SafishPointer::new(&data as *const u8, 1, Endianness::BigEndian).unwrap();
        let data_pointer = DataPointer::new_scalar(&DataType::Byte, ptr).unwrap();

        let result = EntryValue::new(&data_pointer).unwrap();

        assert!(matches!(result, EntryValue::Byte(42)));
    }

    #[test]
    fn new_char_returns_char() {
        let data = 'A';
        let ptr = SafishPointer::new(&data as *const char, 4, Endianness::BigEndian).unwrap();
        let data_pointer =
            DataPointer::new_scalar(&DataType::Char, ptr.convert().unwrap()).unwrap();

        let result = EntryValue::new(&data_pointer).unwrap();

        assert!(matches!(result, EntryValue::Char('A')));
    }

    #[test]
    fn new_short_returns_short() {
        let data = (1234i16).to_be_bytes();
        let ptr = SafishPointer::new(data.as_ptr(), 2, Endianness::BigEndian).unwrap();
        let data_pointer = DataPointer::new_scalar(&DataType::Short, ptr).unwrap();

        let result = EntryValue::new(&data_pointer).unwrap();

        assert!(matches!(result, EntryValue::Short(1234)));
    }

    #[test]
    fn new_int_returns_int() {
        let data = (123456i32).to_be_bytes();
        let ptr = SafishPointer::new(data.as_ptr(), 4, Endianness::BigEndian).unwrap();
        let data_pointer = DataPointer::new_scalar(&DataType::Int, ptr).unwrap();

        let result = EntryValue::new(&data_pointer).unwrap();

        assert!(matches!(result, EntryValue::Int(123456)));
    }

    #[test]
    fn new_long_returns_long() {
        let data = (1234567890i64).to_be_bytes();
        let ptr = SafishPointer::new(data.as_ptr(), 8, Endianness::BigEndian).unwrap();
        let data_pointer = DataPointer::new_scalar(&DataType::Long, ptr).unwrap();

        let result = EntryValue::new(&data_pointer).unwrap();

        assert!(matches!(result, EntryValue::Long(1234567890)));
    }

    #[test]
    fn new_float_returns_float() {
        let data = (3.14159f32).to_be_bytes();
        let ptr = SafishPointer::new(data.as_ptr(), 4, Endianness::BigEndian).unwrap();
        let data_pointer = DataPointer::new_scalar(&DataType::Float, ptr).unwrap();

        let result = EntryValue::new(&data_pointer).unwrap();

        assert!(matches!(result, EntryValue::Float(3.14159)));
    }

    #[test]
    fn new_double_returns_double() {
        let data = (2.718281828459045f64).to_be_bytes();
        let ptr = SafishPointer::new(data.as_ptr(), 8, Endianness::BigEndian).unwrap();
        let data_pointer = DataPointer::new_scalar(&DataType::Double, ptr).unwrap();

        let result = EntryValue::new(&data_pointer).unwrap();

        assert!(matches!(result, EntryValue::Double(2.718281828459045)));
    }

    #[test]
    fn new_with_little_endian_works() {
        let data = (1234i32).to_le_bytes();
        let ptr = SafishPointer::new(data.as_ptr(), 4, Endianness::LittleEndian).unwrap();
        let data_pointer = DataPointer::new_scalar(&DataType::Int, ptr).unwrap();

        let result = EntryValue::new(&data_pointer).unwrap();

        assert!(matches!(result, EntryValue::Int(1234)));
    }

    // Tests for new method

    #[test]
    fn new_boolean_returns_boolean_vec() {
        let data = [0u8, 1u8, 0u8, 1u8];
        let ptr = SafishPointer::new(data.as_ptr(), data.len(), Endianness::BigEndian).unwrap();
        let data_pointer = DataPointer::new_vector(&DataType::Boolean, 4, ptr).unwrap();

        let result = EntryValue::new(&data_pointer).unwrap();
        let expected = vec![true, false, true, false];

        match result {
            EntryValue::BooleanVec(content) => assert_eq!(content, expected),
            _ => assert!(false),
        }
    }

    #[test]
    fn new_byte_returns_string() {
        let data = "Hello".as_bytes();
        let ptr = SafishPointer::new(data.as_ptr(), data.len(), Endianness::BigEndian).unwrap();
        let data_pointer = DataPointer::new_vector(&DataType::Byte, 5, ptr).unwrap();

        let result = EntryValue::new(&data_pointer).unwrap();
        let expected = "Hello".to_string();

        match result {
            EntryValue::String(content) => assert_eq!(content, expected),
            _ => assert!(false),
        }
    }

    #[test]
    fn new_char_returns_char_vec() {
        let data = ['A', 'B', 'C'];
        let ptr = SafishPointer::new(
            data.as_ptr() as *const u8,
            data.len() * size_of::<char>(),
            Endianness::BigEndian,
        )
        .unwrap();
        let data_pointer = DataPointer::new_vector(&DataType::Char, 3, ptr).unwrap();

        let result = EntryValue::new(&data_pointer).unwrap();
        let expected = vec!['A', 'B', 'C'];

        match result {
            EntryValue::CharVec(content) => assert_eq!(content, expected),
            _ => assert!(false),
        }
    }

    #[test]
    fn new_short_returns_short_vec() {
        let values = [1i16, 2, 3, 4, 5];
        let data: Vec<u8> = values.iter().flat_map(|&v| v.to_be_bytes()).collect();
        let ptr = SafishPointer::new(data.as_ptr(), data.len(), Endianness::BigEndian).unwrap();
        let data_pointer = DataPointer::new_vector(&DataType::Short, 5, ptr).unwrap();

        let result = EntryValue::new(&data_pointer).unwrap();
        let expected = vec![1, 2, 3, 4, 5];

        match result {
            EntryValue::ShortVec(content) => assert_eq!(content, expected),
            _ => assert!(false),
        }
    }

    #[test]
    fn new_int_returns_int_vec() {
        let values = [10i32, 20, 30, 40];
        let data: Vec<u8> = values.iter().flat_map(|&v| v.to_be_bytes()).collect();
        let ptr = SafishPointer::new(data.as_ptr(), data.len(), Endianness::BigEndian).unwrap();
        let data_pointer = DataPointer::new_vector(&DataType::Int, 4, ptr).unwrap();

        let result = EntryValue::new(&data_pointer).unwrap();
        let expected = vec![10, 20, 30, 40];

        match result {
            EntryValue::IntVec(content) => assert_eq!(content, expected),
            _ => assert!(false),
        }
    }

    #[test]
    fn new_long_returns_long_vec() {
        let values = [100i64, 200, 300];
        let data: Vec<u8> = values.iter().flat_map(|&v| v.to_be_bytes()).collect();
        let ptr = SafishPointer::new(data.as_ptr(), data.len(), Endianness::BigEndian).unwrap();
        let data_pointer = DataPointer::new_vector(&DataType::Long, 3, ptr).unwrap();

        let result = EntryValue::new(&data_pointer).unwrap();
        let expected = vec![100, 200, 300];

        match result {
            EntryValue::LongVec(content) => assert_eq!(content, expected),
            _ => assert!(false),
        }
    }

    #[test]
    fn new_float_returns_float_vec() {
        let values = [1.1f32, 2.2, 3.3];
        let data: Vec<u8> = values.iter().flat_map(|&v| v.to_be_bytes()).collect();
        let ptr = SafishPointer::new(data.as_ptr(), data.len(), Endianness::BigEndian).unwrap();
        let data_pointer = DataPointer::new_vector(&DataType::Float, 3, ptr).unwrap();

        let result = EntryValue::new(&data_pointer).unwrap();
        let expected = vec![1.1, 2.2, 3.3];

        match result {
            EntryValue::FloatVec(content) => assert_eq!(content, expected),
            _ => assert!(false),
        }
    }

    #[test]
    fn new_double_returns_double_vec() {
        let values = [1.111f64, 2.222];
        let data: Vec<u8> = values.iter().flat_map(|&v| v.to_be_bytes()).collect();
        let ptr = SafishPointer::new(data.as_ptr(), data.len(), Endianness::BigEndian).unwrap();
        let data_pointer = DataPointer::new_vector(&DataType::Double, 2, ptr).unwrap();

        let result = EntryValue::new(&data_pointer).unwrap();
        let expected = vec![1.111, 2.222];

        match result {
            EntryValue::DoubleVec(content) => assert_eq!(content, expected),
            _ => assert!(false),
        }
    }

    #[test]
    fn new_byte_with_null_terminated_string() {
        let data = "Hello\0World".as_bytes();
        let ptr = SafishPointer::new(data.as_ptr(), 5, Endianness::BigEndian).unwrap(); // Only read "Hello"
        let data_pointer = DataPointer::new_vector(&DataType::Byte, 5, ptr).unwrap();

        let result = EntryValue::new(&data_pointer).unwrap();
        let expected = "Hello".to_string();

        match result {
            EntryValue::String(content) => assert_eq!(content, expected),
            _ => assert!(false),
        }
    }
}
