pub fn stars(n: u32) -> String {
    "*".repeat(2usize.pow(n))
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stars() {
        assert_eq!(stars(1), "**");
        println!("{}", stars(1));
        assert_eq!(stars(4), "****************");
        println!("{}", stars(4));
        assert_eq!(stars(5), "********************************");
        println!("{}", stars(5));
    }
}