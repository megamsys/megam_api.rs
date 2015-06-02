use api::Api;
use api::MegError;
use api::MegResponse;
use rustc_serialize::json;

#[derive(PartialEq, Clone, Debug, RustcEncodable, RustcDecodable)]
pub struct Account {
  	pub first_name							: String,
  	pub last_name								: String,
  	pub phone										: String,
  	pub email										: String, 
  	pub api_key									: String, 
  	pub password								: String, 
  	pub authority								: String, 
  	pub password_reset_key			: String,
  	pub password_reset_sent_at	: String,   
}

impl Account {

	pub fn create(&self, options: String) -> Result<String, MegError> {
		println!("Account create Entry...");					
		println!("Account --------- {:?}", self);
    
    // you can access struct values using self.first_name
		let body = json::encode(&self).unwrap();
		self.post("/accounts/content".to_string(), body.as_bytes(), options)		
	} 

}

// Implement the `Api` trait for `Account`
impl Api for Account {
    // Replace `Self` with the implementor type: `Account`
    fn new() -> Account {       
        Account { first_name: "".to_string(), last_name: "".to_string(), phone: "".to_string(), email: "".to_string(), api_key: "".to_string(), password: "".to_string(), authority: "".to_string(), password_reset_key: "".to_string(), password_reset_sent_at: "".to_string() }
    }

}
