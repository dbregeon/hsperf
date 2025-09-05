use crate::{
    EntryValue, VariableEntry,
    data_pointer::DataPointer,
    errors::Error,
    perf_data::{Unit, Variability},
};

/// A pointer reference to a HotSpot data entry.
/// The value can be read as VariableEntry at any time
pub(crate) struct VariableDataReference {
    data_pointer: DataPointer,
    entry: VariableEntry,
}

impl VariableDataReference {
    pub(crate) fn new(
        name: String,
        data_pointer: DataPointer,
        variability: Variability,
        unit: Unit,
    ) -> Result<Self, Error> {
        let entry = VariableEntry::new(name, EntryValue::new(&data_pointer)?, variability, unit);
        Ok(Self {
            data_pointer,
            entry,
        })
    }

    pub(crate) fn name(&self) -> &str {
        self.entry.name()
    }

    /// Updates and returns the internal entry when its value has changed.
    pub(crate) fn refresh_entry(&mut self) -> Result<&VariableEntry, Error> {
        Ok(self.entry.refresh_value(self.data_pointer.read_value()?))
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        EntryValue,
        data_pointer::DataPointer,
        perf_data::{DataType, Endianness, Unit, Variability},
        safish_pointer::SafishPointer,
        variable_data_reference::VariableDataReference,
    };

    #[test]
    fn new_creates_an_entry() {
        let name = "test1".to_string();
        let value = 1234;
        let data_pointer = DataPointer::new_scalar(
            &DataType::Int,
            SafishPointer::new(&value as *const i32, 4, Endianness::BigEndian)
                .unwrap()
                .convert()
                .unwrap(),
        )
        .unwrap();
        let variability = Variability::Monotonic;
        let unit = Unit::Ticks;

        let tested_reference =
            VariableDataReference::new(name.clone(), data_pointer, variability, unit).unwrap();

        assert_eq!(name, *tested_reference.name());
        assert_eq!(name, *tested_reference.entry.name());
        assert_eq!(unit, tested_reference.entry.unit());
        assert_eq!(variability, tested_reference.entry.variability());
    }

    #[test]
    fn refresh_entry_returns_value() {
        let name = "test1".to_string();
        let initial_value = 1234i32;
        let new_value = 5678i32;

        // Create initial data and pointer
        let mut data = initial_value.to_be_bytes();
        let data_pointer = DataPointer::new_scalar(
            &DataType::Int,
            SafishPointer::new(
                data.as_ptr() as *const i32,
                size_of::<i32>(),
                Endianness::BigEndian,
            )
            .unwrap()
            .convert()
            .unwrap(),
        )
        .unwrap();

        let variability = Variability::Monotonic;
        let unit = Unit::Ticks;

        let mut tested_reference =
            VariableDataReference::new(name.clone(), data_pointer, variability, unit).unwrap();

        // First call should return Some since the value is initially NotSet
        let result = tested_reference.refresh_entry().unwrap();
        match result.value() {
            EntryValue::Int(content) => assert_eq!(*content, initial_value),
            _ => assert!(false),
        }
        // Create new data with different value
        data.copy_from_slice(&new_value.to_be_bytes());

        // Second call should return Some since the value changed
        let result = tested_reference.refresh_entry().unwrap();
        match result.value() {
            EntryValue::Int(content) => assert_eq!(*content, new_value),
            _ => assert!(false),
        }
    }

    #[test]
    fn refresh_entry_works_with_vector_data() {
        let name = "test_array".to_string();
        let initial_values = vec![1i32, 2, 3, 4];
        let new_values = vec![5i32, 6, 7, 8];

        // Create initial data
        let initial_data: Vec<u8> = initial_values
            .iter()
            .flat_map(|&v| v.to_be_bytes())
            .collect();
        let data_pointer = DataPointer::new_vector(
            &DataType::Int,
            4,
            SafishPointer::new(
                initial_data.as_ptr() as *const i32,
                4 * size_of::<i32>(),
                Endianness::BigEndian,
            )
            .unwrap()
            .convert()
            .unwrap(),
        )
        .unwrap();

        let variability = Variability::Variable;
        let unit = Unit::Events;

        let mut tested_reference =
            VariableDataReference::new(name.clone(), data_pointer, variability, unit).unwrap();

        // First call should return Some with initial vector
        let result = tested_reference.refresh_entry().unwrap();
        match result.value() {
            EntryValue::IntVec(content) => assert_eq!(*content, initial_values),
            _ => assert!(false),
        }

        // Create new data with different values
        let new_data: Vec<u8> = new_values.iter().flat_map(|&v| v.to_be_bytes()).collect();
        let new_data_pointer =
            SafishPointer::new(new_data.as_ptr(), new_data.len(), Endianness::BigEndian).unwrap();

        // Update the data pointer
        tested_reference.data_pointer =
            DataPointer::new_vector(&DataType::Int, 4, new_data_pointer).unwrap();

        // Second call should return Some since the vector changed
        let result = tested_reference.refresh_entry().unwrap();

        match result.value() {
            EntryValue::IntVec(content) => assert_eq!(*content, new_values),
            _ => assert!(false),
        }
    }

    #[test]
    fn refresh_entry_handles_different_data_types() {
        // Test with Double scalar
        let name = "test_double".to_string();
        let value = 3.14159f64;

        let data = value.to_be_bytes();
        let data_pointer = DataPointer::new_scalar(
            &DataType::Double,
            SafishPointer::new(
                data.as_ptr() as *const u8,
                size_of::<i64>(),
                Endianness::BigEndian,
            )
            .unwrap()
            .convert()
            .unwrap(),
        )
        .unwrap();

        let variability = Variability::Monotonic;
        let unit = Unit::Hertz;

        let mut tested_reference =
            VariableDataReference::new(name.clone(), data_pointer, variability, unit).unwrap();

        let result: &crate::variable_entry::VariableEntry =
            tested_reference.refresh_entry().unwrap();
        match result.value() {
            EntryValue::Double(3.14159f64) => assert!(true),
            _ => assert!(false),
        }
    }
}
