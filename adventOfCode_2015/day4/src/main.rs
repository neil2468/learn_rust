fn main() {
    println!("number = {}", find_number(&"yzbqklnj"));
    println!("number = {}", find_number_part_two(&"yzbqklnj"));
}

fn find_number(secret_key: &str) -> u32 {
    for number in 1..u32::MAX {
        let mut data: String = secret_key.to_owned();
        data.push_str(&number.to_string());

        let digest = md5::compute(&data);

        if digest[0] == 0x00 && digest[1] == 0x00 && digest[2] <= 0x0f {
            println!("{:?} {:?}", data, digest);
            return number;
        }
    }
    panic!();
}

fn find_number_part_two(secret_key: &str) -> u32 {
    for number in 1..u32::MAX {
        let mut data: String = secret_key.to_owned();
        data.push_str(&number.to_string());

        let digest = md5::compute(&data);

        if digest[0] == 0x00 && digest[1] == 0x00 && digest[2] == 0x00 {
            println!("{:?} {:?}", data, digest);
            return number;
        }
    }
    panic!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one() {
        assert_eq!(find_number(&"abcdef"), 609043);
    }

    #[test]
    fn two() {
        assert_eq!(find_number(&"pqrstuv"), 1048970);
    }
}
