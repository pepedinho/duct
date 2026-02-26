#[macro_export]
macro_rules! log {
    ($level: expr,$($arg:tt)*) => {
        $crate::Logger::log($level, &format!($($arg)*))
    };
}
