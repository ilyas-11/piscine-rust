pub fn arrange_phrase(phrase: &str) -> String {
    let mut res: Vec<String> = vec![String::new(); phrase.split_whitespace().count()];

    for word in phrase.split_whitespace() {
        let mut pos = 0;
        let mut clean = String::new();

        for c in word.chars() {
            if c.is_numeric() {
                pos = c.to_digit(10).unwrap() as usize;
            } else {
                clean.push(c);
            }
        }

        res[pos - 1] = clean;
    }

    res.join(" ")
}