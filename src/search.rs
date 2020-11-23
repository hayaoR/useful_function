use std::collections::VecDeque;
use std::cmp::Ordering;

pub fn bfs(
    start: (usize, usize),
    maze: &Vec<Vec<char>>,
    dist: &mut Vec<Vec<isize>>,
) -> ((usize, usize), isize) {
    //let prev_x = vec![vec![-1; width]; height];
    //let prev_y = vec![vec![-1; width]; height];

    let height = maze.len();
    let width = maze[0].len();
    let mut maximum = 0;
    let mut goal = (0, 0);

    let dx = vec![1, 0, -1, 0];
    let dy = vec![0, 1, 0, -1];

    let mut que = VecDeque::new();
    que.push_back(start);

    while !que.is_empty() {
        let current_pos = que.pop_front().unwrap();
        let x = current_pos.0;
        let y = current_pos.1;

        for d in 0..4 {
            let next_x = x as isize + dx[d];
            let next_y = y as isize + dy[d];
            if next_x < 0 || next_x >= height as isize || next_y < 0 || next_y >= width as isize {
                continue;
            }
            if maze[next_x as usize][next_y as usize] == '#' {
                continue;
            }
            if dist[next_x as usize][next_y as usize] == -1 {
                que.push_back((next_x as usize, next_y as usize));
                dist[next_x as usize][next_y as usize] = dist[x][y] + 1;
                if dist[next_x as usize][next_y as usize] > maximum {
                    maximum = dist[next_x as usize][next_y as usize];
                    goal = (next_x as usize, next_y as usize);
                    //println!("goal: ({}, {}) , length: {}", goal.0, goal.1, maximum);
                }
                //prev_x[next_x as usize][next_y as usize] = x as isize;
                //prev_y[next_x as usize][next_y as usize] = y as isize;
            }
        }
    }

    (goal, maximum)
}

//01bfs abc176 d Wizard in Mazeで使用
//コスト0とコスト1の二種類の選択肢がある時に使う
pub fn bfs01(
    start: (usize, usize),
    goal: (usize, usize),
    maze: &Vec<Vec<char>>,
    dist: &mut Vec<Vec<usize>>,
) -> Option<usize> {
    let height = maze.len();
    let width = maze[0].len();

    let dx = vec![1, 0, -1, 0];
    let dy = vec![0, 1, 0, -1];

    let mut que = VecDeque::new();
    que.push_back(start);
    dist[start.0][start.1] = 0;

    while !que.is_empty() {
        let current_pos = que.pop_front().unwrap();
        let y = current_pos.0 as i32;
        let x = current_pos.1 as i32;
        let d = dist[y as usize][x as usize];

        for i in 0..4 {
            let next_y = y + dy[i];
            let next_x = x + dx[i];
            if next_y < 0 || next_y >= height as i32 || next_x < 0 || next_x >= width as i32 {
                continue;
            }
            if maze[next_y as usize][next_x as usize] == '#' {
                continue;
            }
            if dist[next_y as usize][next_x as usize] <= d {
                continue;
            }
            dist[next_y as usize][next_x as usize] = d;
            que.push_front((next_y as usize, next_x as usize));
        }

        for i in -2..=2 {
            for j in -2..=2 {
                let next_y = y + j;
                let next_x = x + i;
                if next_y < 0 || next_y >= height as i32 || next_x < 0 || next_x >= width as i32 {
                    continue;
                }
                if maze[next_y as usize][next_x as usize] == '#' {
                    continue;
                }
                if dist[next_y as usize][next_x as usize] <= d + 1 {
                    continue;
                }
                dist[next_y as usize][next_x as usize] = d + 1;
                que.push_back((next_y as usize, next_x as usize));
            }
        }
    }

    let ans = dist[goal.0][goal.1];
    if ans == usize::MAX / 2 {
        return None;
    } else {
        return Some(ans);
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    pos: (usize, usize),
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
