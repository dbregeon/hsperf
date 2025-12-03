use crate::{
    EntryValue,
    data_pointer::DataPointer,
    errors::Error,
    perf_data::{Unit, Variability},
};

/// Entry read from the Hotspot Data that is characterized as changing in either a Monotonic or Variable way
pub struct VariableEntry {
    data_pointer: DataPointer,
    variability: Variability,
    unit: Unit,
}

impl VariableEntry {
    pub(crate) fn new(data_pointer: DataPointer, variability: Variability, unit: Unit) -> Self {
        Self {
            data_pointer,
            variability,
            unit,
        }
    }

    /// The value read for the entry.
    pub fn value(&self) -> Result<EntryValue, Error> {
        self.data_pointer.read_value()
    }

    /// The variability of the entry as published by the JVM
    pub fn variability(&self) -> Variability {
        self.variability
    }

    /// The unit of the entry as published by the JVM
    pub fn unit(&self) -> Unit {
        self.unit
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        EntryValue,
        data_pointer::DataPointer,
        perf_data::{Endianness, PerfDataEntryHeader, Unit, Variability},
        safish_pointer::SafishPointer,
        variable_entry::VariableEntry,
    };

    fn given_a_pointer(name: &str) -> DataPointer {
        let value = 123;

        // Fake the pointer
        DataPointer::Byte(
            SafishPointer::new(
                &value,
                size_of::<PerfDataEntryHeader>() + name.len() + 1,
                Endianness::BigEndian,
            )
            .unwrap(),
        )
    }

    #[test]
    fn value_returns_the_value() {
        let name = "test1".to_string();
        let data_pointer = given_a_pointer(&name);
        let variability = Variability::Monotonic;
        let unit = Unit::Hertz;
        let tested_entry = VariableEntry::new(data_pointer, variability, unit);

        match tested_entry.value().unwrap() {
            EntryValue::Byte(x) => assert_eq!(x, 123),
            _ => assert!(false),
        }
    }

    #[test]
    fn variability_returns_the_variability() {
        let name = "test1".to_string();
        let data_pointer = given_a_pointer(&name);
        let variability = Variability::Monotonic;
        let unit = Unit::Hertz;
        let tested_entry = VariableEntry::new(data_pointer, variability, unit);

        assert_eq!(variability, tested_entry.variability());
    }

    #[test]
    fn unit_returns_the_unit() {
        let name = "test1".to_string();
        let data_pointer = given_a_pointer(&name);
        let variability = Variability::Monotonic;
        let unit = Unit::Hertz;
        let tested_entry = VariableEntry::new(data_pointer, variability, unit);

        assert_eq!(unit, tested_entry.unit());
    }
}
