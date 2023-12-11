use num::Num;

pub mod add;
pub mod div;
pub mod error;
pub mod field_element;
pub mod mul;
pub mod pow;
pub mod sub;

pub trait FieldElement<T: Num> {
    fn to_field_element(&self, num: T) -> field_element::FieldElement;
}
