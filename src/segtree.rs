use num::Num;

/// opがminならrange minumum query maxならrange maximum query
/// 区間は0-indexed
pub struct SegTree<T> {
    n: usize,
    dat: Vec<T>,
    e: T,
    op: fn(T, T) -> T,
}

impl<T> SegTree<T>
where
    T: Num + Clone + Copy,
{
    pub fn new(_n: usize, e: T, op: fn(T, T) -> T) -> Self {
        let mut n = 1;
        while n < _n {
            n *= 2;
        }

        SegTree {
            n: n,
            dat: vec![e; 2 * n - 1],
            e: e,
            op: op,
        }
    }

    pub fn update(&mut self, mut k: usize, a: T) {
        k += self.n - 1;
        self.dat[k] = a;
        while k > 0 {
            k = (k - 1) / 2;
            self.dat[k] = (self.op)(self.dat[k * 2 + 1], self.dat[k * 2 + 2]);
        }
    }

    /// queryは[a, b)の最大値を返す
    /// 外から呼ぶときはquery(a, b, 0, 0, n)と呼ぶ
    pub fn query(&self, a: usize, b: usize, k: usize, l: usize, r: usize) -> T {
        if r <= a || b <= l {
            return self.e;
        }
        if a <= l && r <= b {
            return self.dat[k];
        } else {
            let vl = self.query(a, b, k * 2 + 1, l, (l + r) / 2);
            let vr = self.query(a, b, k * 2 + 2, (l + r) / 2, r);
            return (self.op)(vl, vr);
        }
    }

    ///queryを呼ぶときに使う
    pub fn get_n(&self) -> usize {
        self.n
    }
}
