use std::fmt;
use std::error::Error;



pub struct DexError {
    pub code: String,
    pub message: String,
}

pub fn new(code: String,message: String) -> DexError{
    DexError{
        code,
        message,
    }
}

// Different error messages according to AppError.code
impl fmt::Display for DexError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} : {}",
            self.code, self.message
        )
    }
}

impl Error for DexError {
}

// A unique format for dubugging output
impl fmt::Debug for DexError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} : {}",
            self.code, self.message
        )
    }
}

