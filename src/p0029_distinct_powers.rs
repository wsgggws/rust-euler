// Distinct powers
// Problem 29
// Consider all integer combinations of ab for 2 ≤ a ≤ 5 and 2 ≤ b ≤ 5:

// 22=4, 23=8, 24=16, 25=32
// 32=9, 33=27, 34=81, 35=243
// 42=16, 43=64, 44=256, 45=1024
// 52=25, 53=125, 54=625, 55=3125
// If they are then placed in numerical order, with any repeats removed, we get the following sequence of 15 distinct terms:

// 4, 8, 9, 16, 25, 27, 32, 64, 81, 125, 243, 256, 625, 1024, 3125

// How many distinct terms are in the sequence generated by ab for 2 ≤ a ≤ 100 and 2 ≤ b ≤ 100?

use std::collections::HashSet;
pub struct Solution {}

impl Solution {
    pub fn distinct_powers(start: u32, end: u32) -> usize {
        let mut collections: HashSet<Vec<_>> = HashSet::new();
        // 得到100以内的质数
        let primes = Solution::get_primes(100);
        for a in start..=end {
            let prime_divictors = Solution::get_prime_divictors(a, &primes);
            for b in start..=end {
                let changed_prime_divictors = prime_divictors
                    .clone()
                    .iter()
                    .map(|&(x, y)| (x, y * b))
                    .collect();
                collections.insert(changed_prime_divictors);
            }
        }
        collections.len()
    }

    fn get_primes(number: u32) -> Vec<u32> {
        (2..number)
            .filter(|&num| Solution::is_prime(num))
            .collect::<Vec<u32>>()
    }

    fn is_prime(num: u32) -> bool {
        if num == 2 || num == 3 {
            return true;
        }
        !(2..=(num as f64).sqrt().ceil() as u32)
            .any(|value| num % value == 0)
    }

    fn get_prime_divictors(number: u32, primes: &Vec<u32>) -> Vec<(u32, u32)> {
        let mut number = number;
        let mut prime_divictors: Vec<(u32, u32)> = Vec::new();
        for &prime in primes {
            if prime > number {
                return prime_divictors;
            } else if number % prime == 0 {
                let mut prime_count = 0u32;
                while number % prime == 0 {
                    number /= prime;
                    prime_count += 1;
                }
                prime_divictors.push((prime, prime_count));
            }
        }
        prime_divictors
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn distinct_powers_test() {
        assert_eq!(Solution::distinct_powers(2, 5), 15);
        assert_eq!(Solution::distinct_powers(2, 6), 23);
        assert_eq!(Solution::distinct_powers(2, 100), 9183);
    }
}
