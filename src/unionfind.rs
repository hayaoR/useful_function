/// petgraphクレートにunion findは存在するがグループのサイズを知る方法が分からないためグループのサイズが分かるunion findを実装した
pub struct UnionFind {
    par: Vec<isize>,
    rank: Vec<usize>,
}

impl UnionFind {
    pub fn new(n: usize) -> Self {
        UnionFind {
            par: vec![-1; n],
            rank: vec![0; n],
        }
    }

    pub fn root(&mut self, x: usize) -> usize {
        if self.par[x] < 0 {
            x
        } else {
            self.par[x] = self.root(self.par[x] as usize) as isize;
            self.par[x] as usize
        }
    }

    pub fn same(&mut self, x: usize, y: usize) -> bool {
        self.root(x) == self.root(y)
    }

    pub fn size(&mut self, x: usize) -> isize {
        let root = self.root(x);
        self.par[root] * -1
    }

    pub fn unite(&mut self, mut x: usize, mut y: usize) {
        x = self.root(x);
        y = self.root(y);
        if x == y {
            return;
        }

        if self.rank[x] < self.rank[y] {
            self.par[y] += self.par[x];
            self.par[x] = y as isize;
        } else {
            self.par[x] += self.par[y];
            self.par[y] = x as isize;
            if self.rank[x] == self.rank[y] {
                self.rank[x] += 1;
            }
        }
    }
}
