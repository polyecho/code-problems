// https://www.codewars.com/kata/513e08acc600c94f01000001

pub mod answer {
    fn clamp(input: i32) -> u8 {
        match input {
            value if value < 0 => 0,
            value if value > 255 => 255,
            value => value as u8,
        }
    }

    pub fn rgb(r: i32, g: i32, b: i32) -> String {
        let (r, g, b): (u8, u8, u8) = (clamp(r), clamp(g), clamp(b));

        let (r_hex, g_hex, b_hex) = (
            format!("{:02X}", r),
            format!("{:02X}", g),
            format!("{:02X}", b),
        );
        let result = format!("{}{}{}", r_hex, g_hex, b_hex);

        result
    }
}

#[cfg(test)]
mod sample_tests {
    use super::answer::rgb;

    #[test]
    fn tests() {
        assert_eq!(rgb(0, 0, 0), "000000");
        assert_eq!(rgb(1, 2, 3), "010203");
        assert_eq!(rgb(255, 255, 255), "FFFFFF");
        assert_eq!(rgb(254, 253, 252), "FEFDFC");
        assert_eq!(rgb(-20, 275, 125), "00FF7D");
    }
}
