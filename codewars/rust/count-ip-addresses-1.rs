// https://www.codewars.com/kata/526989a41034285187000de4

pub mod answer {
    pub fn ips_between(start: &str, end: &str) -> u32 {
        fn ip_to_u32(ip: &str) -> u32 {
            let splitted = ip
                .split('.')
                .map(|part| part.parse::<u64>().expect("Invalid input detected!"));

            let mut output: u64 = 0;
            let mut power_number = 3;

            for item in splitted {
                if power_number != 0 {
                    output += item * u64::pow(256, power_number);
                    power_number -= 1;
                } else {
                    output += item;
                }
            }

            output as u32
        }

        let start_ip = ip_to_u32(start);
        let end_ip = ip_to_u32(end);

        if start_ip > end_ip {
            panic!("start_ip cannot be bigger than end_ip");
        }

        end_ip - start_ip
    }
}

#[cfg(test)]
mod tests {
    use super::answer::ips_between;

    #[test]
    fn basic() {
        assert_eq!(ips_between("10.0.0.0", "10.0.0.50"), 50);
        assert_eq!(ips_between("20.0.0.10", "20.0.1.0"), 246);
    }
}
