use api::Api;
use rustc_serialize::json;

#[derive(Debug)]
pub enum Success { Success }

#[derive(Debug)]
pub enum Error {
	    NotOkResponse,	    
}

#[derive(Debug, RustcEncodable)]
pub struct SSHKey {
   	pub name										: &'static str, 
   	pub accounts_id   					: &'static str, 
   	pub path				  					: &'static str, 
		pub email										: &'static str, 
  	pub api_key									: &'static str, 
}

impl SSHKey {

	pub fn create(&self) -> Result<Success, Error> {
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

 pub fn list(&self) -> Result<Success, Error> {
  	let LIST = "/v2/sshkeys";
		println!("SSHKey list Entry...");					
		println!("SSHKey --------- {:?}", self);
    
		// you can access struct values using self.first_name
		let body = json::encode(&self).unwrap();
		self.get(LIST, body.as_bytes());
    if 1 == 1 {
   		return Ok(Success::Success);
  	} else {
   		return Err(Error::NotOkResponse);
  	}
 }

}   

// Implement the `Api` trait for `Account`
impl Api for SSHKey {
    // Replace `Self` with the implementor type: `Account`
    fn new(email: &'static str, api_key: &'static str) -> SSHKey {
        SSHKey { name: "", accounts_id: "", path: "", email: email, api_key: api_key }
    }

    fn email(&self) -> &'static str {
        self.email
    }

    fn api_key(&self) -> &'static str {
        self.api_key
    }   
}

