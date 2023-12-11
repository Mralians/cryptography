use super::error::{FieldElementError, Result};
use num::{BigInt, Num};
use std::fmt;
#[derive(Clone, PartialEq)]
pub struct FieldElement {
    pub num: BigInt,
    pub prime: BigInt,
}
/////////////////////////////////////////////////////////////////
impl FieldElement {
    pub fn new(num: &str, prime: &str) -> Result<Self> {
        let num = BigInt::from_str_radix(num, 16)?;
        let prime = BigInt::from_str_radix(prime, 16)?;
        if num >= prime {
            return Err(FieldElementError::NotInField { num, prime });
        }
        Ok(Self { num, prime })
    }

    pub(super) fn check_field_panic(&self, other: &Self) {
        assert_eq!(self.prime, other.prime, "DifferentFields")
    }
    pub(super) fn operate<F>(&self, other: &Self, operation: F) -> Self
    where
        F: Fn(&BigInt, &BigInt) -> BigInt,
    {
        self.check_field_panic(other);
        let result = operation(&self.num, &other.num) % &self.prime;
        FieldElement {
            num: result,
            prime: self.prime.clone(),
        }
    }
}

impl fmt::Display for FieldElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "FieldElement_{:x}({:x})", self.prime, self.num)
    }
}
impl fmt::Debug for FieldElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "FieldElement_{:x}({:x})", self.prime, self.num)
    }
}
