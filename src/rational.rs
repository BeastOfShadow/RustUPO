use std::ops::{Add, Mul};
use std::fmt::{Display};
use std::cmp::{Ordering, PartialOrd};

impl Display for Rational {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}/{}",
            if self.sign == Sign::Positive {
                '+'
            } else {
                '-'
            },
            self.num,
            self.den
        )
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Sign {
    Positive,
    Negative,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Rational {
    pub num: u32,
    pub den: u32,
    pub sign: Sign,
}

impl Rational {
    pub fn sign_checker(n1: i32, n2: i32) -> Sign {
        if (n1 < 0) == (n2 < 0) {
            Sign::Positive
        } else {
            Sign::Negative
        }
    }

    pub fn new(num: i32, den: i32) -> Self {
        let sign = Self::sign_checker(num, den);

        let den = if den == 0 { 1 } else { den };
        let mcd_val = Rational::mcd(num.abs(), den.abs());

        Rational {
            num: (num.abs() / mcd_val) as u32,
            den: (den.abs() / mcd_val) as u32,
            sign: sign,
        }
    }

    pub fn product(&self, rat: &Self) -> Self {
        let num = self.num * rat.num;
        let den = self.den * rat.den;

        let sign = if self.sign == rat.sign {
            Sign::Positive
        } else {
            Sign::Negative
        };

        let val_mcd = Rational::mcd((num as i32).abs(), (den as i32).abs()) as u32;

        Rational {
            num: num / val_mcd,
            den: den / val_mcd,
            sign: sign.clone(),
        }
    }

    pub fn sum(&self, rat: &Self) -> Self {
        let den = self.den * rat.den;

        let self_num = self.num * rat.den;
        let rat_num = rat.num * self.den;

        let sign = if self_num >= rat_num {
            &self.sign
        } else {
            &rat.sign
        };

        let num = if self.sign == rat.sign {
            self_num + rat_num
        } else if self_num >= rat_num {
            self_num - rat_num
        } else {
            rat_num - self_num
        };

        let val_mcd = Rational::mcd((num as i32).abs(), (den as i32).abs()) as u32;

        Rational {
            num: num / val_mcd,
            den: den / val_mcd,
            sign: sign.clone(),
        }
    }

    pub fn inverse(&self) -> Self {
        let new_num = self.den;
        let new_den = self.num;

        Rational {
            num: new_num,
            den: new_den,
            sign: self.sign.clone(),
        }
    }

    pub fn match_f(&self, rat: &Self) -> i32 {
        let val_self_mcd = Rational::mcd((self.num as i32).abs(), (self.den as i32).abs()) as u32;
        let val_rat_mcd = Rational::mcd((rat.num as i32).abs(), (rat.den as i32).abs()) as u32;

        if self.sign == rat.sign {
            let l_prod = (self.num / val_self_mcd) as u64 * (rat.den / val_rat_mcd) as u64;
            let r_prod = (rat.num / val_rat_mcd) as u64 * (self.den /val_self_mcd) as u64;

            if self.num == rat.num && self.den == rat.den {
                0
            } else if (self.sign == Sign::Positive && l_prod > r_prod)
                || (self.sign == Sign::Negative && l_prod < r_prod)
            {
                1
            } else {
                -1
            }
        } else if self.sign == Sign::Positive && rat.sign == Sign::Negative {
            1
        } else {
            -1
        }
    }

    pub fn int_to_rational(n: i32) -> Self {
        Rational { num: n.abs() as u32, den: 1, sign: if n >= 0 { Sign::Positive } else { Sign::Negative } }
    }

    pub fn mcd(mut a: i32, mut b: i32) -> i32 {
        while b != 0 {
            let r = a % b;
            a = b;
            b = r;
        }

        a
    }
}

impl Add for Rational {
    type Output = Self;

    fn add(self, rat: Self) -> Self::Output {
        self.sum(&rat)
    }
}

impl Mul for Rational {
    type Output = Self;

    fn mul(self, rat: Self) -> Self::Output {
        self.product(&rat)
    }
}

impl PartialOrd for Rational {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.match_f(other) {
            1 => Some(Ordering::Greater),
            0 => Some(Ordering::Equal),
            -1 => Some(Ordering::Less),
            _ => None,
        }
    }
}