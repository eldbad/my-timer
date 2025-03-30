use std::{env, fmt, io};

pub enum TimerError {
    NoLastRecord,
    WrongNumberOfArguments,
    WrongArgument,
    ParseDuration,
    IoError(io::Error),
    TimeFormat(time::error::Format),
    ParsingDate(time::error::Parse),
    VarError(env::VarError),
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
            TimerError::NoLastRecord => write!(f, "no last record found in file"),
            TimerError::IoError(err) => write!(f, "{}", err),
            TimerError::WrongNumberOfArguments => write!(f, "wrong number of arguments given"),
            TimerError::WrongArgument => write!(f, "wrong argument"),
            TimerError::ParseDuration => write!(f, "couldn't parse duration"),
            TimerError::TimeFormat(err) => write!(f, "{}", err),
            TimerError::ParsingDate(err) => write!(f, "{}", err),
            TimerError::VarError(err) => write!(f, "{}", err),
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
        TimerError::TimeFormat(error)
    }
}

impl From<time::error::Parse> for TimerError {
    fn from(error: time::error::Parse) -> Self {
        TimerError::ParsingDate(error)
    }
}

impl From<env::VarError> for TimerError {
    fn from(error: env::VarError) -> Self {
        TimerError::VarError(error)
    }
}
