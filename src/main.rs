fn main() {
    println!("Hello, world!");
}

fn convert_ruleset(ruleset: &[i32]) -> i32 {
    let mut total = 0;
    for (i, n) in ruleset.iter().enumerate() {
        if i == 0 {
            total += 1;
            continue;
        }
        total += 2_i32.pow(i as u32) * n;
    }
    return total;
}

#[cfg(test)]
mod tests {
    use crate::convert_ruleset;

    #[test]
    fn converts_ruleset_to_bitmask() {
        let ruleset = [1];
        assert_eq!(1, convert_ruleset(&ruleset));

        let ruleset2 = [1, 1];
        assert_eq!(3, convert_ruleset(&ruleset2));

        let ruleset3 = [1, 0, 1];
        assert_eq!(0b101, convert_ruleset(&ruleset3));

        let ruleset4 = [1, 0, 1, 1, 0, 1];
        assert_eq!(0b101101, convert_ruleset(&ruleset4));
    }
}