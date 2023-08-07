struct Solution;

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        let mut rows = vec![String::new(); num_rows as usize];

        let indices = (0..num_rows)
            .chain(
                (1..num_rows)
                .rev()
                .skip(1)
            )
            .cycle()
            .map(|idx| idx as usize);

        let indexed_chars = indices.zip(s.chars());

        for (idx, char) in indexed_chars {
            rows[idx].push(char);
        }

        rows.as_slice().join("")
    }
}

#[cfg(test)]
mod _6 {
    use crate::_6_zigzag_conversion::Solution;
    #[test]
    fn case1() {
        let ans = Solution::convert("PAYPALISHIRING".to_string(), 3);
        let correct = "PAHNAPLSIIGYIR";
        assert_eq!(ans, correct);
    }
    #[test]
    fn case2() {
        let ans = Solution::convert("PAYPALISHIRING".to_string(), 4);
        let correct = "PINALSIGYAHRPI";
        assert_eq!(ans, correct);
    }
    #[test]
    fn case3() {
        let ans = Solution::convert("A".to_string(), 1);
        let correct = "A";
        assert_eq!(ans, correct);
    }
}