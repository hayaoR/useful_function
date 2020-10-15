/// 区間は0-indexed
/// eはopの単位元 op(a, e) = op(e, a) = a
/// idはmappingの恒等写像 mapping(id, a) = a
pub struct LazySegTree<T, F> {
    n: usize,
    dat: Vec<T>,
    lazy: Vec<F>,
    e: T,
    id: F,
    op: fn(T, T) -> T,
    mapping: fn(F, T) -> T,
    composition: fn(F, F) -> F,
}

impl<T, F> LazySegTree<T, F>
where
    T: PartialEq + Clone + Copy,
    F: PartialEq + Clone + Copy,
{
    pub fn new(
        _n: usize,
        e: T,
        id: F,
        op: fn(T, T) -> T,
        mapping: fn(F, T) -> T,
        composition: fn(F, F) -> F,
    ) -> Self {
        let mut n = 1;
        while n < _n {
            n *= 2;
        }

        LazySegTree {
            n: n,
            dat: vec![e; 2 * n - 1],
            lazy: vec![id; 2 * n - 1],
            e,
            id,
            op,
            mapping,
            composition,
        }
    }

    pub fn set(&mut self, i: usize, x: T) {
        let n = self.get_n();
        self.dat[i + n - 1] = x;
    }

    pub fn build(&mut self) {
        if self.get_n() < 2 {
            return;
        }
        let mut k = self.get_n() - 2;
        loop {
            self.dat[k] = (self.op)(self.dat[2 * k + 1], self.dat[2 * k + 2]);
            if k == 0 {
                break;
            }
            k -= 1;
        }
    }

    pub fn eval(&mut self, k: usize) {
        if self.lazy[k] == self.id {
            return;
        }

        if k < self.n - 1 {
            self.lazy[k * 2 + 1] = (self.composition)(self.lazy[k * 2 + 1], self.lazy[k]);
            self.lazy[k * 2 + 2] = (self.composition)(self.lazy[k * 2 + 2], self.lazy[k]);
        }

        self.dat[k] = (self.mapping)(self.lazy[k], self.dat[k]);
        self.lazy[k] = self.id;
    }

    /// [a, b)をxに更新?
    /// update(a, b, 0, 0, x, n)みたいに呼ぶ <- やめた
    fn update_inner(&mut self, a: usize, b: usize, x: F, k: usize, l: usize, r: usize) {
        self.eval(k);
        if a <= l && r <= b {
            self.lazy[k] = (self.composition)(self.lazy[k], x);
            self.eval(k);
        } else if a < r && l < b {
            self.update_inner(a, b, x, k * 2 + 1, l, (l + r) / 2);
            self.update_inner(a, b, x, k * 2 + 2, (l + r) / 2, r);
            self.dat[k] = (self.op)(self.dat[k * 2 + 1], self.dat[k * 2 + 2]);
        }
    }

    pub fn update(&mut self, a: usize, b: usize, x: F) {
        let n = self.get_n();
        self.update_inner(a, b, x, 0, 0, n)
    }

    /// queryは[a, b)に対して何かを返す
    /// 外から呼ぶときはquery(a, b, 0, 0, n)と呼ぶ <- 止めた
    fn query_inner(&mut self, a: usize, b: usize, k: usize, l: usize, r: usize) -> T {
        self.eval(k);
        if r <= a || b <= l {
            return self.e;
        }
        if a <= l && r <= b {
            return self.dat[k];
        } else {
            let vl = self.query_inner(a, b, k * 2 + 1, l, (l + r) / 2);
            let vr = self.query_inner(a, b, k * 2 + 2, (l + r) / 2, r);
            return (self.op)(vl, vr);
        }
    }

    pub fn query(&mut self, a: usize, b: usize) -> T {
        let n = self.get_n();
        self.query_inner(a, b, 0, 0, n)
    }

    pub fn get_n(&self) -> usize {
        self.n
    }
}
