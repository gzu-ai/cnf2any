

use crate::format::{asp::Asp, cnf::CNF};




pub(crate) struct ConvertToAsp{
    pre :String,
    // label: HashMap<String,i64>,
}

impl  ConvertToAsp {
    
    pub(crate) fn new(pre :String)->Self {
        return Self{pre:pre};
    }

    pub(crate) fn to_lit(&self,cnf_val:i64)->i64 {
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
