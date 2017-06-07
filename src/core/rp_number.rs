use num_bigint::BigInt;
use num_traits::cast::ToPrimitive;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RpNumber {
    pub whole: BigInt,
    pub fraction: Option<BigInt>,
    pub exponent: Option<i32>,
}

impl RpNumber {
    pub fn to_u32(&self) -> Option<u32> {
        let mut n = self.whole.clone();

        if let Some(e) = self.exponent {
            let f: BigInt = 10usize.into();

            for _ in 0..e {
                n = if let Some(n) = n.checked_mul(&f) {
                    n
                } else {
                    return None;
                }
            }
        }

        n.to_u32()
    }

    pub fn to_f64(&self) -> Option<f64> {
        let mut n = if let Some(n) = self.whole.to_f64() {
            n
        } else {
            return None;
        };

        if let Some(ref f) = self.fraction {
            let mut f = if let Some(f) = f.to_f64() {
                f
            } else {
                return None;
            };

            while f > 1f64 {
                f = f / 10f64;
            }

            n += f;
        }

        if let Some(e) = self.exponent {
            if e > 0 {
                for _ in 0..e {
                    n = n * 10f64;
                }
            } else {
                for _ in 0..e.abs() {
                    n = n / 10f64;
                }
            }
        }

        Some(n)
    }
}

impl fmt::Display for RpNumber {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.whole)?;

        if let Some(ref fraction) = self.fraction {
            write!(f, ".{}", fraction)?;
        }

        if let Some(ref exponent) = self.exponent {
            if *exponent != 0 {
                write!(f, "e{}", exponent)?;
            }
        }

        Ok(())
    }
}

impl From<u32> for RpNumber {
    fn from(value: u32) -> RpNumber {
        RpNumber {
            whole: value.into(),
            fraction: None,
            exponent: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exponent() {
        let n = RpNumber {
            whole: 10.into(),
            fraction: Some(4212.into()),
            exponent: 10.into(),
        };

        println!("{}", n);
        println!("{:?}", n.to_f64());
    }
}
