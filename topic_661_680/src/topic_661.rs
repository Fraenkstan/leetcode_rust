pub fn image_smoother(nums: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let (m, n) = (nums.len(), nums[0].len());
    let mut ret = vec![vec![0; n]; m];

    for i in 0..m {
        for j in 0..n {
            let mut count = 0;

            for &round_i in &[i - 1, i, i + 1] {
                for &round_j in &[j - 1, j, j + 1] {
                    if round_i == usize::MAX
                        || round_i == m
                        || round_j == usize::MAX
                        || round_j == n
                    {
                        continue;
                    }

                    ret[i][j] += nums[round_i][round_j];
                    count += 1;
                }
            }

            ret[i][j] /= count;
        }
    }

    ret
}
