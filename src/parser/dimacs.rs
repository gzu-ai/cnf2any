use std::{cmp::max, fs, io::{self, Read}};





use crate::format::cnf::{Clause, Lit, CNF};
use crate::error::AppError;
use  pest::Parser;
#[derive(pest_derive::Parser)]
#[grammar = "../pest/dimacs.pest"]
struct DIMACSParser;



fn parse_dimacs_cnf(input: &str) -> Result<CNF,AppError> {
    let mut cnf= CNF::new();
    let pairs= DIMACSParser::parse(Rule::file,input).map_err(|e|AppError::new(format!("DIMACS read failure: {}",e)))?;
    for pair in pairs {
        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::cluase =>{
                    let mut clause= Clause::new();
                    for lit_pair in inner_pair.into_inner(){
                        let lit = lit_pair.as_str().parse::<i32>().map_err(|e|AppError::new(format!("DIMACS read failure: {}",e)))?;
                        let abs=lit.abs();
                        cnf.var_num=max(abs, cnf.var_num);
                        let lit=Lit{neg:lit<0,val:abs};
                        clause.add_lit(lit);
                    }
                    cnf.add_clause(clause);
                },
                _ => {}
            };
        }
    }
    Ok(cnf)
}




pub(crate) fn read_dimacs_from_file(path: &str) -> Result<CNF,AppError> {
    let data= if path=="-" {
        let mut buf = String::new();
        let _=io::stdin().read_to_string(&mut buf);
        buf
    }else{
        fs::read_to_string(path).map_err(|e|AppError::new(format!("File open failure: {}",e)))?
    };
    let cnf=parse_dimacs_cnf(&data)?;

    return Ok(cnf);
}