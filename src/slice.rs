use std::collections::VecDeque;

/// supersliceのupperboundはVecDequeにたいして使えなかったので実装した。
pub fn upper_bound<T>(v: &VecDeque<T>, n: T) -> usize
where
    T: PartialEq + PartialOrd,
{
    let mut left = -1;
    let mut right = v.len() as isize;

    while right - left > 1 {
        let mid = (left + right) / 2;
        if v[mid as usize] <= n {
            left = mid;
        } else {
            right = mid;
        }
    }
    right as usize
}

///Returns the index i pointing to the first element in the ordered slice that is not less than x.
pub fn lower_bound<T>(v: &VecDeque<T>, n: T) -> usize
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
