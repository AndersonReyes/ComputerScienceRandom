/// https://algs4.cs.princeton.edu/15uf/
/// Union Find algorithms interface
pub trait UF {
    /// constructor
    fn new(n: u32) -> Self;
    /// add connection between p and q
    fn union(&mut self, p: u32, q: u32);
    /// find component of p
    fn find(&mut self, p: u32) -> u32;
    /// is p connencted to q?
    fn connected(&mut self, p: u32, q: u32) -> bool;
    /// number of components
    fn count(&self) -> u32;
}
