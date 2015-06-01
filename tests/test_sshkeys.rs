use megam_api::api::Api;
use megam_api::util::sshkeys::SSHKey;
use megam_api::util::sshkeys::Success;
use hamcrest::assert_that;

#[test]
fn list() {
   // let mut p = SSHKey{name: format!("{}", "rr"),	accounts_id: format!("{}", ""), path: format!("{}", "")};
		let mut a: SSHKey = Api::new("a@b.com", "firsttest");
    
		println!("{:?}", a.email);
		println!("{:?}", a.api_key);   
		println!("{:?}", a);
   
    match a.list() {
        Ok(n) => trace!("result: Is OK: {:?}", n),
        Err(FailOne) => trace!("result: Failed One"),
    }
}
