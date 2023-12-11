use num_bigint::ToBigInt;

use super::field_element::FieldElement;
use std::ops::Mul;

impl Mul for FieldElement {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        (&self).operate(&rhs, |a, b| a * b)
    }
}
impl<'a, 'b> Mul<&'b FieldElement> for &'a FieldElement {
    type Output = FieldElement;
    fn mul(self, rhs: &'b FieldElement) -> Self::Output {
        self.check_field_panic(&rhs);
        FieldElement {
            num: (&self.num * &rhs.num) % &self.prime,
            prime: self.prime.clone(),
        }
    }
}

impl<'a> Mul<FieldElement> for &'a FieldElement {
    type Output = FieldElement;
    fn mul(self, rhs: FieldElement) -> Self::Output {
        (&self).operate(&rhs, |a, b| a * b)
    }
}
impl<'a> Mul<&'a FieldElement> for FieldElement {
    type Output = FieldElement;
    fn mul(self, rhs: &'a FieldElement) -> Self::Output {
        (&self).operate(&rhs, |a, b| a * b)
    }
}
impl Mul<FieldElement> for u32 {
    type Output = FieldElement;
    fn mul(self, rhs: FieldElement) -> Self::Output {
        let finite_field_num = FieldElement {
            num: self.to_bigint().unwrap(),
            prime: rhs.prime.clone(),
        };
        finite_field_num.check_field_panic(&rhs);
        FieldElement {
            num: (&finite_field_num.num * &rhs.num) % &finite_field_num.prime,
            prime: finite_field_num.prime.clone(),
        }
    }
}
impl<'a> Mul<&'a FieldElement> for u32 {
    type Output = FieldElement;
    fn mul(self, rhs: &'a FieldElement) -> Self::Output {
        let finite_field_num = FieldElement {
            num: self.to_bigint().unwrap(),
            prime: rhs.prime.clone(),
        };
        finite_field_num.check_field_panic(&rhs);
        FieldElement {
            num: (&finite_field_num.num * &rhs.num) % &finite_field_num.prime,
            prime: finite_field_num.prime.clone(),
        }
    }
}
