use std::fmt::Display;

pub(crate) struct Wcnf {
    weights: Vec<i32>,
    clauses: Vec<Vec<i32>>,
    var_num: i32,
    top_weight: i32,
}

impl Display for Wcnf {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "p wcnf {} {} {}",
            self.var_num,
            self.clauses.len(),
            self.top_weight
        )?;
        for i in 0..self.weights.len() {
            let w = self.weights[i];
            let c = &self.clauses[i];
            write!(f, "{} ", w)?;
            for lit in c {
                write!(f, "{} ", lit)?;
            }
            writeln!(f, "0")?;
        }
        Ok(())
    }
}

impl Wcnf {
    pub(crate) fn new(var_num: i32, top_weight: i32) -> Self {
        return Self {
            weights: vec![],
            clauses: vec![],
            var_num: var_num,
            top_weight: top_weight,
        };
    }

    pub(crate) fn add_clause(&mut self, clause: Vec<i32>, weight: Option<i32>) {
        let w = match weight {
            Some(w) => w,
            None => self.top_weight,
        };
        self.clauses.push(clause);
        self.weights.push(w);
    }
}
