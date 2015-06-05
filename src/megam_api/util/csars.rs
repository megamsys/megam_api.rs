use api::Api;
use api::MegError;
use std::vec::Vec;
use rustc_serialize::json;

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct Csar {
	pub id   					: String, 
	pub desc					: String, 
	pub link					: String, 
	pub created_at		: String,
}

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct CsarResponse {
	results   : Vec<Csar>,
	json_claz : String,
}

impl Csar {

  //
  //create a new Csar 
  //must provide email: String,
	//						 api_key: String,
	//						 host: String,
	//						 version: String,  //optional
	//
	pub fn create(&self, options: String) -> Result<String, MegError> {
	//	println!("Csar create Entry...");		    
		self.post("/csars/content".to_string(), self.desc.as_bytes(), options)		
	} 

  pub fn list(&self, options: String) -> Result<Vec<Csar>, MegError> {
  // println!("Csar list ==>");
   parser(self.get("/csars".to_string(), options))  
  }

  pub fn push(&self, options: String, id: String) -> Result<String, MegError> {
 	//	println!("Csar launch process started...");
		self.get(format!("{}{}", "/csars/push/", id), options)
 }
  
}

impl Api for Csar {
 
 fn new() -> Csar {
   Csar { 
					id					: "".to_string(),
					desc				: "".to_string(),
					link				: "".to_string(),
					created_at	: "".to_string(),
				}
	}
}

fn parser(result: Result<String, MegError>) -> Result<Vec<Csar>, MegError> {
   match result {
        Ok(n) => {
						let res = json::decode::<CsarResponse>(&n).unwrap();
						return Ok(res.results)
				},
        Err(FailOne) => return Err(FailOne),
    }

}