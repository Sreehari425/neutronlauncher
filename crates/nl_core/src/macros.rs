/// A macro to log messages with a colored [LAUNCHER] prefix.
#[macro_export]
macro_rules! launcher_log {
    ($($arg:tt)*) => {{
        use $crate::colored::Colorize;
        // Print [LAUNCHER] in yellow and bold, then your message
        println!("{} {}", "[LAUNCHER]".yellow().bold(), format!($($arg)*));
    }};
}

/// A macro to log error messages with a colored [ERROR] prefix.
#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {{
        use $crate::colored::Colorize;
        // Print [ERROR] in red and bold, then your message
        println!("{} {}", "[ERROR]".red().bold(), format!($($arg)*));
    }};
}
