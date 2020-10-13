use std::mem;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

///テストしていないのでバグが含まれているかもしれない
#[derive(Clone, Copy)]
pub struct Mint {
    x: isize,
    mo: isize,
}

impl Mint {
    pub fn new(x: isize, mo: isize) -> Self {
        Mint { x: x % mo, mo }
    }

    pub fn pow(&self, n: isize) -> Self {
        let mut n = n;
        let mut res = Mint::new(1, self.mo);
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
        let mut b = self.mo;
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

        u %= self.mo;

        if u < 0 {
            u += self.mo;
        }

        u
    }

    pub fn val(&self) -> isize {
        self.x
    }
}

impl AddAssign<isize> for Mint {
    fn add_assign(&mut self, rhs: isize) {
        self.x = (self.x + rhs) % self.mo;
    }
}

impl AddAssign for Mint {
    fn add_assign(&mut self, rhs: Self) {
        self.x = (self.x + rhs.x) % self.mo;
    }
}

impl SubAssign<isize> for Mint {
    fn sub_assign(&mut self, rhs: isize) {
        self.x = (self.x + self.mo - rhs) % self.mo;
    }
}

impl SubAssign for Mint {
    fn sub_assign(&mut self, rhs: Self) {
        self.x = (self.x + self.mo - rhs.x) % self.mo;
    }
}

impl MulAssign<isize> for Mint {
    fn mul_assign(&mut self, rhs: isize) {
        self.x = self.x * rhs % self.mo;
    }
}

impl MulAssign for Mint {
    fn mul_assign(&mut self, rhs: Self) {
        self.x = self.x * rhs.x % self.mo;
    }
}

impl Add<isize> for Mint {
    type Output = Self;
    fn add(self, rhs: isize) -> Self {
        Mint {
            x: (self.x + rhs) % self.mo,
            mo: self.mo,
        }
    }
}

impl DivAssign<isize> for Mint {
    fn div_assign(&mut self, rhs: isize) {
        let mint = Mint {
            x: rhs,
            mo: self.mo,
        };
        self.x = self.x * mint.inv() % self.mo;
    }
}

impl DivAssign for Mint {
    fn div_assign(&mut self, rhs: Self) {
        self.x = self.x * rhs.inv() % self.mo;
    }
}

impl Add for Mint {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Mint {
            x: (self.x + rhs.x) % self.mo,
            mo: self.mo,
        }
    }
}

impl Sub<isize> for Mint {
    type Output = Self;
    fn sub(self, rhs: isize) -> Self {
        Mint {
            x: (self.x + self.mo - rhs) % self.mo,
            mo: self.mo,
        }
    }
}

impl Sub for Mint {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Mint {
            x: (self.x + self.mo - rhs.x) % self.mo,
            mo: self.mo,
        }
    }
}

impl Mul<isize> for Mint {
    type Output = Self;
    fn mul(self, rhs: isize) -> Self {
        Mint {
            x: self.x * rhs % self.mo,
            mo: self.mo,
        }
    }
}

impl Mul for Mint {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Mint {
            x: self.x * rhs.x % self.mo,
            mo: self.mo,
        }
    }
}

impl Div<isize> for Mint {
    type Output = Self;
    fn div(self, rhs: isize) -> Self {
        let mint = Mint {
            x: rhs,
            mo: self.mo,
        };
        Mint {
            x: self.x * mint.inv() % self.mo,
            mo: self.mo,
        }
    }
}

impl Div for Mint {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        Mint {
            x: self.x * rhs.inv() % self.mo,
            mo: self.mo,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_modpow() {
        let mo = 1_000_000_007;
        let n = Mint::new(3, mo);
        let result = n.pow(45);
        assert_eq!(result.val(), 644897553);
    }
}
