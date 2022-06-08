use std::{
    fmt, 
    error::Error,
};


/// Tailor made errors specifically for this engine.
#[derive(Debug, Clone)]
pub enum GameError {
    CustomError(String),
}

impl Error for GameError {
    fn cause(&self) -> Option<&dyn Error> {
        match *self {
            _ => None,
        }
    }
}

impl fmt::Display for GameError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GameError::CustomError(ref message) => write!(f, "Custom error: {}", message),
            _ => write!(f, "GameError: {:?}", self),
        }
    }
}

/// Wrapper type for `Result<T, GameError>`
pub type GameResult<T = ()> = Result<T, GameError>;