use super::UF;

#[derive(Debug)]
/// Weighted union find with path halving.
/// https://algs4.cs.princeton.edu/15uf/QuickUnionPathSplittingUF.java.html
pub struct WeightedWithHalvingUF {
    sites: Vec<u32>,
    components: u32,
    sizes: Vec<u32>,
}

impl UF for WeightedWithHalvingUF {
    fn new(n: u32) -> Self {
        WeightedWithHalvingUF {
            sites: (0..n).collect(),
            components: n,
            sizes: vec![1; n as usize],
        }
    }

    fn union(&mut self, p: u32, q: u32) {
        let p_root = self.find(p);
        let q_root = self.find(q);

        if q_root == p_root {
            return;
        }

        if self.sizes[p_root as usize] >= self.sizes[q_root as usize] {
            self.sizes[p_root as usize] += self.sizes[q_root as usize];
            self.sizes[q_root as usize] = 0;
            self.sites[q_root as usize] = p_root;
        } else {
            self.sizes[q_root as usize] += self.sizes[p_root as usize];
            self.sizes[p_root as usize] = 0;
            self.sites[p_root as usize] = q_root;
        }
        self.components -= 1;
    }

    fn find(&mut self, p: u32) -> u32 {
        let mut parent = self.sites[p as usize];
        while parent != self.sites[parent as usize] {
            self.sites[parent as usize] = self.sites[self.sites[parent as usize] as usize];
            parent = self.sites[parent as usize];
        }
        parent
    }

    fn connected(&mut self, p: u32, q: u32) -> bool {
        self.find(p) == self.find(q)
    }

    fn count(&self) -> u32 {
        self.components
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count() {
        let mut uf = WeightedWithHalvingUF::new(2);
        assert_eq!(uf.count(), 2);
        uf.components = 10;
        assert_eq!(uf.count(), 10);
    }

    #[test]
    fn new() {
        let uf = WeightedWithHalvingUF::new(2);
        assert_eq!(uf.sizes, vec![1, 1]);
        assert_eq!(uf.count(), 2);
        assert_eq!(uf.sites.len(), 2);
        assert_eq!(uf.sizes.len(), 2);
        assert_eq!(uf.sites[0], 0);
        assert_eq!(uf.sites[1], 1);
    }

    #[test]
    fn connected() {
        let mut uf = WeightedWithHalvingUF::new(3);
        for i in 0..3 {
            assert_eq!(uf.connected(i, i), true);
        }

        assert_eq!(uf.sites, vec![0, 1, 2]);
        assert_eq!(uf.connected(0, 2), false);
        assert_eq!(uf.connected(2, 0), false);
        assert_eq!(uf.connected(1, 2), false);
        assert_eq!(uf.connected(2, 1), false);
    }

    #[test]
    fn union() {
        let mut uf = WeightedWithHalvingUF::new(3);
        uf.union(1, 0);

        assert_eq!(uf.sizes, vec![0, 2, 1]);
        assert_eq!(uf.sites, vec![1, 1, 2]);

        uf.union(0, 2);
        assert_eq!(uf.sizes, vec![0, 3, 0]);
        assert_eq!(uf.sites, vec![1, 1, 1])
    }

    #[test]
    fn union_with_weights() {
        let mut uf = WeightedWithHalvingUF::new(5);
        uf.union(0, 1);
        uf.union(2, 3);
        uf.union(2, 4); // should join to 2 since component 2 is bigger

        assert_eq!(uf.sites, vec![0, 0, 2, 2, 2]);
    }

    #[test]
    fn find() {
        let mut uf = WeightedWithHalvingUF::new(7);
        uf.sites = vec![1, 2, 3, 4, 5, 6, 6];

        // calling find updates roots to grandparents
        assert_eq!(uf.find(0), 6);
        assert_eq!(uf.sites, vec![1, 3, 3, 5, 5, 6, 6]);
        uf.find(0);
        assert_eq!(uf.sites, vec![1, 5, 3, 5, 5, 6, 6]);
    }
}
