use std::mem;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

///テストしていないのでバグが含まれているかもしれない
/// 負の数は例えば-6%MOD=-6になる
/// MODは適宜調整する。
const MOD: isize = 1_000_000_007;
#[derive(Clone, Copy)]
pub struct Mint {
    x: isize,
}

impl Mint {
    pub fn new(x: isize) -> Self {
        Mint { x: x % MOD }
    }

    pub fn pow(&self, n: isize) -> Self {
        let mut n = n;
        let mut res = Mint::new(1);
        let mut x = self.clone();
        while n > 0 {
            if n & 1 != 0 {
                res = res * x;
            }
            x = x * x;
            n >>= 1;
        }
        res
    }

    pub fn inv(&self) -> isize {
        let mut b = MOD;
        let mut u = 1;
        let mut v = 0;
        let mut x = self.x;
        while b != 0 {
            let t = x / b;
            x -= t * b;
            mem::swap(&mut x, &mut b);
            u -= t * v;
            mem::swap(&mut u, &mut v);
        }

        u %= MOD;

        if u < 0 {
            u += MOD;
        }

        u
    }

    pub fn val(&self) -> isize {
        self.x
    }
}

impl AddAssign<isize> for Mint {
    fn add_assign(&mut self, rhs: isize) {
        self.x = (self.x + rhs) % MOD;
    }
}

impl AddAssign for Mint {
    fn add_assign(&mut self, rhs: Self) {
        self.x = (self.x + rhs.x) % MOD;
    }
}

impl SubAssign<isize> for Mint {
    fn sub_assign(&mut self, rhs: isize) {
        self.x = (self.x + MOD - rhs) % MOD;
    }
}

impl SubAssign for Mint {
    fn sub_assign(&mut self, rhs: Self) {
        self.x = (self.x + MOD - rhs.x) % MOD;
    }
}

impl MulAssign<isize> for Mint {
    fn mul_assign(&mut self, rhs: isize) {
        self.x = self.x * rhs % MOD;
    }
}

impl MulAssign for Mint {
    fn mul_assign(&mut self, rhs: Self) {
        self.x = self.x * rhs.x % MOD;
    }
}

impl Add<isize> for Mint {
    type Output = Self;
    fn add(self, rhs: isize) -> Self {
        Mint {
            x: (self.x + rhs) % MOD,
        }
    }
}

impl DivAssign<isize> for Mint {
    fn div_assign(&mut self, rhs: isize) {
        let mint = Mint { x: rhs };
        self.x = self.x * mint.inv() % MOD;
    }
}

impl DivAssign for Mint {
    fn div_assign(&mut self, rhs: Self) {
        self.x = self.x * rhs.inv() % MOD;
    }
}

impl Add for Mint {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Mint {
            x: (self.x + rhs.x) % MOD,
        }
    }
}

impl Sub<isize> for Mint {
    type Output = Self;
    fn sub(self, rhs: isize) -> Self {
        Mint {
            x: (self.x + MOD - rhs) % MOD,
        }
    }
}

impl Sub for Mint {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Mint {
            x: (self.x + MOD - rhs.x) % MOD,
        }
    }
}

impl Mul<isize> for Mint {
    type Output = Self;
    fn mul(self, rhs: isize) -> Self {
        Mint {
            x: self.x * rhs % MOD,
        }
    }
}

impl Mul for Mint {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Mint {
            x: self.x * rhs.x % MOD,
        }
    }
}

impl Div<isize> for Mint {
    type Output = Self;
    fn div(self, rhs: isize) -> Self {
        let mint = Mint { x: rhs };
        Mint {
            x: self.x * mint.inv() % MOD,
        }
    }
}

impl Div for Mint {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        Mint {
            x: self.x * rhs.inv() % MOD,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_modpow() {
        let n = Mint::new(3);
        let result = n.pow(45);
        assert_eq!(result.val(), 644897553);
    }
}
