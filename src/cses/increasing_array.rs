pub fn count_min_moves(nums: &mut [i64]) -> i64 {
    if nums.len() < 2 {
        0
    } else {
        let mut count = 0;
        for i in 1..nums.len() {
            if nums[i] < nums[i - 1] {
                count += nums[i - 1] - nums[i];
                nums[i] = nums[i - 1];
            }
        }
        count
    }
}
