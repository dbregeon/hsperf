use std::{
    collections::HashMap,
    ffi::c_void,
    fs::File,
    num::NonZero,
    os::{fd::AsFd, unix::fs::MetadataExt},
    path::PathBuf,
    ptr::NonNull,
};

use nix::sys::mman::{MapFlags, ProtFlags, mmap, munmap};

use crate::{
    EntryValue,
    constant_entry::ConstantEntry,
    errors::Error,
    perf_data::{PerfDataProlog, Unit},
    variable_entry::VariableEntry,
};

pub enum Entry {
    Constant(ConstantEntry),
    Variable(VariableEntry),
}

impl Entry {
    pub fn value(&self) -> Result<EntryValue, Error> {
        match self {
            Entry::Constant(entry) => Ok(entry.value()),
            Entry::Variable(entry) => entry.value(),
        }
    }

    pub fn unit(&self) -> Unit {
        match self {
            Entry::Constant(entry) => entry.unit(),
            Entry::Variable(entry) => entry.unit(),
        }
    }
}
pub struct JvmMonitor {
    pid: u32,
    prolog_addr: NonNull<c_void>,
    length: usize,
    entries: HashMap<String, Entry>,
}

impl JvmMonitor {
    /// the pid of the vm being monitored
    pub fn pid(&self) -> u32 {
        self.pid
    }

    /// Returns the  hsperfdata entries presented by the JVMM
    pub fn entries(&self) -> &HashMap<String, Entry> {
        &self.entries
    }

    /// Refines the JVM monitor so that it only keep the variable entries matching the filter.
    pub fn only<P>(mut self, mut filter: P) -> Self
    where
        P: FnMut(&str) -> bool,
    {
        self.entries.retain(|key, _value| filter(key));
        self
    }

    pub(crate) fn map(pid: u32, path: &PathBuf) -> Result<Self, Error> {
        let f = File::open(path).map_err(|e| Error::FailedToOpen(e))?;
        let length = f
            .metadata()
            .map_err(|e| Error::FailedToReadMetaData(e))?
            .size() as usize;

        // Ensure there are enough bytes in the mapped file to read a PerfDataProlog.
        if length < size_of::<PerfDataProlog>() {
            return Err(Error::WontBeAbleToRead);
        }

        let prolog_addr = Self::map_file_to_memory(f, length)?;

        let entries = PerfDataProlog::read_entries(&prolog_addr, length)?;

        Ok(Self {
            pid,
            prolog_addr,
            length,
            entries,
        })
    }

    fn map_file_to_memory(f: File, length: usize) -> Result<NonNull<c_void>, Error> {
        unsafe {
            mmap(
                None,
                NonZero::new(length).unwrap(),
                ProtFlags::PROT_READ,
                MapFlags::MAP_SHARED,
                f.as_fd(),
                0,
            )
        }
        .map_err(|e| Error::FailedToMapToMemory(e))
    }
}

impl Drop for JvmMonitor {
    fn drop(&mut self) {
        unsafe { munmap(self.prolog_addr, self.length) }.unwrap();
    }
}
