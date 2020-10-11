pub fn lis(v: &Vec<usize>, max_n: usize) -> usize {
    let inf = 10usize.pow(10);
    let mut dp = vec![inf; max_n];

    for i in 0..v.len() {
        let index = lower_bound(&dp, v[i]);
        dp[index] = v[i];
    }

    lower_bound(&dp, inf)
}

fn lower_bound<T>(v: &Vec<T>, n: T) -> usize
where
    T: PartialEq + PartialOrd,
{
    let mut left = -1;
    let mut right = v.len() as isize;

    while right - left > 1 {
        let mid = (left + right) / 2;
        if v[mid as usize] < n {
            left = mid;
        } else {
            right = mid;
        }
    }
    right as usize
}
