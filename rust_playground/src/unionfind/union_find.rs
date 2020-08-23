pub struct UnionFind {
    parents: Vec<usize>,
    size: Vec<usize>,
    count: usize,
}

impl UnionFind {
    pub fn with_capacity(n: usize) -> Self {
        let mut v = vec![0; n];
        for i in 0..n {
            v[i] = i;
        }
        UnionFind {
            parents: v,
            size: vec![1; n],
            count: n,
        }
    }

    pub fn find(&mut self, mut p: usize) -> usize {
        while self.parents[p] != p {
            self.parents[p] = self.parents[self.parents[p]];
            p = self.parents[p];
        }
        p
    }

    pub fn connected(&mut self, p: usize, q: usize) -> bool {
        self.find(p) == self.find(q)
    }

    pub fn union(&mut self, p: usize, q: usize) {
        let i = self.find(p);
        let j = self.find(q);
        if i != j {
            self.count -= 1;
            if self.size[i] > self.size[j] {
                self.parents[j] = i;
                self.size[i] += self.size[j];
            } else {
                self.parents[i] = j;
                self.size[j] += self.size[i];
            }
        }
    }

    pub fn count(&self) -> usize {
        self.count
    }
}

#[cfg(test)]
mod test {
    use super::UnionFind;
    use std::collections::HashMap;
    use std::ops::Add;

    #[test]
    fn test_union_find() {
        let mut uf = UnionFind::with_capacity(10);
        uf.union(1, 3);
        uf.union(3, 9);
        uf.union(6, 7);
        uf.union(6, 5);
        uf.union(7, 1);
        assert_eq!(uf.connected(5, 9), true);
        assert_eq!(uf.count(), 5);
    }
}
