use crate::{EntryValue, errors::Error, perf_data::DataType, safish_pointer::SafishPointer};

pub(crate) enum DataPointer {
    Boolean(SafishPointer<u8>),
    Byte(SafishPointer<u8>),
    Char(SafishPointer<char>),
    Short(SafishPointer<i16>),
    Int(SafishPointer<i32>),
    Long(SafishPointer<i64>),
    Float(SafishPointer<[u8; 4]>),
    Double(SafishPointer<[u8; 8]>),
    BooleanVec(SafishPointer<u8>, usize),
    ByteVec(SafishPointer<u8>, usize),
    CharVec(SafishPointer<char>, usize),
    ShortVec(SafishPointer<i16>, usize),
    IntVec(SafishPointer<i32>, usize),
    LongVec(SafishPointer<i64>, usize),
    FloatVec(SafishPointer<[u8; 4]>, usize),
    DoubleVec(SafishPointer<[u8; 8]>, usize),
}

impl DataPointer {
    pub(crate) fn new_scalar(data_type: &DataType, ptr: SafishPointer<u8>) -> Result<Self, Error> {
        Ok(match data_type {
            DataType::Boolean => Self::Boolean(ptr),
            DataType::Byte => Self::Byte(ptr),
            DataType::Char => Self::Char(ptr.convert::<char>()?),
            DataType::Short => Self::Short(ptr.convert::<i16>()?),
            DataType::Int => Self::Int(ptr.convert::<i32>()?),
            DataType::Long => Self::Long(ptr.convert::<i64>()?),
            DataType::Float => Self::Float(ptr.convert::<[u8; 4]>()?),
            DataType::Double => Self::Double(ptr.convert::<[u8; 8]>()?),
        })
    }

    pub(crate) fn new_vector(
        data_type: &DataType,
        length: usize,
        ptr: SafishPointer<u8>,
    ) -> Result<Self, Error> {
        Ok(match data_type {
            DataType::Boolean => Self::BooleanVec(ptr, length),
            DataType::Byte => Self::ByteVec(ptr, length),
            DataType::Char => Self::CharVec(ptr.convert::<char>()?, length),
            DataType::Short => Self::ShortVec(ptr.convert::<i16>()?, length),
            DataType::Int => Self::IntVec(ptr.convert::<i32>()?, length),
            DataType::Long => Self::LongVec(ptr.convert::<i64>()?, length),
            DataType::Float => Self::FloatVec(ptr.convert::<[u8; 4]>()?, length),
            DataType::Double => Self::DoubleVec(ptr.convert::<[u8; 8]>()?, length),
        })
    }

    pub(crate) fn read_value(&self) -> Result<EntryValue, Error> {
        EntryValue::new(self)
    }
}
