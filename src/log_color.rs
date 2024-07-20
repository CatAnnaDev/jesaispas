#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        eprintln!("\x1B[31m{}\x1B[0m", format_args!($($arg)*))
    };
}
#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        eprintln!("\x1B[33m{}\x1B[0m", format_args!($($arg)*))
    };
}
#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        eprintln!("\x1B[32m{}\x1B[0m", format_args!($($arg)*))
    };
}

#[macro_export]
macro_rules! interaction {
    ($($arg:tt)*) => {
        eprintln!("\x1B[94m{}\x1B[0m", format_args!($($arg)*))
    };
}

#[macro_export]
macro_rules! bool_state {
    ($t:expr, $f:expr, $status:expr) => {
        match $status {
            true => format!("\x1B[{}\x1B[0m", $t),
            false => format!("\x1B[{}\x1B[0m", $f),
        }
    };
}
