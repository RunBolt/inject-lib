//! Adds cross platform usage of a Process
use crate::Result;

/// Cross platform Process
pub struct Process {
    #[cfg(target_family = "windows")]
    process: crate::platforms::windows::process::Process,
}

impl Process {
    /// Creates a new process (opens it up) for use in the injector
    #[allow(unused_variables)]
    pub fn new_injector(pid: u32) -> Result<Self> {
        #[cfg(target_family = "windows")]
        {
            let process = crate::platforms::windows::process::Process::new_injector(pid)?;
            Ok(Process { process })
        }
        #[cfg(not(target_family = "windows"))]
        Ok(Process {})
    }

    /// Determines if process is 64 bit or not
    pub fn is_32_bit(&self) -> Result<bool> {
        #[cfg(target_family = "windows")]
        {
            self.process.is_under_wow()
        }

        #[cfg(not(target_family = "windows"))]
        Err(crate::error::Error::Unsupported(Some(
            "is 32 bit not supported for non windows machines yet",
        )))
    }
}
