use super::error::{PointError, Result};
use finite_field::field_element::FieldElement;
use num::bigint::ToBigInt;
use num::traits::Pow;
use std::fmt;
use std::ops::Mul;
#[derive(Debug, Clone, PartialEq)]
pub struct Point {
    x: Option<FieldElement>,
    y: Option<FieldElement>,
    a: FieldElement,
    b: FieldElement,
}

impl Point {
    pub fn new(
        x: Option<FieldElement>,
        y: Option<FieldElement>,
        a: FieldElement,
        b: FieldElement,
    ) -> Result<Self> {
        match (x, y) {
            (Some(x_val), Some(y_val)) => {
                let lhs = y_val.pow(2);
                let rhs = x_val.pow(3) + a.mul(x_val) + b;
                if lhs != rhs {
                    Err(PointError::NotOnCurve { x: x_val, y: y_val })
                } else {
                    Ok(Point {
                        x: Some(x_val),
                        y: Some(y_val),
                        a,
                        b,
                    })
                }
            }
            _ => Ok(Point { x, y, a, b }),
        }
    }

    // Additional methods (e.g., ec_add_identity, ec_not_eq_point_add, ec_eq_point_add)...
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:?},{:?})_{}_{}", self.x, self.y, self.a, self.b)
    }
}

#[inline(always)]
fn ec_add_identity(p1: &Point, p2: &Point) -> Point {
    if p1.x.is_none() {
        *p2
    } else {
        *p1
    }
}
// When the points are different.
// s = (y2 - y1) / (x2 - x1)
// x3 = s^2 - (x1 - x2)
// y3 = s * (x1 - x3) - y1

fn ec_not_eq_point_add(p1: Point, p2: Point) -> Point {
    let x = (p1.x.unwrap(), p2.x.unwrap()); // X pair
    let y = (p1.y.unwrap(), p2.y.unwrap()); // y pair

    let s = (y.1 - y.0) / (x.1 - x.0);
    let x3 = s.pow(2) - x.0 - x.1;
    let y3 = s * (x.0 - x3) - y.0;
    Point {
        x: Some(x3),
        y: Some(y3),
        a: p1.a,
        b: p2.b,
    }
}

// When p1 == p2
//  s = (3x1^2 + a) / 2y1
// x3 = s^2 - 2x1
// y3 = s * (x1 - x3) - y1
// fn ec_eq_point_add(p1: Point, p2: Point) -> Point {
//     let x = (p1.x.unwrap(), p2.x.unwrap());
//     let y = (p1.y.unwrap(), p2.y.unwrap());
//
//     let s = (FieldElement::new(3.to_bigint(), x.0) * x.0.pow(2) + p1.a)
//         / (2_u64.to_filed_element(x.0.prime) * y.0);
//     let x3 = s.pow(2) - (2_u64.to_filed_element(x.0.prime) * x.0);
//     let y3 = s * (x.0 - x3) - y.0;
//     Point {
//         x: Some(x3),
//         y: Some(y3),
//         a: p1.a,
//         b: p1.b,
//     }
// }

impl std::ops::Add for Point {
    type Output = Result<()>;

    fn add(self, rhs: Self) -> Self::Output {
        if self.a != rhs.a || self.b != rhs.b {
            return Err(PointError::DifferentCurves);
        }

        match (&self.x, &self.y, &rhs.x, &rhs.y) {
            // Points are inverses of each other
            (Some(x1), Some(y1), Some(x2), Some(y2)) if x1 == x2 && y1 != y2 => {
                Err(PointError::PointAtInfinity)
            }

            // Point addition when points are different
            (Some(x1), Some(y1), Some(x2), Some(y2)) if x1 != x2 => {
                let p = ec_not_eq_point_add(self.clone(), rhs.clone());
                Ok(())
            }

            // Point doubling
            // (Some(x), Some(y), _, _) if self == rhs => ec_eq_point_add(x, y, self.a, self.b),

            // One of the points is the identity
            (None, None, _, _) => Ok(()),
            (_, _, None, None) => Ok(()),

            // Unhandled cases
            _ => Err(PointError::UnhandledAdditionCase),
        }
    }
}
