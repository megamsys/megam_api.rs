use Accounts;
use Accounts::Account;
use hamcrest::assert_that;
use turbo;

fn setup() {}

test!(simple {
    let p = Account{first_name: "rr",	last_name: "", phone: "", email: "b@test.com", api_key: "testapikey", password: "testpassword",	authority: "", password_reset_key: "", password_reset_sent_at: ""};

    assert_that(create(p), execs().with_status(0));


});
