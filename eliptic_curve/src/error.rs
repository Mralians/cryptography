use finite_field::field_element::FieldElement;
use std::{error, fmt};

// Custom Error Type for Point Operations
#[derive(Debug)]
pub enum PointError {
    NotOnCurve { x: FieldElement, y: FieldElement },
    DifferentCurves,
    PointAtInfinity,
    UnhandledAdditionCase,
}

impl fmt::Display for PointError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PointError::NotOnCurve { x, y } => {
                write!(f, "Point ({:?}, {:?}) is not on the curve", x, y)
            }
            PointError::DifferentCurves => write!(f, "Points are not on the same curve"),
            PointError::PointAtInfinity => write!(f, "Operation resulted in a point at infinity"),
            PointError::UnhandledAdditionCase => write!(f, "Unhandled addition case"),
        }
    }
}

pub type Result<T> = core::result::Result<T, PointError>;
impl error::Error for PointError {}
