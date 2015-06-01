use megam_api::api::Api;
use megam_api::util::accounts::Account;
use megam_api::util::accounts::Success;
use hamcrest::assert_that;

#[test]
fn create() {
    let mut a: Account = Api::new("a@b.com", "firsttest");
    
		println!("{:?}", a.email);
		println!("{:?}", a.api_key);   
		println!("{:?}", a);
   
    //assign the values for struct Account
    a.first_name = "raj";
    a.phone = "97948698";

     match a.create() {
        Ok(n) => trace!("result: Is OK: {:?}", n),
        Err(FailOne) => trace!("result: Failed One"),
    }
}
