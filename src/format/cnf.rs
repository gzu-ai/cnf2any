use std::fmt::Display;

pub(crate) struct CNF {
   pub var_num: i32,
   pub clauses: Vec<Clause>,
}

pub(crate) struct Clause {
    pub(crate) body: Vec<Lit>,
    pub(crate) head: Vec<Lit>,
}

pub(crate) struct Lit {
    pub(crate) val: i32,
    pub(crate) neg: bool,
}

impl Display for Lit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.neg {
            write!(f, "-")?;
        }
        write!(f,"{}",self.val)
    }
}

impl From<Lit> for i32 {
    fn from(value: Lit) -> Self {
        if value.neg {
            -value.val
        }else {
            value.val
        }
    }
}

impl Display for Clause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for h in &self.head {
            write!(f, "{} ", h)?;
        }
        for h in &self.body {
            write!(f, "{} ", h)?;
        }
        write!(f, "0")
    }
}

impl Display for CNF {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "p cnf {} {}\n", self.var_num, self.clauses.len())?;

        for c in &self.clauses  {
            writeln!(f, "{}",c)?;
        }

        Ok(())
    }
}

impl CNF {
    pub(crate) fn new()->Self {
        return  Self{
            var_num:0,
            clauses:vec![]
        };
    }

    pub(crate) fn add_clause(&mut self,c :Clause) {
        self.clauses.push(c);
    }
}


impl Clause  {
    pub(crate) fn new()->Self{
        return Clause{body:vec![],head:vec![]};
    }

    // pub(crate) fn add_head(&mut self,atom:Lit) {
    //     self.head.push(atom)
    // }
    // pub(crate) fn add_body(&mut self,atom:Lit) {
    //     self.body.push(atom)
    // }

    pub(crate) fn add_lit(&mut self,lit:Lit) {
       if lit.neg {
           self.body.push(lit)
       }else {
           self.head.push(lit)
       }
    }

}
