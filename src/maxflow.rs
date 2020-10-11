use std::cmp;

#[derive(Clone, Copy)]
struct Edge {
    to: usize,
    cap: usize,
    rev: usize,
}

pub struct Maxflow {
    graph: Vec<Vec<Edge>>,
    used: Vec<bool>,
    max_v: usize,
}

impl Maxflow {
    pub fn new(max_v: usize) -> Self {
        Maxflow {
            graph: vec![vec![]; max_v],
            used: vec![false; max_v],
            max_v: max_v,
        }
    }

    pub fn add_edge(&mut self, from: usize, to: usize, cap: usize) {
        let rev = self.graph[to].len();
        self.graph[from].push(Edge { to, cap, rev });
        let rev = self.graph[from].len() - 1;
        self.graph[to].push(Edge {
            to: from,
            cap: 0,
            rev,
        });
    }

    fn dfs(&mut self, v: usize, t: usize, f: usize) -> usize {
        if v == t {
            return f;
        }
        self.used[v] = true;
        for i in 0..self.graph[v].len() {
            if !self.used[self.graph[v][i].to] && self.graph[v][i].cap > 0 {
                let d = self.dfs(self.graph[v][i].to, t, cmp::min(f, self.graph[v][i].cap));
                if d > 0 {
                    self.graph[v][i].cap -= d;
                    let to = self.graph[v][i].to;
                    let rev = self.graph[v][i].rev;
                    self.graph[to][rev].cap += d;
                    return d;
                }
            }
        }
        0
    }

    fn clear(&mut self) {
        self.used = vec![false; self.max_v]
    }

    pub fn execute(&mut self, s: usize, t: usize) -> usize {
        let mut flow = 0;
        let inf = std::usize::MAX;
        loop {
            self.clear();
            let f = self.dfs(s, t, inf);
            if f == 0 {
                return flow;
            }
            flow += f;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_maxflow() {
        let v = 4;
        let e = 5;
        let edges = vec![(0, 1, 2), (0, 2, 1), (1, 2, 1), (1, 3, 1), (2, 3, 2)];

        let mut mf = Maxflow::new(v);
        for &e in &edges {
            mf.add_edge(e.0, e.1, e.2);
        }
        let result = mf.execute(0, v - 1);
        assert_eq!(result, 3);
    }
}
