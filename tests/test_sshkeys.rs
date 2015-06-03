use megam_api::api::Api;
use megam_api::api::Options;
use megam_api::util::sshkeys::SSHKey;
use rustc_serialize::json;

#[test]
fn list() {
   // let mut p = SSHKey{name: format!("{}", "rr"),	accounts_id: format!("{}", ""), path: format!("{}", "")};
		let options = Options {
    Email: "b@a.com".to_string(),
    Apikey: "GGUdgT-sjLXn9EYRkS8Q7g==".to_string(),
    Host: "http://localhost:9000".to_string(),
    Version: "/v2".to_string(),
    };

    println!("{:?}", json::encode(&options).unwrap());

    let a = SSHKey::new();
 
		println!("{:?}", a);
   
    match a.list(json::encode(&options).unwrap()) {
        Ok(n) => println!("result: Is OK: {:?}", n),
        Err(FailOne) => println!("result: Failed One: {:?}", FailOne),
    }
}
