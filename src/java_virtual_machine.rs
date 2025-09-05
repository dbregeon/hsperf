use std::{
    env::temp_dir,
    fs::{self, DirEntry},
    path::PathBuf,
};

use crate::{errors::Error, jvm_monitor::JvmMonitor};

#[derive(Debug)]
pub struct JavaVirtualMachine {
    hs_perf_data_path: PathBuf,
    pid: u32,
}

impl JavaVirtualMachine {
    /// Creates a monitor to read the hsperfdata of a specific JVM.
    pub fn monitor(self) -> Result<JvmMonitor, Error> {
        JvmMonitor::map(self.pid, &self.hs_perf_data_path)
    }

    /// Lists the Java Virtual Machines on this host that publish their data in /tmp/hsperfdata for the current user.
    pub fn list_jvms() -> Vec<JavaVirtualMachine> {
        let mut base_path = temp_dir();
        base_path.push(format!("hsperfdata_{}", whoami::username()));
        if let Ok(java_processes) = fs::read_dir(base_path) {
            java_processes
                .flat_map(|b| b)
                .flat_map(|p| JavaVirtualMachine::try_from(p))
                .collect()
        } else {
            vec![]
        }
    }
}

impl TryFrom<DirEntry> for JavaVirtualMachine {
    type Error = Error;

    fn try_from(value: DirEntry) -> Result<Self, Self::Error> {
        let hs_perf_data_path = value.path();
        let file_name = hs_perf_data_path
            .file_name()
            .ok_or(Error::InvalidPath(hs_perf_data_path.clone()))?;
        let pid = file_name
            .to_string_lossy()
            .parse()
            .map_err(|e| Error::FailedToParsePid(e))?;
        Ok(JavaVirtualMachine {
            hs_perf_data_path,
            pid,
        })
    }
}
