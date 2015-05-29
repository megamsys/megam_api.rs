//use std::result;
//use rustc_serialize::json;

//pub type Result<Success, Error> = result::Result<Success, Error>;

#[derive(Debug)]
pub enum Success { Success }

#[derive(Debug)]
pub enum Error {
	    NotOkResponse,	    
}

pub mod accounts;
pub mod sshkeys;