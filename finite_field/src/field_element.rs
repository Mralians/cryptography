use super::error::{FieldElementError, Result};
use num::{traits::Pow, BigInt, FromPrimitive, Num};
use num_bigint::ToBigInt;
use std::{
    fmt,
    ops::{Add, Div, Mul, Sub},
};
#[derive(Clone, PartialEq)]
pub struct FieldElement {
    pub num: BigInt,
    pub prime: BigInt,
}

impl FieldElement {
    pub fn new(num: &str, prime: &str) -> Result<Self> {
        let num = BigInt::from_str_radix(num, 16)?;
        let prime = BigInt::from_str_radix(prime, 16)?;
        if num >= prime {
            return Err(FieldElementError::NotInField { num, prime });
        }
        Ok(Self { num, prime })
    }

    fn check_field_panic(&self, other: &Self) {
        if self.prime != other.prime {
            panic!("DifferentFields");
        }
    }
}

impl Add for FieldElement {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        self.check_field_panic(&rhs);
        Self {
            num: (&self.num + &rhs.num) % &self.prime,
            prime: self.prime,
        }
    }
}

impl Sub for FieldElement {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self.check_field_panic(&rhs);
        let num = if self.num < rhs.num {
            &self.prime + &self.num - &rhs.num
        } else {
            &self.num - &rhs.num
        };
        Self {
            num,
            prime: self.prime,
        }
    }
}
impl Pow<usize> for FieldElement {
    type Output = Self;
    fn pow(self, rhs: usize) -> Self::Output {
        let num = self.num.modpow(&rhs.to_bigint().unwrap(), &self.prime);
        Self {
            num,
            prime: self.prime,
        }
    }
}

impl Mul for FieldElement {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        self.check_field_panic(&rhs);
        Self {
            num: (&self.num * &rhs.num) % &self.prime,
            prime: self.prime,
        }
    }
}

impl Div for FieldElement {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        self.check_field_panic(&rhs);
        let inverse = rhs
            .num
            .modpow(&(&self.prime - BigInt::from(2)), &self.prime);
        Self {
            num: (&self.num * &inverse) % &self.prime,
            prime: self.prime,
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
