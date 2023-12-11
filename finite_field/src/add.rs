use super::field_element::FieldElement;
use std::ops::Add;
impl Add for FieldElement {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        (&self).operate(&rhs, |a, b| a + b)
    }
}
impl<'a, 'b> Add<&'b FieldElement> for &'a FieldElement {
    type Output = FieldElement;
    fn add(self, rhs: &'b FieldElement) -> Self::Output {
        (&self).operate(&rhs, |a, b| a + b)
    }
}
impl<'a> Add<FieldElement> for &'a FieldElement {
    type Output = FieldElement;
    fn add(self, rhs: FieldElement) -> Self::Output {
        (&self).operate(&rhs, |a, b| a + b)
    }
}
impl<'a> Add<&'a FieldElement> for FieldElement {
    type Output = FieldElement;
    fn add(self, rhs: &'a FieldElement) -> Self::Output {
        (&self).operate(&rhs, |a, b| a + b)
    }
}
