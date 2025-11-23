h2l0l\src\monitor.rs
//! Module for monitoring if a specific process is running.

use sysinfo::{System, SystemExt, ProcessExt};

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
    sys.refresh_processes();
    sys.processes_by_name(process_name).next().is_some()
}
