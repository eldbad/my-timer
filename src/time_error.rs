use std::{fmt, io};

pub enum TimerError {
    NoLastRecordError,
    WrongNumberOfArguments,
    WrongArgument,
    IoError(io::Error),
    TimeFormatError(time::error::Format),
}

// impl fmt::Display for TimerError {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             TimerError::NoLastRecordError => write!(f, "no last record found in file"),
//             TimerError::IoError(err) => write!(f, "{}", err),
//             TimerError::WrongNumberOfArguments => write!(f, "wrong number of arguments given"),
//             TimerError::WrongArgument => write!(f, "wrong argument"),
//             TimerError::TimeFormatError(err) => write!(f, "{}", err),
//         }
//     }
// }

impl fmt::Debug for TimerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TimerError::NoLastRecordError => write!(f, "no last record found in file"),
            TimerError::IoError(err) => write!(f, "{}", err),
            TimerError::WrongNumberOfArguments => write!(f, "wrong number of arguments given"),
            TimerError::WrongArgument => write!(f, "wrong argument"),
            TimerError::TimeFormatError(err) => write!(f, "{}", err),
        }
    }
}

impl From<io::Error> for TimerError {
    fn from(error: io::Error) -> Self {
        TimerError::IoError(error)
    }
}

impl From<time::error::Format> for TimerError {
    fn from(error: time::error::Format) -> Self {
        TimerError::TimeFormatError(error)
    }
}
