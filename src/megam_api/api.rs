use curl;
use curl::http;
use curl::http::handle::Method::{Post, Get};
use curl::http::handle::{Method, Request};
use rustc_serialize::json;
use std::result;
use rustc_serialize::base64::{STANDARD, ToBase64};
use crypto::digest::Digest;
use crypto::md5::Md5;
use crypto::sha1::Sha1;
use crypto::hmac::Hmac;
use crypto::mac::{Mac};
use time;
use rustc_serialize::hex::ToHex;

//	const API_MEGAM_IO: String = "api.megam.io";
	//static API_VERSION2: String = "/v2";

	//const X_Megam_DATE: String = "X-Megam-DATE".to_string();
	//const X_Megam_HMAC: String = "X-Megam-HMAC".to_string();

//pub type Result<T> = result::Result<T, Error>;

#[derive(PartialEq, Clone, Copy)]
pub enum Auth {
 	   Authorized,
 	   Unauthorized
}

#[derive(Debug)]
pub enum Success { Success }

#[derive(Debug)]
pub enum Error {
			Curl(curl::ErrCode),
	    NotOkResponse,	
			Unauthorized, 
}

#[derive(RustcEncodable, Debug, RustcDecodable)]
pub struct Options {
  pub Email: String,
	pub Apikey: String,
	pub Host: String,
	pub Version: String,
}

pub trait Api {
    // Static method signature; `Self` refers to the implementor type
    fn new() -> Self;
   
	   // These definitions can access other methods declared in the same
     // trait
    fn post(&self, path: String, data: &[u8], options: String) -> Result<String, Error> {        
     self.req(path, Some(data), Post, options)
    }	    

    fn get(&self, path: String, options: String) -> Result<String, Error> {			
			self.req(path, None, Get, options)
		}

		fn req(&self, path: String, body: Option<&[u8]>, method: Method, options: String) -> Result<String, Error>  {
       
				let api_options = json::decode::<Options>(&options).unwrap();

				let current_date = time::strftime("%Y-%m-%d %H:%M", &time::now()).unwrap();
        let mut http_handle = http::Handle::new();
        let mut req = Request::new(&mut http_handle, method)
															.uri(format!("{}{}{}", api_options.Host, api_options.Version, path))
                              .header("Accept", "application/json")
															.header("X-Megam-DATE", &current_date)
															.header("X-Megam-HMAC", &self.encode_header(format!("{}{}", api_options.Version, path), api_options.Email, api_options.Apikey))
                              .content_type("application/json");
        
        match body {
            Some(b) => req = req.body(b),
            None => {}
        }			
        
		 handle(req.exec())
 		}
    
    fn encode_header(&self, path: String, email: String, api_key: String) -> String {
				let input = "";
				let mut digest = Md5::new();
    		digest.input_str(input);

				let mut out = vec![digest.output_bytes() as u8; digest.output_bytes()];				
				digest.result(&mut out[..]);
				let body_base64 = out.to_base64(STANDARD);
				
				let current_date = time::strftime("%Y-%m-%d %H:%M", &time::now()).unwrap();
				let data = format!("{}\n{}\n{}", current_date, path, body_base64);

				let mut out = Hmac::new(Sha1::new(), api_key.as_bytes());
				out.input(data.as_bytes());			
        println!("HMAC digest: {}", out.result().code().to_hex());
        format!("{}:{}", email, out.result().code().to_hex()).to_string()
    }

}

fn handle(response: result::Result<http::Response, curl::ErrCode>) -> Result<String, Error> {
    let response = try!(response.map_err(Error::Curl));
    println!("{}", response);
    match response.get_code() {
        0 => {} // file upload url sometimes
        200  => {}
				201  => {}
        403 => return Err(Error::Unauthorized),
        _ => return Err(Error::NotOkResponse)
    }

    Ok(format!("{}", "hai"))
}

	


