// https://www.codewars.com/kata/526989a41034285187000de4

pub mod answer {
    pub fn ips_between(start: &str, end: &str) -> u32 {
        fn ip_to_u32(ip: &str) -> u32 {
            ip.split('.')
                .map(|part| part.parse::<u32>().expect("Invalid input detected!"))
                .fold(0, |sum, item| (sum << 8) | item)
        }

        let start_ip = ip_to_u32(start);
        let end_ip = ip_to_u32(end);

        println!("Start IP {}, End IP {}", start_ip, end_ip);

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
