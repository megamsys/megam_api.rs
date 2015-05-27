mod Api {

	extern crate curl;
	extern crate rustc_serialize;

	use self::curl::http;
	use self::curl::http::handle::Method::{Post, Put, Get, Delete};
	use self::curl::http::handle::{Method, Request};
	use self::rustc_serialize::json;
	use std::io::prelude::*;
	use std::io::{self, Cursor};
	use std::result;


//	const API_MEGAM_IO: String = "api.megam.io";
	//static API_VERSION2: String = "/v2";

	//const X_Megam_DATE: String = "X-Megam-DATE".to_string();
	//const X_Megam_HMAC: String = "X-Megam-HMAC".to_string();

	pub type Result<T> = result::Result<T, Error>;

	#[derive(PartialEq,Clone, Copy)]
	pub enum Auth {
 	   Authorized,
 	   Unauthorized
	}

	pub enum Error {
	    Curl(curl::ErrCode),
	    NotOkResponse(http::Response),
	    NonUtf8Body,
	    Api(Vec<String>),
	    Unauthorized,
	    TokenMissing,
	    Io(io::Error),
	}

	 struct ApiErrorList { errors: Vec<ApiError> }
	 struct ApiError { detail: String }
   struct R { ok: bool }

	//pub fn create(path: String, data: &[u8]) -> Result<()> {        
	//     let body = try!(req(path, Some(data), Post, Auth::Authorized));
	//     assert!(json::decode::<R>(&body).unwrap().ok);
	//     Ok(())
	//}	

 pub fn create(path: String, data: &[u8]) {        
	     req(path, Some(data), Post, Auth::Authorized)
	    
	}	

	fn req(path: String, body: Option<&[u8]>,
						method: Method, authorized: Auth) {
        //   method: Method, authorized: Auth) -> Result<String> {
        //let mut req = Request::new(&mut http::Handle::new(), method)
				//											.uri(format!("{}/{}{}", "api.megam.io", "/v2", path))
       //                       .header("Accept", "application/json")
			//												.header("X_Megam_DATE", "2015-05-26 13:32:53 +0000")
			//												.header("X_Megam_HMAC", "")
      //                        .content_type("application/json");
        
      //  match body {
       //     Some(b) => req = req.body(b),
       //     None => {}
       // }
      //  handle(req.exec()) 
	 }

	

	//fn handle(response: result::Result<http::Response, curl::ErrCode>)
   //       -> Result<String> {
   // 	let response = try!(response.map_err(Error::Curl));
   //	 match response.get_code() {
   ////     0 => {} // file upload url sometimes
   //     200 => {}
   //    403 => return Err(Error::Unauthorized),
   ////     _ => return Err(Error::NotOkResponse(response))
  // 	 }

  //  let body = match String::from_utf8(response.move_body()) {
  //      Ok(body) => body,
  //      Err(..) => return Err(Error::NonUtf8Body),
   // };
    
   // Ok(body)
	//}
}