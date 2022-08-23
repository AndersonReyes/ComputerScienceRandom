use super::UF;

#[derive(Debug)]
pub struct QuickFindUF {
    sites: Vec<u32>,
    components: u32,
}

impl UF for QuickFindUF {
    fn new(n: u32) -> Self {
        QuickFindUF {
            sites: (0..n).collect(),
            components: n,
        }
    }

    fn union(&mut self, p: u32, q: u32) {
        let p_root = self.find(p);
        let q_root = self.find(q);
        self.sites[p_root as usize] = q_root;
        self.components -= 1;
    }

    fn find(&mut self, p: u32) -> u32 {
        let mut parent = self.sites[p as usize];
        while parent != self.sites[parent as usize] {
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
        let mut uf = QuickFindUF::new(2);
        assert_eq!(uf.count(), 2);
        uf.union(0, 1);
        assert_eq!(uf.count(), 1);
    }

    #[test]
    fn new() {
        let uf = QuickFindUF::new(2);
        assert_eq!(uf.count(), 2);
    }

    #[test]
    fn connected() {
        let mut uf = QuickFindUF::new(3);
        for i in 0..3 {
            assert_eq!(uf.connected(i, i), true);
        }
        assert_eq!(uf.connected(0, 2), false);
        assert_eq!(uf.connected(2, 0), false);
        assert_eq!(uf.connected(1, 2), false);
        assert_eq!(uf.connected(2, 1), false);
    }

    #[test]
    fn union() {
        let mut uf = QuickFindUF::new(3);
        uf.union(0, 1);

        assert_eq!(uf.connected(0, 1), true);
        assert_eq!(uf.connected(0, 2), false);
        assert_eq!(uf.connected(1, 2), false);
    }
}
