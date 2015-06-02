use megam_api::api::Api;
use megam_api::util::accounts::Account;
use rustc_serialize::json;
use megam_api::api::Options;

#[test]
fn create() {
    // create hashmap for api settings
    let options = Options {
    Email: "c@b.com".to_string(),
    Apikey: "firsttest".to_string(),
    Host: "http://localhost:9000".to_string(),
    Version: "/v2".to_string(),
    };

     println!("{:?}", json::encode(&options).unwrap());

    let mut a = Account::new();		
   
    //assign the values for struct Account
    a.first_name = "raj".to_string();
    a.phone = "97948698".to_string();
		a.email = "c@b.com".to_string();
		a.api_key = "firsttest".to_string();

     match a.create(json::encode(&options).unwrap()) {
        Ok(n) => println!("result: Is OK: {:?}", n),
        Err(FailOne) => println!("result: Failed One: {:?}", FailOne),
    }
}
