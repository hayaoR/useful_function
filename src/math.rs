// O(log max(x, y))
pub fn gcd(x: isize, y: isize) -> isize {
    return if y != 0 { gcd(y, x % y) } else { x };
}

pub fn lcm(x: isize, y: isize) -> isize {
    x / gcd(x, y) * y
}

pub fn is_prime(n: usize) -> bool {
    if n == 1 {
        return false;
    }
    let mut i = 2;
    while i * i <= n {
        if n % i == 0 {
            return false;
        }
        i += 1;
    }
    true
}

// used in abc_152_e
use std::collections::HashMap;
use MyEither::{IsPrime, NotPrime, Undecided};

#[derive(Clone, Copy, PartialEq)]
pub enum MyEither<T> {
    IsPrime,
    NotPrime(T),
    Undecided,
}

pub fn make_sieve(limit: usize) -> Vec<MyEither<usize>> {
    let mut sieve: Vec<MyEither<usize>> = vec![Undecided; limit + 1];
    let mut num = 2;

    while num <= limit {
        if sieve[num] == Undecided {
            sieve[num] = IsPrime;
            let mut tmp = num * num;
            while tmp <= limit {
                sieve[tmp] = match &sieve[tmp] {
                    Undecided => NotPrime(num),
                    n => *n,
                };
                tmp += num;
            }
        }
        num += 1;
    }
    sieve
}

pub fn prime_factorize_fast(mut n: usize, sieve: &Vec<MyEither<usize>>) -> HashMap<usize, usize> {
    let mut res = HashMap::new();

    while sieve[n] != IsPrime {
        match &sieve[n] {
            NotPrime(num) => {
                let entry = res.entry(*num).or_insert(0);
                *entry += 1;
                n = n / *num;
            }
            _ => break,
        }
    }
    let entry = res.entry(n).or_insert(0);
    *entry += 1;

    res
}

pub fn prime_factorize(mut n: usize) -> Vec<(usize, usize)> {
    let mut res = vec![];
    let mut a = 2;
    while a * a <= n {
        if n % a != 0 {
            a += 1;
            continue;
        }
        let mut ex = 0;

        while n % a == 0 {
            ex += 1;
            n /= a;
        }

        res.push((a, ex));
        a += 1;
    }
    if n != 1 {
        res.push((n, 1));
    }

    res
}

//　約数列挙
pub fn enum_divisors(n: usize) -> Vec<usize> {
    let mut res = vec![];
    let mut i = 1;
    while i * i <= n {
        if n % i == 0 {
            res.push(i);
            if n / i != i {
                res.push(n / i);
            }
        }
        i += 1;
    }
    res.sort();
    res
}

pub fn chmax<T: Ord>(a: &mut T, b: T) -> bool {
    if *a < b {
        *a = b;
        return true;
    }
    false
}

pub fn chmin<T: Ord>(a: &mut T, b: T) -> bool {
    if *a > b {
        *a = b;
        return true;
    }
    false
}

pub struct COM {
    fac: Vec<isize>,
    finv: Vec<isize>,
    inv: Vec<isize>,
    max: usize,
    m: isize,
}

impl COM {
    pub fn new(max: usize, m: isize) -> Self {
        let mut com = COM {
            fac: vec![0; max],
            finv: vec![0; max],
            inv: vec![0; max],
            max: max,
            m: m,
        };
        com.init();
        com
    }

    pub fn init(&mut self) {
        self.fac[0] = 1;
        self.fac[1] = 1;
        self.finv[0] = 1;
        self.finv[1] = 1;
        self.inv[0] = 0;
        self.inv[1] = 1;

        let mut i = 2;
        while i < self.max {
            self.fac[i] = self.fac[i - 1] * i as isize % self.m;
            self.inv[i] =
                self.m as isize - self.inv[self.m as usize % i] * (self.m / i as isize) % self.m;
            self.finv[i] = self.finv[i - 1] * self.inv[i] % self.m;
            i += 1;
        }
    }

    pub fn com(&self, n: usize, k: usize) -> isize {
        if n < k {
            return 0;
        }
        self.fac[n] * (self.finv[k] * self.finv[n - k] % self.m) % self.m
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_prime() {
        for i in 1..10000 {
            assert_eq!(primes::is_prime(i), is_prime(i as usize));
        }
    }

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(36, 24), 12);
        assert_eq!(gcd(12323, 3424), 1);
    }

    #[test]
    fn test_lcm() {
        assert_eq!(lcm(8, 12), 24);
        assert_eq!(lcm(64, 126), 4032);
    }

    #[test]
    fn test_com() {
        let max = 510000;
        let m = 1000000007;

        let com = COM::new(max, m);

        assert_eq!(com.com(100000, 50000), 149033233);
    }
}
