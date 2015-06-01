use curl;
use curl::http;
use curl::http::handle::Method::{Post, Put, Get, Delete};
use curl::http::handle::{Method, Request};
use rustc_serialize::json;
use std::io::prelude::*;
use std::io::{self, Cursor};
use std::result;
use rustc_serialize::base64::{STANDARD, ToBase64};
use crypto::digest::Digest;
use crypto::md5::Md5;
use crypto::sha1::Sha1;
use crypto::hmac::Hmac;
use crypto::mac::{Mac, MacResult};
use time;
use std::iter::repeat;
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

pub enum SuccessResult { Success }

#[derive(Debug)]
pub enum Error {
			Curl(curl::ErrCode),
	    NotOkResponse,	
			Unauthorized, 
}

struct ApiErrorList { errors: Vec<ApiError> }
struct ApiError { detail: String }
struct R { ok: bool }

pub trait Api {
    // Static method signature; `Self` refers to the implementor type
    fn new(email: &'static str, api_key: &'static str) -> Self;
   
		// Instance methods, only signatures
    fn email(&self) -> &'static str;
    fn api_key(&self) -> &'static str;

   // A trait can provide default method definitions
    fn talk(&self) {
        // These definitions can access other methods declared in the same
        // trait
        println!("{} says {}", self.email(), self.api_key());
    }

     // These definitions can access other methods declared in the same
     // trait
    fn post(&self, path: &'static str, data: &[u8]) {        
     self.req(path, Some(data), Post)
    }	    

    fn get(&self, path: &'static str, data: &[u8]) {			
			self.req(path, None, Get)
		}

		fn req(&self, path: &'static str, body: Option<&[u8]>, method: Method)  {
				//self.encode_header(path);
        println!("=======456456546=======");
				let current_date = time::strftime("%Y-%m-%d %H:%M", &time::now()).unwrap();
        let mut http_handle = http::Handle::new();
        let mut req = Request::new(&mut http_handle, method)
															.uri(format!("{}{}", "http://localhost:9000", path))
                              .header("Accept", "application/json")
															.header("X-Megam-DATE", &current_date)
															.header("X-Megam-HMAC", &self.encode_header(path))
                              .content_type("application/json");
        
        match body {
            Some(b) => req = req.body(b),
            None => {}
        }
			
        
			match handle(req.exec())  {
        Ok(n) => println!("result: Is OK: {:?}", n),
        Err(FailOne) => println!("result: response result Failed One: {:?}", FailOne),
    		}
 		}
    
    fn encode_header(&self, path: &'static str) -> String {
				let input = "";
				let mut digest = Md5::new();
    		digest.input_str(input);
    		println!("{}", digest.result_str());

				let mut out = vec![digest.output_bytes() as u8; digest.output_bytes()];				
				digest.result(&mut out[..]);
				let body_base64 = out.to_base64(STANDARD);
				println!("{}", body_base64);
				
				let current_date = time::strftime("%Y-%m-%d %H:%M", &time::now()).unwrap();
				let data = format!("{}\n{}\n{}", current_date, path, body_base64);
        println!("{}", data);

				let mut out = Hmac::new(Sha1::new(), self.api_key().as_bytes());
				println!("--------------");
				out.input(data.as_bytes());			
        println!("HMAC digest: {}", out.result().code().to_hex());
       // println!("==============");
				let pointer = concat!("a@b.com", ':', "351b911f5ad166184c7d5fb32d6f660eb66dbe29");
        pointer.to_string()
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

	


