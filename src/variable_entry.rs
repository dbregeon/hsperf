use crate::{
    EntryValue,
    perf_data::{Unit, Variability},
};

/// Entry read from the Hotspot Data that is characterized as changing in either a Monotonic or Variable way
pub struct VariableEntry {
    name: String,
    value: EntryValue,
    variability: Variability,
    unit: Unit,
}

impl VariableEntry {
    pub(crate) fn new(
        name: String,
        value: EntryValue,
        variability: Variability,
        unit: Unit,
    ) -> Self {
        Self {
            name,
            value,
            variability,
            unit,
        }
    }

    /// The name of the entry as published by the JVM.
    pub fn name(&self) -> &String {
        &self.name
    }

    /// The last value read for the entry.
    pub fn value(&self) -> &EntryValue {
        &self.value
    }

    /// Updates the value when the new_value is different
    /// returns None when the new_value is the same as the existing value,
    /// Some of a reference to itself when the value was updated.
    pub fn refresh_value(&mut self, new_value: EntryValue) -> &Self {
        self.value = new_value;
        self
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
        perf_data::{Unit, Variability},
        variable_entry::VariableEntry,
    };

    #[test]
    fn name_returns_the_name() {
        let name = "test1".to_string();
        let value = EntryValue::Int(1234);
        let variability = Variability::Monotonic;
        let unit = Unit::Hertz;
        let tested_entry = VariableEntry::new(name.clone(), value, variability, unit);

        assert_eq!(name, *tested_entry.name());
    }

    #[test]
    fn value_returns_the_value() {
        let name = "test1".to_string();
        let value = EntryValue::Int(1234);
        let variability = Variability::Monotonic;
        let unit = Unit::Hertz;
        let tested_entry = VariableEntry::new(name, value, variability, unit);

        match tested_entry.value {
            EntryValue::Int(1234) => assert!(true),
            _ => assert!(false),
        }
    }

    #[test]
    fn variability_returns_the_variability() {
        let name = "test1".to_string();
        let value = EntryValue::Int(1234);
        let variability = Variability::Monotonic;
        let unit = Unit::Hertz;
        let tested_entry = VariableEntry::new(name, value, variability, unit);

        assert_eq!(variability, tested_entry.variability());
    }

    #[test]
    fn unit_returns_the_unit() {
        let name = "test1".to_string();
        let value = EntryValue::Int(1234);
        let variability = Variability::Monotonic;
        let unit = Unit::Hertz;
        let tested_entry = VariableEntry::new(name, value, variability, unit);

        assert_eq!(unit, tested_entry.unit());
    }

    #[test]
    fn refresh_value_is_self_when_changed() {
        let name = "test1".to_string();
        let value = EntryValue::Int(1234);
        let new_value = EntryValue::Int(1235);
        let variability = Variability::Monotonic;
        let unit = Unit::Hertz;
        let mut tested_entry = VariableEntry::new(name, value, variability, unit);

        let result = tested_entry.refresh_value(new_value);
        match result.value {
            EntryValue::Int(1235) => assert!(true),
            _ => assert!(false),
        }
    }
}
