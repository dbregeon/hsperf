use std::{
    ffi::c_void,
    fs::File,
    num::NonZero,
    os::{fd::AsFd, unix::fs::MetadataExt},
    path::PathBuf,
    ptr::NonNull,
};

use nix::sys::mman::{MapFlags, MsFlags, ProtFlags, mmap, msync, munmap};

use crate::{
    ConstantEntry, VariableEntry, errors::Error, perf_data::PerfDataProlog,
    variable_data_reference::VariableDataReference,
};

pub struct JvmMonitor {
    pid: u32,
    prolog_addr: NonNull<c_void>,
    length: usize,
    constants: Vec<ConstantEntry>,
    variables: Vec<VariableDataReference>,
}

impl JvmMonitor {
    /// the pid of the vm being monitored
    pub fn pid(&self) -> u32 {
        self.pid
    }

    /// Returns the constant hsperfdata values presented by the JVMM
    pub fn constants(&self) -> &Vec<ConstantEntry> {
        &self.constants
    }

    /// Refreshes and returns the variable entry values from the JVM.
    pub fn refresh(&mut self) -> Result<Vec<&VariableEntry>, Error> {
        unsafe {
            msync(self.prolog_addr, self.length, MsFlags::MS_SYNC)
                .map_err(|e| Error::FailedToSync(e))?
        };
        self.refresh_entries()
    }

    /// Refines the JVM monitor so that it only keep the variable entries matching the filter.
    pub fn only<P>(mut self, mut filter: P) -> Self
    where
        P: FnMut(&str) -> bool,
    {
        self.variables.retain(|data| filter(data.name()));
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

        let (constants, variables) = PerfDataProlog::read_entries(&prolog_addr, length)?;

        Ok(Self {
            pid,
            prolog_addr,
            length,
            constants,
            variables,
        })
    }

    fn refresh_entries(&mut self) -> Result<Vec<&VariableEntry>, Error> {
        let mut result = Vec::with_capacity(self.variables.len());
        for reference in self.variables.iter_mut() {
            result.push(reference.refresh_entry()?);
        }
        Ok(result)
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
