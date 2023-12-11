use super::field_element::FieldElement;
use num::traits::Pow;
use num_bigint::ToBigInt;

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
impl<'a> Pow<usize> for &'a FieldElement {
    type Output = FieldElement;
    fn pow(self, rhs: usize) -> Self::Output {
        let num = self.num.modpow(&rhs.to_bigint().unwrap(), &self.prime);
        FieldElement {
            num,
            prime: self.prime.clone(),
        }
    }
}
