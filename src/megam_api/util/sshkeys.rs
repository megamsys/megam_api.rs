use api::Api;
use api::Success;
use api::Error;

#[derive(Debug, RustcEncodable)]
pub struct SSHKey {
   	pub name										: &'static str, 
   	pub accounts_id   					: &'static str, 
   	pub path				  					: &'static str,	
}

impl SSHKey {	

 pub fn list(&self, options: String) -> Result<String, Error> {
		println!("SSHKey list Entry...");					
		println!("SSHKey --------- {:?}", self);    
		
		self.get("/sshkeys".to_string(), options)    
 }

}   

// Implement the `Api` trait for `Account`
impl Api for SSHKey {
    // Replace `Self` with the implementor type: `Account`
    fn new() -> SSHKey {
        SSHKey { name: "", accounts_id: "", path: "" }
			
    }
   
}

