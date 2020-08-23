pub struct Solution;
impl Solution {
    pub fn judge_point24(nums: Vec<i32>) -> bool {
        Self::help(nums)
    }
    pub fn a(nums: Vec<i32>) -> Vec<i32> {
        let mut v = vec![
            nums[0] + nums[1],
            nums[0] - nums[1],
            nums[1] - nums[0],
            nums[0] * nums[1],
        ];
        if nums[0] != 0 {
            v.push((nums[1] as f64 / nums[0] as f64).round() as i32)
        }
        if nums[1] != 0 {
            v.push((nums[0] as f64 / nums[1] as f64).round() as i32)
        }
        v
    }
    pub fn help(nums: Vec<i32>) -> bool {
        if nums.len() == 2 {
            Self::a(nums).contains(&24)
        } else {
            for i in 0..nums.len() {
                for j in 0..nums.len() {
                    if i != j {
                        for n in Self::a(vec![nums[i], nums[j]]) {
                            let mut copy = nums.clone();
                            copy.remove(j.max(i));
                            copy.remove(i.min(j));
                            copy.push(n);
                            if Self::help(copy) {
                                return true;
                            }
                        }
                    }
                }
            }
            false
        }
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_knapsack() {
        assert_eq!(Solution::judge_point24(vec![2, 1, 3, 4]), true);
    }
}
