pub struct Rob;

impl Rob {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let l = nums.len();
        if l == 0 {
            return 0;
        }
        if l == 1 {
            return nums[0];
        }
        let (mut i, mut a, mut b) = (2usize, nums[0], nums[0].max(nums[1]));
        while i < l {
            let tmp = b;
            b = b.max(a + nums[i]);
            a = tmp;
            i += 1;
        }
        return b;
    }
}

#[cfg(test)]
mod test {
    use super::Rob;
    #[test]
    fn rob_test() {
        assert_eq!(Rob::rob(vec![1, 2, 3]), 4)
    }
}
