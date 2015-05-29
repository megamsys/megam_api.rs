use megam_api::util::sshkeys::SSHKey;
use megam_api::util::sshkeys::Success;
use hamcrest::assert_that;

#[test]
fn create() {
    let mut p = SSHKey{name: format!("{}", "rr"),	accounts_id: format!("{}", ""), path: format!("{}", "")};
		//p.create();
   // assert_that(p.create(), execs().with_status(0));
    //assert_that(p.create(), Ok(Success::Success));
     //assert_that(p.create(), is(Ok(Success::Success)));
     //assert!(p.create().is_err());
}
