use super::unionfind::WeightedWithHalvingUF;
use crate::unionfind::UF;

fn convert_2d_to_index(n: u32, i: u32, j: u32) -> u32 {
    assert!(i < n && j < n, "({}, {})", i, j);
    // should map (i, j) to index in range [1, n]. 0 and n+1 are dummy sites
    (i * n) + j + 1
}

pub struct Percolation {
    uf: WeightedWithHalvingUF,
    grid: Vec<bool>,
    n: u32,
    virtual_top: u32,
    virtual_bottom: u32,
}

impl Percolation {
    fn new(n: u32) -> Self {
        let mut uf = WeightedWithHalvingUF::new(n * n + 2); // Pseudo start site + pseudo end site

        for i in 0..(n) {
            // union top row to 0 dummy site
            uf.union(0, convert_2d_to_index(n, 0, i));
            // union bottom row to n+1 dummy site
            uf.union(n * n + 1, convert_2d_to_index(n, n - 1, i));
        }
        let mut grid = vec![false; (n * n + 2) as usize];
        grid[0] = true;
        grid[(n * n + 1) as usize] = true;

        Percolation {
            uf,
            grid,
            n,
            virtual_top: 0,
            virtual_bottom: n * n + 1,
        }
    }

    /// Is site open at i,j
    pub fn is_open(&self, i: u32, j: u32) -> bool {
        assert!(i < self.n && j < self.n);
        let index = convert_2d_to_index(self.n, i, j);
        self.grid[index as usize]
    }

    /// is the site at i, j connected to it's surrounding sites?
    pub fn is_full(&mut self, i: u32, j: u32) -> bool {
        assert!(i < self.n && j < self.n, "({}, {})", i, j);
        let index = convert_2d_to_index(self.n, i, j);
        self.is_open(i, j) && self.uf.connected(index, self.virtual_top)
    }

    pub fn percolates(&mut self) -> bool {
        self.uf.connected(self.virtual_top, self.virtual_bottom)
    }

    pub fn open(&mut self, i: u32, j: u32) {
        assert!(i < self.n && j < self.n, "({}, {})", i, j);
        let index = convert_2d_to_index(self.n, i, j);
        self.grid[index as usize] = true;

        // union neighbors if they open
        // UP
        if i > 0 && self.is_open(i - 1, j) {
            let up = convert_2d_to_index(self.n, i - 1, j);
            self.uf.union(index, up);
        }
        // DOWN
        if i < self.n - 1 && self.is_open(i + 1, j) {
            let down = convert_2d_to_index(self.n, i + 1, j);
            self.uf.union(index, down);
        }

        // LEFT
        if j > 0 && self.is_open(i, j - 1) {
            let left = convert_2d_to_index(self.n, i, j - 1);
            self.uf.union(index, left);
        }

        // RIGHT
        if j < self.n - 1 && self.is_open(i, j + 1) {
            let right = convert_2d_to_index(self.n, i, j + 1);
            self.uf.union(index, right);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn small() {
        let mut perc = Percolation::new(3);
        perc.open(0, 1);
        perc.open(1, 1);
        assert_eq!(perc.percolates(), false);
    }

    #[test]
    fn percolate() {
        let mut perc = Percolation::new(2);
        assert_eq!(perc.percolates(), false);
        perc.open(0, 0);
        assert_eq!(perc.percolates(), false);
        perc.open(1, 0);
        assert_eq!(perc.percolates(), true);
    }

    #[test]
    fn is_full() {
        let mut perc = Percolation::new(2);
        assert_eq!(perc.is_full(1, 0), false);

        perc.grid[1] = true;
        perc.grid[3] = true;
        // 1, 0 CENTER site
        perc.uf.union(perc.virtual_top, 3);
        // 0, 0 UP
        perc.uf.union(perc.virtual_top, 1);

        assert_eq!(perc.is_full(1, 0), true);

        perc.grid[3] = false;
        assert_eq!(perc.is_full(1, 0), false);
    }

    #[test]
    fn is_open() {
        let mut uf = Percolation::new(2);
        uf.grid[1] = false;
        assert_eq!(uf.is_open(0, 0), false);

        uf.grid[1] = true;
        assert_eq!(uf.is_open(0, 0), true);
    }

    #[test]
    fn test_convert_2d_to_index() {
        assert_eq!(convert_2d_to_index(2, 1, 1), 4);
    }

    #[test]
    fn open() {
        let mut uf = Percolation::new(2);
        assert_eq!(uf.grid[1], false);
        uf.open(0, 0);
        assert_eq!(uf.grid[1], true);
    }
}
