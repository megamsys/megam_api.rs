use api::Api;
use api::MegError;
use api::MegResponse;
use std::vec::Vec;
use rustc_serialize::json;

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct SSHKey {
   	pub name										: String, 
   	pub accounts_id   					: String, 
   	pub path				  					: String,	
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
        SSHKey { name: "".to_string(), accounts_id: "".to_string(), path: "".to_string() }
			
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
