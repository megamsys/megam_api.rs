pub type MegResult<T> = Result<T, MegError>;

#[derive(Debug)]
pub struct MegError {
    pub code: u32,
    pub msg: String,
    pub more: String,
    pub links: String,
}

#[derive(Debug)]
pub enum Error {
			Curl(curl::ErrCode),
	    NotOkResponse,	
			Unauthorized, 
}

#[derive(RustcEncodable, Debug, RustcDecodable)]
pub struct MegResponse {
 pub code: u32,
 pub msg_type: String,
 pub msg: String,
 pub more: String,
 pub json_claz: String,
 pub links: String,
}

