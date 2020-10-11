use num::Num;

pub struct BIT<T> {
    pub n: usize,
    pub d: Vec<T>,
}

impl<T> BIT<T>
where
    T: Num + Copy + Clone,
{
    pub fn new(n: usize) -> Self {
        BIT {
            n: n,
            d: vec![T::zero(); n + 1],
        }
    }

    pub fn add(&mut self, mut i: usize, x: T) {
        i += 1;
        while i <= self.n {
            self.d[i] = self.d[i] + x;
            i += i & i.wrapping_neg();
        }
    }

    //多分[0, i]の和を求める
    pub fn sum_one(&self, mut _i: isize) -> T {
        let mut x = T::zero();
        _i += 1;
        let mut i = _i as usize;
        while i != 0 {
            x = x + self.d[i];
            i -= i & i.wrapping_neg();
        }
        x
    }

    // [l, r)の和を求める(rは含まない)
    pub fn sum(&self, l: isize, r: isize) -> T {
        self.sum_one(r - 1) - self.sum_one(l - 1)
    }
}
