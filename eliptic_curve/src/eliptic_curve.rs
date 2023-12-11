use super::error::{PointError, Result};
use finite_field::field_element::FieldElement;
use num::traits::Pow;
use std::fmt;
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
        if let (Some(ref x_val), Some(ref y_val)) = (&x, &y) {
            let lhs = y_val.pow(2);
            let rhs = x_val.pow(3) + (&a * x_val) + &b;
            if lhs != rhs {
                return Err(PointError::NotOnCurve {
                    x: x_val.clone(),
                    y: y_val.clone(),
                });
            }
        }
        Ok(Point { x, y, a, b })
    }

    // Additional methods (e.g., ec_add_identity, ec_not_eq_point_add, ec_eq_point_add)...
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:?},{:?})_{}_{}", self.x, self.y, self.a, self.b)
    }
}

#[inline(always)]
fn ec_add_identity<'a>(p1: &'a Point, p2: &'a Point) -> &'a Point {
    if p1.x.is_none() {
        p2
    } else {
        p1
    }
}

impl std::ops::Add for Point {
    type Output = Result<Point>;

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
            // s = (y2 - y1) / (x2 - x1)
            // x3 = s^2 - (x1 - x2)
            // y3 = s * (x1 - x3) - y1
            (Some(x1), Some(y1), Some(x2), Some(y2)) if x1 != x2 => {
                let s = &((y2 - y1) / (x2 - x1));
                let x3 = s.pow(2) - (x1 - x2);
                let y3 = s * (x1 - &x3) - y1;
                Ok(Point::new(Some(x3), Some(y3), self.a, self.b)?)
            }

            // When p1 == p2
            //  s = (3x1^2 + a) / 2y1
            // x3 = s^2 - 2x1
            // y3 = s * (x1 - x3) - y1
            (Some(x1), Some(y1), Some(_), Some(_)) if self == rhs => {
                let s = &(3 * x1.pow(2) + &self.a / (2 * y1));
                let x3 = s.pow(2) - (2 * x1);
                let y3 = s * (x1 - &x3) - y1;
                Ok(Point {
                    x: Some(x3),
                    y: Some(y3),
                    a: self.a,
                    b: self.b,
                })
            }

            // One of the points is the identity
            // (None, None, _, _) => Ok(()),
            // (_, _, None, None) => Ok(()),

            // Unhandled cases
            _ => Err(PointError::UnhandledAdditionCase),
        }
    }
}
