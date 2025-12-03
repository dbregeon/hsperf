use crate::{EntryValue, perf_data::Unit};

/// Entry read from the Hotspot Data that is characterized as unchanging
pub struct ConstantEntry {
    value: EntryValue,
    unit: Unit,
}

impl ConstantEntry {
    pub(crate) fn new(value: EntryValue, unit: Unit) -> Self {
        Self { value, unit }
    }

    /// The (constant) value of the entry.
    pub fn value(&self) -> EntryValue {
        self.value.clone()
    }

    /// The unit of the entry as published by the JVM
    pub fn unit(&self) -> Unit {
        self.unit
    }
}

#[cfg(test)]
mod tests {
    use crate::{constant_entry::ConstantEntry, entry_value::EntryValue, perf_data::Unit};

    #[test]
    fn value_returns_the_value() {
        let value = EntryValue::Int(1234);
        let unit = Unit::Hertz;
        let tested_entry = ConstantEntry::new(value, unit);

        match tested_entry.value() {
            EntryValue::Int(1234) => assert!(true),
            _ => assert!(false),
        }
    }

    #[test]
    fn unit_returns_the_unit() {
        let value = EntryValue::Int(1234);
        let unit = Unit::Hertz;
        let tested_entry = ConstantEntry::new(value, unit);

        assert_eq!(unit, tested_entry.unit());
    }
}
