#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Rational {
    p: u64,
    q: u64,
    sign: Sign,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Sign {
    Positive,
    Negative,
}

use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};
use Sign::*;

impl From<u64> for Rational {
    fn from(p: u64) -> Self {
        Self {
            p,
            q: 1,
            sign: Positive,
        }
    }
}

macro_rules! impl_from_unsized {
    ($($t:ty), +) => {
        $(impl From<$t> for Rational {
            fn from(p: $t) -> Self {
                (p as u64).into()
            }
        })+
    };
}

impl_from_unsized!(u8, u16, u32);

macro_rules! impl_from_sized {
    ($($t:ty), +) => {
        $(impl From<$t> for Rational {
            fn from(p: $t) -> Self {
                (p as i64).into()
            }
        })+
    };
}

impl_from_sized!(i8, i16, i32);

impl From<i64> for Rational {
    fn from(p: i64) -> Self {
        Self {
            p: p.abs() as u64,
            q: 1,
            sign: if p < 0 { Negative } else { Positive },
        }
    }
}

impl From<Rational> for f64 {
    fn from(r: Rational) -> Self {
        let q = r.p as f64 / r.q as f64;
        match r.sign {
            Positive => q,
            _ => -q,
        }
    }
}

impl Add<Rational> for Rational {
    type Output = Self;

    fn add(mut self, mut rhs: Rational) -> Self::Output {
        if self.q != rhs.q {
            let lcm = self.q.lcm(&rhs.q);
            let self_factor = lcm / self.q;
            let rhs_factor = lcm / rhs.q;

            self.p *= self_factor;
            self.q *= self_factor;
            rhs.p *= rhs_factor;
            rhs.q *= rhs_factor;
        }

        let p = if self.sign == rhs.sign {
            self.p + rhs.p
        } else if self.sign == Positive {
            self.p.checked_sub(rhs.p).unwrap_or(rhs.p - self.p)
        } else {
            rhs.p.checked_sub(self.p).unwrap_or(self.p - rhs.p)
        };
        let q = self.q;
        let sign = if self.p > rhs.p { self.sign } else { rhs.sign };

        Self {
            p: p / p.gcd(&q),
            q: q / p.gcd(&q),
            sign,
        }
    }
}

impl AddAssign for Rational {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}

impl Neg for Rational {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            sign: match self.sign {
                Positive => Negative,
                _ => Positive,
            },
            ..self
        }
    }
}

impl Sub for Rational {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self + -rhs
    }
}

impl SubAssign for Rational {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs
    }
}

impl Mul for Rational {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let p = self.p * rhs.p;
        let q = self.q * rhs.q;
        let sign = if self.sign == rhs.sign {
            Positive
        } else {
            Negative
        };

        Self {
            p: p / p.gcd(&q),
            q: q / p.gcd(&q),
            sign,
        }
    }
}

impl MulAssign for Rational {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs
    }
}

impl Div for Rational {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        let p = self.p * rhs.q;
        let q = self.q * rhs.p;

        let sign = if self.sign == rhs.sign {
            Positive
        } else {
            Negative
        };

        Self {
            p: p / p.gcd(&q),
            q: q / p.gcd(&q),
            sign,
        }
    }
}

impl DivAssign for Rational {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs
    }
}

// Implementation from https://docs.rs/num-integer/0.1.44/src/num_integer/lib.rs.html#1011
trait GcdLcm {
    fn lcm(&self, other: &Self) -> Self;
    fn gcd_lcm(&self, other: &Self) -> (Self, Self)
    where
        Self: Sized;
    fn gcd(&self, other: &Self) -> Self;
}

impl GcdLcm for u64 {
    fn lcm(&self, other: &Self) -> Self {
        self.gcd_lcm(other).1
    }

    fn gcd_lcm(&self, other: &Self) -> (Self, Self) {
        if self == &0 && other == &0 {
            return (0, 0);
        }
        let gcd = self.gcd(other);
        let lcm = *self * (*other / gcd);
        (gcd, lcm)
    }
    fn gcd(&self, other: &Self) -> Self {
        // Use Stein's algorithm
        let mut m = *self;
        let mut n = *other;
        if m == 0 || n == 0 {
            return m | n;
        }

        // find common factors of 2
        let shift = (m | n).trailing_zeros();

        // divide n and m by 2 until odd
        m >>= m.trailing_zeros();
        n >>= n.trailing_zeros();

        while m != n {
            if m > n {
                m -= n;
                m >>= m.trailing_zeros();
            } else {
                n -= m;
                n >>= n.trailing_zeros();
            }
        }
        m << shift
    }
}
