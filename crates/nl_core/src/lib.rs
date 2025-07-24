pub mod init;

/// Re-export colored for use in macros
pub extern crate colored;

/// A macro to log messages with a colored [LAUNCHER] prefix.

#[macro_export]
macro_rules! launcher_log {
    ($($arg:tt)*) => {{
        use $crate::colored::Colorize;
        // Print [LAUNCHER] in yellow and bold, then your message
        println!("{} {}", "[LAUNCHER]".yellow().bold(), format!($($arg)*));
    }};
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {{
        use $crate::colored::Colorize;
        // Print [ERROR] in red and bold, then your message
        println!("{} {}", "[ERROR]".red().bold(), format!($($arg)*));
    }};
}
