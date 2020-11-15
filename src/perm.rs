//https://gist.github.com/horyu/566a5155db897a47b5d893e9307e1f93
pub fn make_permutation(n: usize) -> Vec<Vec<usize>> {
    let mut vecs: Vec<Vec<usize>> = vec![Vec::new(); factorial(n)];
    let nums: Vec<usize> = (0..n).collect();
    let indexes: Vec<usize> = (0..factorial(n)).collect();
    push_recusive(nums, indexes, &mut vecs);
    vecs
}

fn push_recusive<T: Clone>(
    nums: Vec<T>,
    indexes: Vec<usize>,
    vecs: &mut Vec<Vec<T>>,
) -> &mut Vec<Vec<T>> {
    if nums.len() == 0 {
        return vecs;
    }
    let block_size = factorial(nums.len() - 1);
    for (block_index, num) in nums.iter().enumerate() {
        for inner_index in 0..block_size {
            let index = indexes[block_size * block_index + inner_index];
            vecs[index].push(num.clone());
        }
        let new_nums = {
            let mut tmp = nums.clone();
            tmp.remove(block_index);
            tmp
        };
        let new_indexes: Vec<usize> = {
            let slice = &indexes[(block_size * block_index)..(block_size * (block_index + 1))];
            slice.to_vec()
        };
        push_recusive(new_nums, new_indexes, vecs);
    }
    vecs
}

fn factorial(i: usize) -> usize {
    if i <= 1 {
        1
    } else {
        (2..=i).fold(1, |acc, x| acc * x)
    }
}
