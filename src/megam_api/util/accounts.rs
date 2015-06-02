use api::Api;
use api::Success;
use api::Error;
use rustc_serialize::json;

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

	pub fn create(&self, options: String) -> Result<String, Error> {
		println!("Account create Entry...");					
		println!("Account --------- {:?}", self);
    
    // you can access struct values using self.first_name
		let body = json::encode(&self).unwrap();
		self.post("/accounts/content".to_string(), body.as_bytes(), options)		
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
    fn new() -> Account {       
        Account { first_name: "", last_name: "", phone: "", email: "", api_key: "", password: "", authority: "", password_reset_key: "", password_reset_sent_at: "" }
    }

}
