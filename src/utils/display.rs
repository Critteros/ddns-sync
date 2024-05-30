pub fn mask_display(mask: &str) -> String {
    let mut masked = String::new();
    for (i, c) in mask.chars().enumerate() {
        if i < 4 {
            masked.push(c);
        } else {
            masked.push('*');
        }
    }
    masked
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case("1234567890", "1234******")]
    #[case("1234", "1234")]
    #[case("12345", "1234*")]
    fn test_mask_display(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(mask_display(input), expected);
    }
}
