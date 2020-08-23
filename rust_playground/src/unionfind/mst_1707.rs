use super::union_find::UnionFind;
pub struct Solution;
impl Solution {
    pub fn find_circle_num(m: Vec<Vec<i32>>) -> i32 {
        let mut u = UnionFind::with_capacity(m.len());
        for i in 0..m.len() {
            for j in 0..i {
                if m[i][j] == 1 {
                    u.union(i, j);
                }
            }
        }
        u.count() as i32
    }
}
