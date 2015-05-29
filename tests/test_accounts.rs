use megam_api::util::accounts::Account;
use megam_api::util::accounts::Success;
use hamcrest::assert_that;

#[test]
fn create() {
    let mut p = Account{first_name: format!("{}", "rr"),	last_name: format!("{}", ""), phone: format!("{}", ""), email: format!("{}", "b@test.com"), api_key: format!("{}", "testapikey"), password: format!("{}", "testpassword"),	authority: format!("{}", ""), password_reset_key: format!("{}", ""), password_reset_sent_at: format!("{}", "")};
		//p.create();
   // assert_that(p.create(), execs().with_status(0));
    //assert_that(p.create(), Ok(Success::Success));
     //assert_that(p.create(), is(Ok(Success::Success)));
     //assert!(p.create().is_err());
}
