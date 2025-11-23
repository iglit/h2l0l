/// Configuration constants for the h2l0l application.

/// The name of the process to monitor.
/// Change this if your League of Legends executable has a different name.
pub const PROCESS_NAME: &str = "League of Legends.exe";

/// Polling interval in seconds.
/// Adjust this value to change how often the process is checked.
pub const POLL_INTERVAL_SECS: u64 = 30;
