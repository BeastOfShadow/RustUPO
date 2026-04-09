#[cfg(test)]
mod tests {
    use esercizi::rational::{Rational, Sign};

    #[test]
    fn test_new_and_reduction() {
        let rat = Rational::new(10, 20);
        assert_eq!(rat.num, 1);
        assert_eq!(rat.den, 2);
        assert_eq!(rat.sign, Sign::Positive);

        let rat_neg = Rational::new(-12, 6);
        assert_eq!(rat_neg.num, 2);
        assert_eq!(rat_neg.den, 1);
        assert_eq!(rat_neg.sign, Sign::Negative);
    }

    #[test]
    fn test_signs() {
        let rat1 = Rational::new(-5, -5); // Entrambi negativi -> Positivo
        assert_eq!(rat1.sign, Sign::Positive);

        let rat2 = Rational::new(3, -4); // Uno negativo -> Negativo
        assert_eq!(rat2.sign, Sign::Negative);
    }

    #[test]
    fn test_product() {
        let r1 = Rational::new(-7, 6);
        let r2 = Rational::new(-3, 4);
        let prod = r1.product(&r2);
        print!("r1: {}\nr2: {}", r1.to_string(), r2.to_string());
        // (-7/6) * (-3/4) = 21/24 -> ridotto a 7/8
        assert_eq!(prod.num, 7);
        assert_eq!(prod.den, 8);
        assert_eq!(prod.sign, Sign::Positive);
    }

    #[test]
    fn test_sum() {
        let r1 = Rational::new(1, 2);
        let r2 = Rational::new(1, 3);
        let sum = r1.sum(&r2);
        
        // 1/2 + 1/3 = 5/6
        assert_eq!(sum.num, 5);
        assert_eq!(sum.den, 6);
        assert_eq!(sum.sign, Sign::Positive);
    }

    #[test]
    fn test_inverse() {
        let r = Rational::new(-3, 4);
        let inv = r.inverse();
        
        assert_eq!(inv.num, 4);
        assert_eq!(inv.den, 3);
        assert_eq!(inv.sign, Sign::Negative);
    }

    #[test]
    fn test_match_f() {
        let r1 = Rational::new(1, 2);
        let r2 = Rational::new(1, 3);
        assert_eq!(r1.match_f(&r2), 1); // 1/2 > 1/3

        let r3 = Rational::new(-1, 2);
        let r4 = Rational::new(-1, 3);
        assert_eq!(r3.match_f(&r4), -1); // -1/2 < -1/3

        let r5 = Rational::new(2, 4);
        assert_eq!(r1.match_f(&r5), 0); // 1/2 == 2/4
    }

    #[test]
    fn test_int_to_rational() {
        let r = Rational::int_to_rational(-7);
        assert_eq!(r.num, 7);
        assert_eq!(r.den, 1);
        assert_eq!(r.sign, Sign::Negative);
    }

    #[test]
    fn test_operators() {
        let r1 = Rational::new(1, 2);
        let r2 = Rational::new(1, 4);

        // Test Add trait (+)
        let sum = r1.clone() + r2.clone(); 
        assert_eq!(sum, Rational::new(3, 4));

        // Test Mul trait (*)
        let prod = r1 * r2;
        assert_eq!(prod, Rational::new(1, 8));

        // Test PartialOrd traits (>, <, >=, <=)
        assert!(Rational::new(1, 2) > Rational::new(1, 3));
        assert!(Rational::new(-1, 2) < Rational::new(1, 2));
    }
}
