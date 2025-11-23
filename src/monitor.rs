//! Module for monitoring if a specific process is running.

use std::ffi::OsStr;
use sysinfo::System;

/// Checks if a process with the given name is currently running.
///
/// # Arguments
///
/// * `process_name` - The name of the process executable to look for (e.g., "League of Legends.exe").
///
/// # Returns
///
/// `true` if the process is running, `false` otherwise.
pub fn is_process_running(process_name: &str) -> bool {
    let mut sys = System::new_all();
    sys.refresh_all();
    let os_name = OsStr::new(process_name);
    sys.processes_by_name(os_name).next().is_some()
}
