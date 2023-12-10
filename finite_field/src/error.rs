use num::BigInt;
use std::{error, fmt};

#[derive(Debug)]
pub enum FieldElementError {
    NotInField { num: BigInt, prime: BigInt },
    DifferentFields,
    ParseError(num::bigint::ParseBigIntError),
}

impl fmt::Display for FieldElementError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FieldElementError::NotInField { num, prime } => {
                write!(f, "Num {} not in field 0 to {}", num, prime)
            }
            FieldElementError::DifferentFields => {
                write!(f, "Cannot operate on numbers in different fields")
            }
            FieldElementError::ParseError(e) => e.fmt(f),
        }
    }
}

impl error::Error for FieldElementError {}

// Implement From for ParseBigIntError to automatically convert errors
impl From<num::bigint::ParseBigIntError> for FieldElementError {
    fn from(err: num::bigint::ParseBigIntError) -> Self {
        FieldElementError::ParseError(err)
    }
}

pub type Result<T> = std::result::Result<T, FieldElementError>;
