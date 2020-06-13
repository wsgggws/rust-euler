pub struct Solution {}


impl Solution {
    pub fn multiples_of_3_and_5() -> u32 {
        (1..1000).filter(|x| x%3 == 0 || x%5 == 0).sum::<u32>()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn multiples_of_3_and_5_test() {
        assert_eq!(Solution::multiples_of_3_and_5(), 233168);
    }
}
