pub struct Stock;

impl Stock {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let (len, mut i, mut m, mut rm) = (prices.len(), (prices.len() - 1) as i32, 0i32, 0i32);
        if len > 1 {
            while i >= 0 {
                let iu = i as usize;
                m = m.max(0).max(rm - prices[iu]);
                rm = rm.max(prices[iu]);
                i -= 1;
            }
        }
        m
    }
}

#[cfg(test)]
mod test {

    use super::Stock;

    #[test]
    fn stockproblem_test() {
        assert_eq!(Stock::max_profit(vec![1, 2, 3]), 2)
    }
}
