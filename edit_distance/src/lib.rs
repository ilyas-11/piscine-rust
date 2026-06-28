pub fn edit_distance(source: &str, target: &str) -> usize {
     let s: Vec<char> = source.chars().collect();
    let t: Vec<char> = target.chars().collect();
    let mut dp = vec![vec![0; t.len()+1]; s.len()+1];
    for i in 0..=s.len() {
        dp[i][0] = i;
    }
    for j in 0..=t.len() {
        dp[0][j] = j;
    }
    for i in 1..=s.len() {
        for j in 1..=t.len() {
            if s[i-1] == t[j-1] {
                dp[i][j] = dp[i-1][j-1];
            } else {
                let insert = dp[i][j-1] + 1;
                let delete = dp[i-1][j] + 1;
                let replace = dp[i-1][j-1] + 1;
                dp[i][j] = insert.min(delete).min(replace);
            }
        }
    }
    dp[s.len()][t.len()]
}
