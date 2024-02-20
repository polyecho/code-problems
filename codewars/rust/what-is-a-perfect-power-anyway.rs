// https://www.codewars.com/kata/54d4c8b08776e4ad92000835

pub mod answer {
    pub fn is_perfect_power(n: u64) -> Option<(u64, u32)> {
        for i in 2..(get_floor_sqrt(n) + 1) {
            let mut j = 2;
            let mut powered = i.pow(j);

            while powered <= n {
                powered = i.pow(j);

                if powered == n {
                    return Some((i, j));
                }

                j += 1
            }
        }

        None
    }

    fn get_floor_sqrt(n: u64) -> u64 {
        f64::sqrt(n as f64) as u64
    }
}

#[cfg(test)]
mod tests {
    use super::answer::is_perfect_power;

    #[test]
    fn basic_examples() {
        assert_eq!(is_perfect_power(4), Some((2, 2)), "4 = 2^2");
        assert_eq!(is_perfect_power(9), Some((3, 2)), "9 = 3^2");
        assert_eq!(is_perfect_power(5), None, "5 is not a perfect power");
    }

    #[test]
    fn first_perfect_powers() {
        let pp = &[
            4, 8, 9, 16, 25, 27, 32, 36, 49, 64, 81, 100, 121, 125, 128, 144, 169, 196, 216, 225,
            243, 256, 289, 324, 343, 361, 400, 441, 484,
        ];
        for p in pp {
            if is_perfect_power(*p) == None {
                assert!(false, "{} wasn't recognized as a perfect power", p)
            } else {
                assert!(true)
            }
        }
    }
}
