use crate::{
    ConstantEntry,
    data_pointer::DataPointer,
    entry_value::EntryValue,
    errors::Error,
    perf_data::{DataType, Unit, Variability},
    safish_pointer::SafishPointer,
    variable_data_reference::VariableDataReference,
};

/// /* The PerfDataEntry structure defines the fixed portion of an entry
///  * in the PerfData memory region. The PerfDataBuffer Java libraries
///  * are aware of this structure and need to be changed when this
///  * structure changes.
///  */
/// typedef struct {
///     jint entry_length;      // entry length in bytes
///     jint name_offset;       // offset of the data item name
///     jint vector_length;     // length of the vector. If 0, then scalar
///     jbyte data_type;        // type of the data item -
///                             // 'B','Z','J','I','S','C','D','F','V','L','['
///     jbyte flags;            // flags indicating misc attributes
///     jbyte data_units;       // unit of measure for the data type
///     jbyte data_variability; // variability classification of data type
///     jint  data_offset;      // offset of the data item
///   /*
///     body of PerfData memory entry is variable length
///     jbyte[name_length] data_name;        // name of the data item
///     jbyte[pad_length] data_pad;          // alignment of data item
///     j<data_type>[data_length] data_item; // array of appropriate types.
///                                          // data_length is > 1 only when the
///                                          // data_type is T_ARRAY.
///   */
///   } PerfDataEntry;
#[repr(C)]
#[derive(Debug)]
pub(crate) struct PerfDataEntryHeader {
    entry_length: i32,
    name_offset: i32,
    vector_length: i32,
    data_type: DataType, // JNI fireadeld descriptor type
    flags: Flag,
    data_units: Unit,
    data_variability: Variability,
    data_offset: u8, // offset to data item, relative to start of entry.
}

impl PerfDataEntryHeader {
    pub(crate) fn is_variable_entry(&self) -> bool {
        self.data_variability == Variability::Monotonic
            || self.data_variability == Variability::Variable
    }

    pub(crate) fn entry_length(&self) -> i32 {
        self.entry_length
    }

    pub(crate) fn read_variable_entry(
        &self,
        header_ptr: SafishPointer<PerfDataEntryHeader>,
    ) -> Result<VariableDataReference, Error> {
        let entry_ptr = header_ptr.convert()?;
        let variability = self.data_variability;
        let unit = self.data_units;
        let name = self.read_name(entry_ptr.clone().add(self.name_offset as usize)?)?;
        let data_ptr = entry_ptr.add(self.data_offset as usize)?;
        let data_pointer = if self.vector_length == 0 {
            DataPointer::new_scalar(&self.data_type, data_ptr)
        } else {
            DataPointer::new_vector(&self.data_type, self.vector_length as usize, data_ptr)
        }?;
        VariableDataReference::new(name, data_pointer, variability, unit)
    }

    pub(crate) fn read_constant_entry(
        &self,
        header_ptr: SafishPointer<PerfDataEntryHeader>,
    ) -> Result<ConstantEntry, Error> {
        let entry_ptr = header_ptr.convert()?;
        let name = self.read_name(entry_ptr.clone().add(self.name_offset as usize)?)?;
        let unit = self.data_units;
        let data_ptr = entry_ptr.add(self.data_offset as usize)?;
        let data_pointer = if self.vector_length == 0 {
            DataPointer::new_scalar(&self.data_type, data_ptr)?
        } else {
            DataPointer::new_vector(&self.data_type, self.vector_length as usize, data_ptr)?
        };
        let value = EntryValue::new(&data_pointer)?;
        let reference = ConstantEntry::new(name, value, unit);
        Ok(reference)
    }

    fn read_name(&self, ptr: SafishPointer<u8>) -> Result<String, Error> {
        ptr.read_string(self.data_offset as usize - self.name_offset as usize)
    }
}

/// // Miscellaneous flags
/// enum Flags {
///     F_None = 0x0,
///     F_Supported = 0x1    // interface is supported - java.* and com.sun.*
//   };
#[repr(u8)]
#[derive(Debug, Clone)]
#[allow(dead_code)]
enum Flag {
    None = 0,
    Supported = 1,
}

#[cfg(test)]
mod tests {
    use parameterized::parameterized;
    use test_strategy::proptest;

    use crate::{
        EntryValue,
        errors::Error,
        perf_data::{
            DataType, Endianness, Variability,
            perf_data_entry_header::{Flag, PerfDataEntryHeader},
            unit::Unit,
        },
        safish_pointer::SafishPointer,
    };

    fn given_a_header() -> PerfDataEntryHeader {
        PerfDataEntryHeader {
            entry_length: 0,
            name_offset: 0,
            vector_length: 0,
            data_type: DataType::Boolean,
            flags: Flag::None,
            data_units: Unit::Hertz,
            data_variability: Variability::Invalid,
            data_offset: 0 as u8,
        }
    }

    fn pack_binary_value(name: &str, value: u8) -> Vec<u8> {
        let mut binary_data: Vec<u8> = vec![0; (name.len() + 1) as usize];
        binary_data[..name.len()].copy_from_slice(name.as_bytes());
        binary_data[name.len()] = value;
        binary_data
    }

    fn pack_binary_array(name: &str, value: &str) -> Vec<u8> {
        let mut binary_data: Vec<u8> = vec![0; (name.len() + value.len()) as usize];
        binary_data[..name.len()].copy_from_slice(name.as_bytes());
        binary_data[name.len()..].copy_from_slice(value.as_bytes());
        binary_data
    }

    #[parameterized(variability = {
        Variability::Variable, Variability::Monotonic
    })]
    fn is_variable_entry(variability: Variability) {
        let mut tested_header = given_a_header();
        tested_header.data_variability = variability;

        assert!(tested_header.is_variable_entry());
    }

    #[parameterized(variability = {
        Variability::Invalid, Variability::Constant
    })]
    fn is_not_variable_entry(variability: Variability) {
        let mut tested_header = given_a_header();
        tested_header.data_variability = variability;

        assert!(!tested_header.is_variable_entry());
    }

    #[proptest]
    fn entry_length_returns_the_entry_length(entry_lemgth: i32) {
        let mut tested_header = given_a_header();
        tested_header.entry_length = entry_lemgth;

        assert_eq!(entry_lemgth, tested_header.entry_length())
    }

    #[proptest]
    fn name_is_read_from_the_pointer(
        #[strategy("[a-zA-Z.]{1, 10}")] name: String,
        #[strategy(0..100)] name_offset: i32,
    ) {
        let mut tested_header = given_a_header();
        tested_header.name_offset = name_offset;
        tested_header.data_offset = name_offset as u8 + name.len() as u8;

        let safish_pointer =
            SafishPointer::new(name.as_ptr(), name.len(), Endianness::BigEndian).unwrap();

        assert_eq!(name, tested_header.read_name(safish_pointer).unwrap());
    }

    #[test]
    fn read_constant_entry_populates_a_constant_scalar_entry() {
        let name = "test.vm";
        let value = 123;
        let unit = Unit::Ticks;
        let mut tested_header = given_a_header();
        tested_header.name_offset = 0;
        tested_header.data_offset = name.len() as u8;
        tested_header.data_type = DataType::Byte;
        tested_header.vector_length = 0;
        tested_header.data_units = unit;

        let binary_data = pack_binary_value(&name, value);

        // Fake the pointer
        let safish_pointer = SafishPointer::new(
            binary_data.as_ptr(),
            size_of::<PerfDataEntryHeader>() + name.len() + 1,
            Endianness::BigEndian,
        )
        .unwrap();

        let constant_entry = tested_header
            .read_constant_entry(safish_pointer.convert().unwrap())
            .unwrap();

        assert_eq!(name, constant_entry.name());
        match constant_entry.value() {
            EntryValue::Byte(content) => assert_eq!(*content, value),
            _ => assert!(false),
        }
        assert!(matches!(constant_entry.unit(), Unit::Ticks));
    }

    #[test]
    fn read_constant_entry_populates_a_constant_vector_entry() {
        let name = "test.vm";
        let value = "12";
        let unit = Unit::Ticks;
        let mut tested_header = given_a_header();
        tested_header.name_offset = 0;
        tested_header.data_offset = name.len() as u8;
        tested_header.data_type = DataType::Byte;
        tested_header.vector_length = 2;
        tested_header.data_units = unit;

        let binary_data = pack_binary_array(&name, &value);

        // Fake the pointer
        let safish_pointer = SafishPointer::new(
            binary_data.as_ptr(),
            size_of::<PerfDataEntryHeader>() + name.len() + 1,
            Endianness::BigEndian,
        )
        .unwrap();

        let constant_entry = tested_header
            .read_constant_entry(safish_pointer.convert().unwrap())
            .unwrap();

        assert_eq!(name, constant_entry.name());
        match constant_entry.value() {
            EntryValue::String(content) => assert_eq!(*content, value.to_string()),
            _ => assert!(false),
        }
        assert!(matches!(constant_entry.unit(), Unit::Ticks));
    }

    #[test]
    fn read_constant_entry_returns_err_when_pointer_too_short_for_name() {
        let name = "test.vm";
        let value = 123;
        let unit = Unit::Ticks;
        let mut tested_header = given_a_header();
        tested_header.name_offset = size_of::<PerfDataEntryHeader>() as i32;
        tested_header.data_offset = (size_of::<PerfDataEntryHeader>() + name.len()) as u8;
        tested_header.data_type = DataType::Byte;
        tested_header.vector_length = 0;
        tested_header.data_units = unit;

        let binary_data = pack_binary_value(&name, value);

        // Fake the pointer
        let safish_pointer = SafishPointer::new(
            binary_data.as_ptr(),
            size_of::<PerfDataEntryHeader>(),
            Endianness::BigEndian,
        )
        .unwrap();

        let error = tested_header
            .read_constant_entry(safish_pointer.convert().unwrap())
            .err()
            .unwrap();

        assert!(matches!(error, Error::WontBeAbleToRead));
    }

    #[test]
    fn read_variable_entry_populates_a_variable_scalar_data_reference() {
        let name = "test.vm";
        let value = 123;
        let unit = Unit::Ticks;
        let variability = Variability::Monotonic;
        let mut tested_header = given_a_header();
        tested_header.name_offset = 0;
        tested_header.data_offset = name.len() as u8;
        tested_header.data_type = DataType::Byte;
        tested_header.vector_length = 0;
        tested_header.data_units = unit;
        tested_header.data_variability = variability;

        let binary_data = pack_binary_value(&name, value);

        // Fake the pointer
        let safish_pointer = SafishPointer::new(
            binary_data.as_ptr(),
            size_of::<PerfDataEntryHeader>() + name.len() + 1,
            Endianness::BigEndian,
        )
        .unwrap();

        let mut variable_data_reference = tested_header
            .read_variable_entry(safish_pointer.convert().unwrap())
            .unwrap();
        let entry = variable_data_reference.refresh_entry().unwrap();

        assert_eq!(name, entry.name());
        assert!(matches!(entry.unit(), Unit::Ticks));
        assert!(matches!(entry.variability(), Variability::Monotonic));
        match entry.value() {
            EntryValue::Byte(content) => assert_eq!(*content, value),
            _ => assert!(false),
        }
    }

    #[test]
    fn read_variable_entry_populates_a_variable_vector_data_reference() {
        let name = "test.vm";
        let value = "12";
        let unit = Unit::Ticks;
        let variability = Variability::Monotonic;
        let mut tested_header = given_a_header();
        tested_header.name_offset = 0;
        tested_header.data_offset = name.len() as u8;
        tested_header.data_type = DataType::Byte;
        tested_header.vector_length = 2;
        tested_header.data_units = unit;
        tested_header.data_variability = variability;

        let binary_data = pack_binary_array(&name, &value);

        // Fake the pointer
        let safish_pointer = SafishPointer::new(
            binary_data.as_ptr(),
            size_of::<PerfDataEntryHeader>() + name.len() + 1,
            Endianness::BigEndian,
        )
        .unwrap();

        let mut variable_data_reference = tested_header
            .read_variable_entry(safish_pointer.convert().unwrap())
            .unwrap();
        let entry = variable_data_reference.refresh_entry().unwrap();

        assert_eq!(name, entry.name());
        match entry.value() {
            EntryValue::String(content) => assert_eq!(*content, value.to_string()),
            _ => assert!(false),
        }
        assert!(matches!(entry.unit(), Unit::Ticks));
        assert!(matches!(entry.variability(), Variability::Monotonic));
    }
}
