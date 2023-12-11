use super::field_element::FieldElement;
use std::ops::Sub;

impl Sub for FieldElement {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        (&self).operate(
            &rhs,
            |a, b| {
                if a < b {
                    &self.prime + (a - b)
                } else {
                    a - b
                }
            },
        )
    }
}
impl<'a, 'b> Sub<&'b FieldElement> for &'a FieldElement {
    type Output = FieldElement;
    fn sub(self, rhs: &'b FieldElement) -> Self::Output {
        (&self).operate(
            &rhs,
            |a, b| {
                if a < b {
                    &self.prime + (a - b)
                } else {
                    a - b
                }
            },
        )
    }
}
impl<'a> Sub<FieldElement> for &'a FieldElement {
    type Output = FieldElement;
    fn sub(self, rhs: FieldElement) -> Self::Output {
        (&self).operate(
            &rhs,
            |a, b| {
                if a < b {
                    &self.prime + (a - b)
                } else {
                    a - b
                }
            },
        )
    }
}
impl<'a> Sub<&'a FieldElement> for FieldElement {
    type Output = FieldElement;
    fn sub(self, rhs: &'a FieldElement) -> Self::Output {
        (&self).operate(
            &rhs,
            |a, b| {
                if a < b {
                    &self.prime + (a - b)
                } else {
                    a - b
                }
            },
        )
    }
}
