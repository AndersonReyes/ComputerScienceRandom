use super::term::Term;

pub struct Autocomplete<'a> {
    terms: &'a Vec<Term>,
}

impl<'a> Autocomplete<'a> {
    fn new(terms: &'a Vec<Term>) -> Self {
        Autocomplete { terms }
    }

    pub fn all_matches(&self, prefix: &str) -> Vec<Term> {
        let mut matches = self
            .terms
            .iter()
            .filter(|t| t.query.starts_with(prefix))
            // map out of references by cloning
            .map(|t| t.clone())
            .collect::<Vec<Term>>();
        matches.sort_by(Term::by_reserve_weight_order());
        matches
    }

    pub fn number_of_matches(&self, prefix: &str) -> usize {
        self.terms
            .iter()
            .filter(|t| t.query.starts_with(prefix))
            .collect::<Vec<&Term>>()
            .len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let terms = vec![Term::new("the", 1), Term::new("of", 3)];

        let auto = Autocomplete::new(&terms);

        assert_eq!(&terms, auto.terms);
    }

    #[test]
    fn all_matches() {
        let terms = vec![
            Term::new("automobile", 1),
            Term::new("automatic", 3),
            Term::new("company", 1),
        ];
        let auto = Autocomplete::new(&terms);

        let result = auto.all_matches("auto");

        assert_eq!(result[0], terms[1]);
        assert_eq!(result[1], terms[0]);
    }

    #[test]
    fn number_of_matches() {
        let terms = vec![
            Term::new("automobile", 1),
            Term::new("automatic", 3),
            Term::new("company", 1),
        ];
        let auto = Autocomplete::new(&terms);

        let result = auto.number_of_matches("auto");

        assert_eq!(result, 2);
    }
}
