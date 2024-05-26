

use crate::format::{asp::Asp, cnf::CNF, wcnf::Wcnf};




pub(crate) struct ConvertToAsp{
    pre :String,
    // label: HashMap<String,i32>,
}

impl  ConvertToAsp {
    
    pub(crate) fn new(pre :String)->Self {
        return Self{pre};
    }

    pub(crate) fn to_lit(&self,cnf_val:i32)->i32 {
        return  cnf_val+1;
    }



    pub(crate) fn to_asp(&self,cnf:&CNF) -> Asp{
        let mut asp=Asp::new();
        for v in 1..cnf.var_num+1 {
            asp.add_label(v+1,format!("{}{}",&self.pre,v));
        }

        for clause in &cnf.clauses{
            let head =clause.head.iter().map(|lit|self.to_lit(lit.val)).collect();
            let pos_body=clause.body.iter().map(|lit| self.to_lit(lit.val)).collect();
            asp.add_rule(head, pos_body, vec![]);
        }

        return  asp;
    }
}



pub(crate) struct ConvertToWcnf{

}

impl  ConvertToWcnf {
    
    pub(crate) fn new()->Self {
        return Self{};
    }

    pub(crate) fn to_wcnf(&self,cnf:&CNF,m:&Vec<i32>) -> Wcnf{
        let mut wcnf=Wcnf::new(cnf.var_num,i32::MAX);

        for clause in &cnf.clauses{
            let mut c=vec![];
            clause.head.iter().for_each(|lit|c.push(lit.val));//.collect();
            clause.body.iter().for_each(|lit|c.push(-lit.val));
            wcnf.add_clause(c, None);
        }

        for atom in m{
            wcnf.add_clause(vec![-atom], Some(1));
        }
        return  wcnf;
    }
}
