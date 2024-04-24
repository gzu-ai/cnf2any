use std::{cmp::max, fs, io::{self, Read}};



use nom::{branch::alt, bytes::complete::{tag, take_while}, character::complete::{digit1, multispace0}, combinator::{map_res, opt}, multi::{many0, many_till}, sequence::{pair, terminated}, IResult};

use crate::format::cnf::{Clause, Lit, CNF};
use crate::error::AppError;
enum DIMACSRule {
    Clause(Clause,i64),
    Comment,
    Format,
}


// impl DIMACSRule {
//     fn  is_clause(&self) ->bool {
//         match &self {
//            DIMACSRule::Clause(_,_)=> return  true,
//            _=> return false,
//         }
//     }
// }



fn parse_literal(input: &str) -> IResult<&str, Lit> {
    // 首先尝试匹配负号，如果匹配成功，则将 negative 设置为 true
    // let (input,_)= many0(tag(" "))(input)?;
    let (input, negative) = opt(tag("-"))(input)?;
    let negative = negative.is_some();

    // 然后匹配数字部分
    let (input, digits) = map_res(digit1, |s: &str| s.parse::<i64>())(input)?;

    Ok((input, Lit{neg:negative,val:digits}))
}

fn parse_clause(input: &str) -> IResult<&str,DIMACSRule> {
    let mut c =Clause::new();
    let mut vars=0;
    let (input,_) =multispace0(input)?;

    let (input,(_,_))= many_till(
        terminated(map_res(parse_literal, |lit: Lit|->Result<(), std::num::ParseIntError> {
            vars=max(vars, lit.val);
            c.add_lit(lit);
               return  Ok(());
            }),multispace0)
        , tag("0"))(input)?;
    Ok((input,DIMACSRule::Clause(c,vars)))
}
fn parse_comment(input: &str) -> IResult<&str, DIMACSRule> {
    let (input,_) =multispace0(input)?;
    // 匹配以 'c' 开头的字符后，直到行尾的所有内容
    let (input,_)= pair(tag("c"), take_while(|c: char| c != '\n'))(input)?;
    return Ok((input,DIMACSRule::Comment));
}

fn parse_format(input: &str) -> IResult<&str, DIMACSRule> {
    // 匹配以 'c' 开头的字符后，直到行尾的所有内容
    let (input,_) =multispace0(input)?;
    let (input,_)= pair(tag("p"), take_while(|c: char| c != '\n'))(input)?;
    return Ok((input,DIMACSRule::Format));
}

fn parse_dimacs_cnf(input: &str) -> IResult<&str, CNF> {
    let mut cnf= CNF::new();
    let (input, clauses) = many0(alt((parse_clause, parse_comment, parse_format)))(input)?;
    // clauses.into_iter().filter(|res| res.is_clause()).collect()
    clauses.into_iter().for_each(|res|{
        match res {
            DIMACSRule::Clause(c,v)=> {
                cnf.var_num=max(cnf.var_num, v);
                cnf.add_clause(c);
            },
            _=> {},
         }
    });
    Ok((input,cnf))
}

pub(crate) fn read_dimacs_from_file(path: &str) -> Result<CNF,AppError> {
    let data= if path=="-" {
        let mut buf = String::new();
        let _=io::stdin().read_to_string(&mut buf);
        buf
    }else{
        fs::read_to_string(path).map_err(|e|AppError::new(format!("File open failure: {}",e)))?
    };
    let (input,cnf)=parse_dimacs_cnf(&data).map_err(|e|AppError::new(format!("DIMACS read failure: {}",e)))?;
    if input.len() > 0 {
        return Err(AppError::new(format!("DIMACS read failure: {}",input)));
    }
    return Ok(cnf);
}