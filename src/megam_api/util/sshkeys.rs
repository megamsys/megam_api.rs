use std::result;
//use rustc_serialize::json;

pub type Result<Success, Error> = result::Result<Success, Error>;

#[derive(Debug)]
pub enum Success { Success }

#[derive(Debug)]
pub enum Error {
	    NotOkResponse,	    
}

pub struct SSHKey {
   	pub name					: String,
   	pub accounts_id   : String,
   	pub path				  : String,
}

impl SSHKey {

	pub fn create(self) -> Result<Success, Error> {
      //let CREATE = "/accounts/content";
		println!("format {} arguments", "hai");
					// you can access struct values using self.first_name
			//let body = json::encode(data).unwrap();
		//	self.megam_api.create(CREATE, body.as_bytes())
		if 1 == 1 {
   		return Ok(Success::Success);
  	} else {
   		return Err(Error::NotOkResponse);
  	}
	}

  pub fn failure() -> Result<Success, Error> {
   	if 1 == 1 {
   		return Err(Error::NotOkResponse);
  	} else {
   		return Ok(Success::Success);
  	}
	}

}   

