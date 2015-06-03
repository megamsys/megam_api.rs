use api::Api;
use api::MegError;
use std::vec::Vec;
use rustc_serialize::json;

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct SSHKey {
		pub id											: String,
   	pub name										: String, 
   	pub accounts_id   					: String, 
   	pub path				  					: String,	
		pub created_at							: String,
    pub json_claz								: String,
}

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct SSHKeyResponse {
	results   : Vec<SSHKey>,
	json_claz : String,
}

impl SSHKey {	

 pub fn list(&self, options: String) -> Result<Vec<SSHKey>, MegError> {
		println!("SSHKey list Entry...");					
		println!("SSHKey --------- {:?}", self);    
		
		parser(self.get("/sshkeys".to_string(), options))    
 }

}   

// Implement the `Api` trait for `Account`
impl Api for SSHKey {
    // Replace `Self` with the implementor type: `Account`
    fn new() -> SSHKey {
        SSHKey { 
									id								: "".to_string(), 
									name							: "".to_string(), 
									accounts_id				: "".to_string(), 
									path							: "".to_string(),
									created_at				: "".to_string(), 
									json_claz					: "".to_string(), 
								}			
    }   
}

fn parser(result: Result<String, MegError>) -> Result<Vec<SSHKey>, MegError> {
   match result {
        Ok(n) => {
						let res = json::decode::<SSHKeyResponse>(&n).unwrap();
						return Ok(res.results)
				},
        Err(FailOne) => return Err(FailOne),
    }

}
