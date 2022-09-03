use std::cmp::{min, Ordering};

#[derive(Debug, Eq, Clone)]
pub struct Term {
    pub query: String,
    pub weight: u32,
}

impl Term {
    pub fn new(q: &str, weight: u32) -> Self {
        Term {
            query: q.to_string(),
            weight,
        }
    }

    /// Compares the two terms in descending order by weight.
    pub fn by_reserve_weight_order() -> impl FnMut(&Term, &Term) -> Ordering {
        |a, b| b.weight.cmp(&a.weight)
    }

    /// Compares the two terms in lexicographic order,
    /// but using only the first r characters of each query.
    pub fn by_prefix_order(r: usize) -> impl FnMut(&Term, &Term) -> Ordering {
        move |a, b| {
            let m = min(r, min(a.query.len(), b.query.len()));

            let ord = (&a.query[..m]).cmp(&b.query[..m]);

            // If m is different than r and the substrings are
            // equal check which string is smaller overall
            if ord == Ordering::Equal && m < r {
                return if a.query.len() < b.query.len() {
                    Ordering::Less
                } else {
                    Ordering::Greater
                };
            }

            ord
        }
    }
}

impl Ord for Term {
    fn cmp(&self, other: &Self) -> Ordering {
        self.query.cmp(&other.query)
    }
}

impl PartialOrd for Term {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Term {
    fn eq(&self, other: &Self) -> bool {
        self.query == other.query
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ord() {
        let a = Term::new("aato", 2);
        let b = Term::new("abr", 5);
        let c = Term::new("ab", 5);

        assert_eq!(a.cmp(&b), Ordering::Less);
        assert_eq!(b.cmp(&a), Ordering::Greater);
        assert_eq!(b.cmp(&c), Ordering::Greater);
        assert_eq!(c.cmp(&b), Ordering::Less);
    }

    #[test]
    fn by_prefix_order() {
        let a = Term::new("aato", 2);
        let b = Term::new("abr", 5);
        let c = Term::new("ab", 5);

        let mut comparator = Term::by_prefix_order(2);
        assert_eq!(comparator(&a, &b), Ordering::Less);
        assert_eq!(comparator(&b, &a), Ordering::Greater);
        assert_ne!(comparator(&a, &b), Ordering::Equal);

        let mut comparator2 = Term::by_prefix_order(3);
        assert_eq!(comparator2(&b, &c), Ordering::Greater);
        assert_eq!(comparator2(&c, &b), Ordering::Less);
        assert_ne!(comparator2(&b, &c), Ordering::Equal);
    }

    #[test]
    fn by_reserve_weight_order() {
        let a = Term::new("auto", 2);
        let b = Term::new("car", 5);

        let mut comparator = Term::by_reserve_weight_order();
        assert_eq!(comparator(&a, &b), Ordering::Greater);
        assert_eq!(comparator(&b, &a), Ordering::Less);
        assert_ne!(comparator(&a, &b), Ordering::Equal);
    }
}
