use num::Num;

pub mod error;
pub mod field_element;

pub trait FieldElement<T: Num> {
    fn to_field_element(&self, num: T) -> field_element::FieldElement;
}
