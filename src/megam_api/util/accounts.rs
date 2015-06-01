use api::Api;
use rustc_serialize::json;

#[derive(Debug)]
pub enum Success { Success }

#[derive(Debug)]
pub enum Error {
	    NotOkResponse,	    
}

#[derive(PartialEq, Clone, Debug, RustcEncodable)]
pub struct Account {
  	pub first_name							: &'static str,
  	pub last_name								: &'static str,
  	pub phone										: &'static str,
  	pub email										: &'static str, 
  	pub api_key									: &'static str, 
  	pub password								: &'static str, 
  	pub authority								: &'static str, 
  	pub password_reset_key			: &'static str,
  	pub password_reset_sent_at	: &'static str,   
}

impl Account {

	pub fn create(&self) -> Result<Success, Error> {
    let CREATE = "/accounts/content";
		println!("Account create Entry...");					
		println!("Account --------- {:?}", self);
 		self.talk();
    
    // you can access struct values using self.first_name
		let body = json::encode(&self).unwrap();
		self.post(CREATE, body.as_bytes());

		if 1 == 1 {
   		return Ok(Success::Success);
  	} else {
   		return Err(Error::NotOkResponse);
  	}
	}

  pub fn failure(&self) -> Result<Success, Error> {
   	if 1 == 1 {
   		return Err(Error::NotOkResponse);
  	} else {
   		return Ok(Success::Success);
  	}
	}

}

// Implement the `Api` trait for `Account`
impl Api for Account {
    // Replace `Self` with the implementor type: `Account`
    fn new(email: &'static str, api_key: &'static str) -> Account {
        Account { first_name: "", last_name: "", phone: "", email: email, api_key: api_key, password: "", authority: "", password_reset_key: "", password_reset_sent_at: "" }
    }

    fn email(&self) -> &'static str {
        self.email
    }

    fn api_key(&self) -> &'static str {
        self.api_key
    }   
}
