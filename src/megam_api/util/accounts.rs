use api::Api;
use api::Options;
use api::MegError;
use rustc_serialize::json;

#[derive(PartialEq, Clone, Debug, RustcEncodable, RustcDecodable)]
pub struct Account {
		pub id											: String,
  	pub first_name							: String,
  	pub last_name								: String,
  	pub phone										: String,
  	pub email										: String, 
  	pub api_key									: String, 
  	pub password								: String, 
  	pub authority								: String, 
  	pub password_reset_key			: String,
  	pub password_reset_sent_at	: String,   
		pub created_at							: String,
    pub json_claz								: String,

}

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct AccountShowResponse {
	results   : Account,
	json_claz : String,
}

impl Account {

  //
  //create a new account 
  //must provide email: String,
	//						 api_key: String,
	//						 host: String,
	//						 version: String,  //optional
	//
	pub fn create(&self, options: String) -> Result<String, MegError> {
		println!("Account create Entry...");					
		println!("Account --------- {:?}", self);
    
    // you can access struct values using self.first_name
		let body = json::encode(&self).unwrap();
		self.post("/accounts/content".to_string(), body.as_bytes(), options)		
	} 
  
  //show the particular account details 
  pub fn show(&self, options: String) -> Result<Account, MegError> {
    println!("Account show procees started...");
		let api_options = json::decode::<Options>(&options).unwrap();
    parser(self.get(format!("{}{}", "/accounts/", api_options.Email), options)) 
  } 

}

// Implement the `Api` trait for `Account`
impl Api for Account {
    // Replace `Self` with the implementor type: `Account`
    fn new() -> Account {       
        Account { 
									id											: "".to_string(), 
									first_name							: "".to_string(), 
									last_name								: "".to_string(),
									phone           				: "".to_string(), 
									email										: "".to_string(), 
									api_key									: "".to_string(), 
									password								: "".to_string(), 
									authority								: "".to_string(), 
									password_reset_key			: "".to_string(), 
									password_reset_sent_at	: "".to_string(), 
									created_at							: "".to_string(), 
									json_claz								: "".to_string(), 
								}
    }

}

fn parser(result: Result<String, MegError>) -> Result<Account, MegError> {
   match result {
        Ok(n) => {
						return Ok(json::decode::<Account>(&n).unwrap())
				},
        Err(FailOne) => return Err(FailOne),
    }
}
