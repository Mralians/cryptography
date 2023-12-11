use super::field_element::FieldElement;
use num::BigInt;
use std::ops::Div;

impl Div for FieldElement {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        (&self).operate(&rhs, |a, b| {
            a * b.modpow(&(&self.prime - BigInt::from(2)), &self.prime)
        })
    }
}

impl<'a, 'b> Div<&'b FieldElement> for &'a FieldElement {
    type Output = FieldElement;

    fn div(self, rhs: &'b FieldElement) -> Self::Output {
        (&self).operate(&rhs, |a, b| {
            a * b.modpow(&(&self.prime - BigInt::from(2)), &self.prime)
        })
    }
}
impl<'a> Div<FieldElement> for &'a FieldElement {
    type Output = FieldElement;
    fn div(self, rhs: FieldElement) -> Self::Output {
        (&self).operate(&rhs, |a, b| {
            a * b.modpow(&(&self.prime - BigInt::from(2)), &self.prime)
        })
    }
}
impl<'a> Div<&'a FieldElement> for FieldElement {
    type Output = FieldElement;
    fn div(self, rhs: &'a FieldElement) -> Self::Output {
        (&self).operate(&rhs, |a, b| {
            a * b.modpow(&(&self.prime - BigInt::from(2)), &self.prime)
        })
    }
}
