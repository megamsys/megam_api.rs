
mod Accounts {

	static CREATE: String = "/accounts/content"

	pub struct Account {
  	first_name							: String,
  	last_name								: String,
  	phone										: String,
  	email										: String, 
  	api_key									: String, 
  	password								: String, 
  	authority								: String, 
  	password_reset_key			: String,
  	password_reset_sent_at	: String,   
	}

	pub fn create(data: Accounts) -> MegamResult<()> {
					// you can access struct values using self.first_name
			let body = json::encode(data).unwrap();
			self.api.create(CREATE, body.as_bytes())
	}
   

}