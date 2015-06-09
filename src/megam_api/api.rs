use curl;
use curl::http;
use curl::http::handle::Method::{Post, Get};
use curl::http::handle::{Method, Request};
use rustc_serialize::json;
use std::result;
use std::{str};
use rustc_serialize::base64::{STANDARD, ToBase64};
use crypto::digest::Digest;
use crypto::md5::Md5;
use crypto::sha1::Sha1;
use crypto::hmac::Hmac;
use crypto::mac::{Mac};
use time;
use rustc_serialize::hex::ToHex;
use std::error::Error;


#[derive(Debug)]
pub struct MegError {
    pub code: u32,
    pub msg: String,
    pub more: String,
}

impl MegError {
    pub fn new(err: MegResponse) -> MegError {
        MegError { code: err.code, msg: err.msg, more: err.more }
    }

    pub fn msg(msg: String) -> MegError {
        MegError { code: 500, msg: msg, more: "".to_string() }
    }
}


#[derive(Debug)]
pub enum CurlError {
		  Curl(curl::ErrCode),
	    NotOkResponse,	
			Unauthorized, 
}

#[derive(RustcEncodable, Debug, RustcDecodable)]
pub struct MegResponse {
 pub code: u32,
 pub msg_type: String,
 pub msg: String,
 pub more: String,
 pub json_claz: String,
 pub links: String,
}

#[derive(PartialEq, Clone, Copy)]
pub enum Auth {
 	   Authorized,
 	   Unauthorized
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
    fn post(&self, path: String, data: &[u8], options: String) -> Result<String, MegError> {        
     self.req(path, Some(data), Post, options)
    }	    

    fn get(&self, path: String, options: String) -> Result<String, MegError> {			
			self.req(path, None, Get, options)
		}

		fn req(&self, path: String, body: Option<&[u8]>, method: Method, options: String) -> Result<String, MegError>  {
       
				let api_options = json::decode::<Options>(&options).unwrap();

				let current_date = time::strftime("%Y-%m-%d %H:%M", &time::now()).unwrap();
        let mut http_handle = http::Handle::new();
        let mut req = Request::new(&mut http_handle, method)
															.uri(format!("{}{}{}", api_options.Host, api_options.Version, path))
                              .header("Accept", "application/json")
															.header("X-Megam-DATE", &current_date)
															.header("X-Megam-HMAC", &self.encode_header(format!("{}{}", api_options.Version, path), api_options.Email, api_options.Apikey, body))
                              .content_type("application/json");
        
        match body {
            Some(b) => req = req.body(b),
            None => {}
        }			
        
		 handle(req.exec())
 		}
    
    fn encode_header(&self, path: String, email: String, api_key: String, body: Option<&[u8]>) -> String {
				let mut input = "";
        match body {
            Some(b) => input = &str::from_utf8(b).unwrap(),
            None => {}
        }		
				let mut digest = Md5::new();
    		digest.input_str(input);

				let mut out = vec![digest.output_bytes() as u8; digest.output_bytes()];				
				digest.result(&mut out[..]);
				let body_base64 = out.to_base64(STANDARD);
				
				let current_date = time::strftime("%Y-%m-%d %H:%M", &time::now()).unwrap();
				let data = format!("{}\n{}\n{}", current_date, path, body_base64);

				let mut out = Hmac::new(Sha1::new(), api_key.as_bytes());
				out.input(data.as_bytes());			
      //  println!("HMAC digest: {}", out.result().code().to_hex());
        format!("{}:{}", email, out.result().code().to_hex()).to_string()
    }
}

fn handle(response: result::Result<http::Response, curl::ErrCode>) -> Result<String, MegError> {
   match response {
    result::Result::Ok(res) => {
   			// println!("{}", res);
    			match res.get_code() {
        		200  => return Ok(str::from_utf8(res.get_body()).unwrap().to_string()),
						201  => return Ok(str::from_utf8(res.get_body()).unwrap().to_string()),
        		403  => return Err(MegError::new(json::decode::<MegResponse>(&str::from_utf8(res.get_body()).unwrap()).unwrap())),
        		_    => return Err(MegError::new(json::decode::<MegResponse>(&str::from_utf8(res.get_body()).unwrap()).unwrap())),
    			}
  	 },
   result::Result::Err(err) => {
       return Err(MegError::msg(err.to_string()));
    },
 	}
	}
   

	


