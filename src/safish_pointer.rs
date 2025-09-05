use std::mem::transmute;

use crate::{errors::Error, perf_data::Endianness};

#[derive(Clone)]
pub(crate) struct SafishPointer<T: Sized> {
    ptr: *const T,
    max_byte_offset: usize,
    endianness: Endianness,
}

impl<T: Sized> SafishPointer<T> {
    pub(crate) fn new(
        ptr: *const T,
        max_byte_offset: usize,
        endianness: Endianness,
    ) -> Result<Self, Error> {
        if size_of::<T>() > max_byte_offset {
            Err(Error::WontBeAbleToRead)
        } else {
            Ok(Self {
                ptr,
                max_byte_offset,
                endianness,
            })
        }
    }

    pub(crate) fn read(&self) -> T {
        unsafe { self.ptr.read_volatile() }
    }

    fn validate_room_for(&self, count: usize) -> Result<(), Error> {
        let required_size = count * size_of::<T>();
        if self.max_byte_offset < required_size {
            Err(Error::WontBeAbleToRead)
        } else {
            Ok(())
        }
    }

    pub(crate) fn read_n(&self, count: usize) -> Result<Vec<T>, Error> {
        self.validate_room_for(count)?;
        let mut result = Vec::with_capacity(count);
        for i in 0..count {
            result.push(unsafe { self.ptr.add(i).read_volatile() });
        }
        Ok(result)
    }

    pub(crate) fn add(mut self, offset: usize) -> Result<Self, Error> {
        let byte_offset = (offset + 1) * size_of::<T>();
        if self.max_byte_offset < byte_offset {
            Err(Error::WontBeAbleToRead)
        } else {
            self.ptr = unsafe { self.ptr.add(offset) };
            self.max_byte_offset = self.max_byte_offset - offset * size_of::<T>();
            Ok(self)
        }
    }

    pub(crate) fn convert<S: Sized>(self) -> Result<SafishPointer<S>, Error> {
        if size_of::<S>() > self.max_byte_offset {
            Err(Error::WontBeAbleToRead)
        } else if self.ptr.align_offset(align_of::<S>()) == 0 {
            Ok(unsafe { transmute(self) })
        } else {
            Err(Error::NotAlignedForCOnversion)
        }
    }
}

impl SafishPointer<u8> {
    pub(crate) fn read_string(&self, count: usize) -> Result<String, Error> {
        self.validate_room_for(count)?;

        let mut result = String::with_capacity(count);
        for i in 0..count {
            let c = unsafe { self.ptr.add(i).read_volatile() as char };
            if c.is_ascii_control() {
                break;
            } else {
                result.push(c);
            }
        }
        Ok(result)
    }

    pub(crate) fn read_booleans(&self, count: usize) -> Result<Vec<bool>, Error> {
        self.validate_room_for(count)?;

        let mut result = Vec::with_capacity(count);
        for i in 0..count {
            result.push(unsafe { self.ptr.add(i).read_volatile() } == 0);
        }
        Ok(result)
    }
}

impl SafishPointer<i16> {
    pub fn read_i16(&self) -> Result<i16, Error> {
        let value = self.read();
        Ok(match self.endianness {
            Endianness::LittleEndian => i16::from_le(value),
            Endianness::BigEndian => i16::from_be(value),
        })
    }

    pub(crate) fn read_n_i16(&self, count: usize) -> Result<Vec<i16>, Error> {
        self.validate_room_for(count)?;

        let mut result = Vec::with_capacity(count);
        for i in 0..count {
            let value = unsafe { self.ptr.add(i).read_volatile() };
            result.push(match self.endianness {
                Endianness::LittleEndian => i16::from_le(value),
                Endianness::BigEndian => i16::from_be(value),
            });
        }
        Ok(result)
    }
}

impl SafishPointer<i32> {
    pub fn read_i32(&self) -> Result<i32, Error> {
        let value = self.read();
        Ok(match self.endianness {
            Endianness::LittleEndian => i32::from_le(value),
            Endianness::BigEndian => i32::from_be(value),
        })
    }

    pub(crate) fn read_n_i32(&self, count: usize) -> Result<Vec<i32>, Error> {
        self.validate_room_for(count)?;

        let mut result = Vec::with_capacity(count);
        for i in 0..count {
            let value = unsafe { self.ptr.add(i).read_volatile() };
            result.push(match self.endianness {
                Endianness::LittleEndian => i32::from_le(value),
                Endianness::BigEndian => i32::from_be(value),
            });
        }
        Ok(result)
    }
}

impl SafishPointer<i64> {
    pub fn read_i64(&self) -> Result<i64, Error> {
        let value = self.read();
        Ok(match self.endianness {
            Endianness::LittleEndian => i64::from_le(value),
            Endianness::BigEndian => i64::from_be(value),
        })
    }

    pub(crate) fn read_n_i64(&self, count: usize) -> Result<Vec<i64>, Error> {
        self.validate_room_for(count)?;

        let mut result = Vec::with_capacity(count);
        for i in 0..count {
            let value = unsafe { self.ptr.add(i).read_volatile() };
            result.push(match self.endianness {
                Endianness::LittleEndian => i64::from_le(value),
                Endianness::BigEndian => i64::from_be(value),
            });
        }
        Ok(result)
    }
}

impl SafishPointer<[u8; 4]> {
    pub fn read_f32(&self) -> Result<f32, Error> {
        let value = self.read();
        Ok(match self.endianness {
            Endianness::LittleEndian => f32::from_le_bytes(value),
            Endianness::BigEndian => f32::from_be_bytes(value),
        })
    }

    pub(crate) fn read_n_f32(&self, count: usize) -> Result<Vec<f32>, Error> {
        self.validate_room_for(count)?;

        let mut result = Vec::with_capacity(count);
        for i in 0..count {
            let value = unsafe { self.ptr.add(i).read_volatile() };
            result.push(match self.endianness {
                Endianness::LittleEndian => f32::from_le_bytes(value),
                Endianness::BigEndian => f32::from_be_bytes(value),
            });
        }
        Ok(result)
    }
}

impl SafishPointer<[u8; 8]> {
    pub fn read_f64(&self) -> Result<f64, Error> {
        let value = self.read();
        Ok(match self.endianness {
            Endianness::LittleEndian => f64::from_le_bytes(value),
            Endianness::BigEndian => f64::from_be_bytes(value),
        })
    }

    pub(crate) fn read_n_f64(&self, count: usize) -> Result<Vec<f64>, Error> {
        self.validate_room_for(count)?;

        let mut result = Vec::with_capacity(count);
        for i in 0..count {
            let value = unsafe { self.ptr.add(i).read_volatile() };
            result.push(match self.endianness {
                Endianness::LittleEndian => f64::from_le_bytes(value),
                Endianness::BigEndian => f64::from_be_bytes(value),
            });
        }
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use test_strategy::{Arbitrary, proptest};

    use crate::{errors::Error, perf_data::Endianness, safish_pointer::SafishPointer};

    #[derive(Default, PartialEq, Debug, Arbitrary, Clone)]
    #[repr(C)]
    struct TestStruct {
        a: u32,
        b: i64,
    }

    #[test]
    fn read_returns_the_struct() {
        let test_value = TestStruct::default();
        let max_offset = size_of::<TestStruct>();
        let result = SafishPointer::new(
            &test_value as *const TestStruct,
            max_offset,
            Endianness::LittleEndian,
        )
        .unwrap();
        assert_eq!(test_value, result.read());
    }

    #[test]
    fn read_the_second_struct_after_add() {
        let test_value = [TestStruct::default(), TestStruct { a: 1234, b: -234 }];
        let max_offset = test_value.len() * size_of::<TestStruct>();
        let result = SafishPointer::new(
            &test_value as *const TestStruct,
            max_offset,
            Endianness::LittleEndian,
        )
        .unwrap()
        .add(1)
        .unwrap();
        assert_eq!(test_value[1], result.read());
    }

    #[test]
    fn read_n_returns_an_error_when_max_offset_is_too_small() {
        let test_value = [TestStruct::default(), TestStruct { a: 1234, b: -234 }];
        let max_offset = test_value.len() * size_of::<TestStruct>() - 1;
        let result = SafishPointer::new(
            &test_value as *const TestStruct,
            max_offset,
            Endianness::LittleEndian,
        )
        .unwrap()
        .read_n(test_value.len());
        assert!(result.is_err_and(|e| matches!(e, Error::WontBeAbleToRead)));
    }

    #[test]
    fn read_n_is_ok_when_max_offset_is_exact() {
        let test_value = [TestStruct::default(), TestStruct { a: 1234, b: -234 }];
        let max_offset = test_value.len() * size_of::<TestStruct>();
        let result = SafishPointer::new(
            &test_value as *const TestStruct,
            max_offset,
            Endianness::LittleEndian,
        )
        .unwrap()
        .read_n(test_value.len());
        assert!(result.is_ok());
    }

    #[test]
    fn read_n_is_ok_when_max_offset_is_larger() {
        let test_value = [TestStruct::default(), TestStruct { a: 1234, b: -234 }];
        let max_offset = test_value.len() * size_of::<TestStruct>() + 1;
        let result = SafishPointer::new(
            &test_value as *const TestStruct,
            max_offset,
            Endianness::LittleEndian,
        )
        .unwrap()
        .read_n(test_value.len());
        assert!(result.is_ok());
    }

    #[proptest]
    fn new_is_error_when_size_too_small(
        test_value: TestStruct,
        #[strategy(0..(3 * size_of::<TestStruct>()))] max_offset: usize,
    ) {
        let result = SafishPointer::new(
            &test_value as *const TestStruct,
            max_offset,
            Endianness::LittleEndian,
        );
        assert_eq!(max_offset >= size_of::<TestStruct>(), result.is_ok());
        if result.is_err() {
            assert!(matches!(result.err().unwrap(), Error::WontBeAbleToRead));
        }
    }

    #[proptest]
    fn add_is_error_when_size_too_smmall(
        test_value: TestStruct,
        #[strategy(size_of::<TestStruct>()..(4 * size_of::<TestStruct>()))] max_offset: usize,
        #[strategy(0..10 as usize)] count: usize,
    ) {
        let required_offset = (count + 1) * size_of::<TestStruct>();
        let result = SafishPointer::new(
            &test_value as *const TestStruct,
            max_offset,
            Endianness::LittleEndian,
        )
        .unwrap()
        .add(count);
        assert_eq!(max_offset >= required_offset, result.is_ok());
        if result.is_err() {
            let error = result.err().unwrap();
            println!("{:?}", error);
            assert!(matches!(error, Error::WontBeAbleToRead));
        }
    }

    // todo proptest for size for add and convert

    #[proptest]
    fn all_structs_are_read_from_the_pointer(test_value: [TestStruct; 10]) {
        let max_offset = test_value.len() * size_of::<TestStruct>();
        let initial_pointer = SafishPointer::new(
            &test_value as *const TestStruct,
            max_offset,
            Endianness::LittleEndian,
        )
        .unwrap();
        for i in 0..test_value.len() {
            assert_eq!(
                test_value[i],
                initial_pointer.clone().add(i).unwrap().read()
            );
        }
    }

    #[proptest]
    fn all_structs_are_read_n_from_the_pointer(test_value: [TestStruct; 10]) {
        let max_offset = test_value.len() * size_of::<TestStruct>();
        let initial_pointer = SafishPointer::new(
            &test_value as *const TestStruct,
            max_offset,
            Endianness::LittleEndian,
        )
        .unwrap();
        let result = initial_pointer.read_n(test_value.len()).unwrap();
        for i in 0..test_value.len() {
            assert_eq!(test_value[i], result[i]);
        }
    }

    #[proptest]
    fn all_struct_fields_are_read(test_value: TestStruct) {
        let max_offset = size_of::<TestStruct>();

        let initial_pointer = SafishPointer::new(
            &test_value as *const TestStruct,
            max_offset,
            Endianness::LittleEndian,
        )
        .unwrap();
        let first_field_pointer = initial_pointer.convert::<u32>().unwrap();
        assert_eq!(test_value.a, first_field_pointer.read());
        let second_field_pointer = first_field_pointer
            .add(2)
            .unwrap()
            .convert::<i64>()
            .unwrap();
        assert_eq!(test_value.b, second_field_pointer.read());
    }
}
