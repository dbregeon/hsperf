use crate::{EntryValue, perf_data::Unit};

/// Entry read from the Hotspot Data that is characterized as unchanging
pub struct ConstantEntry {
    name: String,
    value: EntryValue,
    unit: Unit,
}

impl ConstantEntry {
    pub(crate) fn new(name: String, value: EntryValue, unit: Unit) -> Self {
        Self { name, value, unit }
    }

    /// The name of the entry as published by the JVM.
    pub fn name(&self) -> &String {
        &self.name
    }

    /// The (constant) value of the entry.
    pub fn value(&self) -> &EntryValue {
        &self.value
    }

    /// The unit of the entry as published by the JVM
    pub fn unit(&self) -> &Unit {
        &self.unit
    }
}

#[cfg(test)]
mod tests {
    use crate::{constant_entry::ConstantEntry, entry_value::EntryValue, perf_data::Unit};

    #[test]
    fn name_returns_the_name() {
        let name = "test1".to_string();
        let value = EntryValue::Int(1234);
        let unit = Unit::Hertz;
        let tested_entry = ConstantEntry::new(name.clone(), value, unit);

        assert_eq!(name, *tested_entry.name());
    }

    #[test]
    fn value_returns_the_value() {
        let name = "test1".to_string();
        let value = EntryValue::Int(1234);
        let unit = Unit::Hertz;
        let tested_entry = ConstantEntry::new(name, value, unit);

        match tested_entry.value() {
            EntryValue::Int(1234) => assert!(true),
            _ => assert!(false),
        }
    }

    #[test]
    fn unit_returns_the_unit() {
        let name = "test1".to_string();
        let value = EntryValue::Int(1234);
        let unit = Unit::Hertz;
        let tested_entry = ConstantEntry::new(name, value, unit);

        assert_eq!(unit, *tested_entry.unit());
    }
}
