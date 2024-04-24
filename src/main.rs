
use clap::{Parser, Subcommand};
use error::AppError;
use parser::dimacs::read_dimacs_from_file;

use crate::convert::ConvertToAsp;

pub(crate) mod format;
pub(crate) mod parser;

pub(crate) mod  error;

pub(crate) mod convert;

/// A tools for converting CNF file to other formats
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}



#[derive(Subcommand, Debug)]
enum Commands {
    /// Command for converting CNF to dlp (circumscription).
    Circ {
        /// Prefix for atoms when converting CNF to ASP.
        #[arg( default_value = "p",long,short)]
        pre: String,
        /// Fixed atoms that are to be eliminated during conversion (split by ' ' ).
        #[arg(long,short,value_delimiter = ' ')]
        fixed: Vec<i64>,
         /// The file path to the CNF file; use "-" for stdin.
        #[arg( default_value = "-")]
        path: String,
    },
}



fn run()->Result<(),AppError>{
    let args = Args::parse();
    match args.command {
        Commands::Circ { pre, fixed, path }=>{
            let cnf= read_dimacs_from_file(&path)?;
            let convert=  ConvertToAsp::new(pre);
            let mut asp = convert.to_asp(&cnf);
            let mut i =cnf.var_num+1;
            for v in fixed {
                let f=convert.to_lit(v);
                let not_f=convert.to_lit(i);
                asp.add_rule(vec![f,not_f], vec![], vec![]);
                asp.add_rule(vec![], vec![f,not_f], vec![]);
                i=i+1;
            }
            println!("{}",asp);
        }
    }

    Ok(())
}




fn main() {
    let result =run();
    match result {
        Err(e)=>{println!("{}",e)},
        Ok(_)=>{},
    }

}