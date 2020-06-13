// Even Fibonacci numbers
// Problem 2
// Each new term in the Fibonacci sequence is generated by adding the previous two terms. By starting with 1 and 2, the first 10 terms will be:

// 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, ...

// By considering the terms in the Fibonacci sequence whose values do not exceed four million, find the sum of the even-valued terms.

pub struct Solution {}

struct Fib {
    x: (u64, u64),
}

impl Fib {
    fn new() -> Fib {
        Fib { x: (1, 1) }
    }
}

impl Iterator for Fib {
    type Item = u64;
    fn next(&mut self) -> Option<Self::Item> {
        self.x = (self.x.1, self.x.0 + self.x.1);
        Some(self.x.0)
    }
}

impl Solution {
    pub fn even_fibonacci_numbers(num: u64) -> u64 {
        Fib::new().take_while(|&a| a<=num).filter(|&a| a % 2 == 0).sum::<u64>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn even_fibonacci_numbers_test() {
        // 1, 2, 3, 5 => 2
        assert_eq!(Solution::even_fibonacci_numbers(5), 2);
        // 1, 2, 3, 5, 8 => 2, 8 => 10
        assert_eq!(Solution::even_fibonacci_numbers(8), 10);
        assert_eq!(Solution::even_fibonacci_numbers(4000000u64), 4613732);
    }
}
