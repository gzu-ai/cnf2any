use std::fmt::Display;


#[derive(Debug)]
pub(crate) struct  AppError{
    msg : String,
}


impl Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}",self.msg)
    }
}

impl AppError {
    pub(crate) fn new(msg:String)->Self{
        return  Self{msg:msg};
    }
}


