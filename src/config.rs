/// Configuration constants for the h2l0l application.

/// The name of the process to monitor.
/// This is platform-specific - Windows uses .exe, macOS uses the app name.
#[cfg(target_os = "windows")]
pub const PROCESS_NAME: &str = "League of Legends.exe";

#[cfg(target_os = "macos")]
pub const PROCESS_NAME: &str = "League of Legends";

#[cfg(not(any(target_os = "windows", target_os = "macos")))]
pub const PROCESS_NAME: &str = "League of Legends";

/// Polling interval in seconds.
/// Adjust this value to change how often the process is checked.
pub const POLL_INTERVAL_SECS: u64 = 30;
