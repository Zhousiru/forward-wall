use cookie::{time::Duration, Cookie, SameSite};

pub fn build_cookie(passcode: String) -> String {
  Cookie::build(("forward_wall_passcode", passcode))
    .path("/")
    .same_site(SameSite::Lax)
    .secure(true)
    .max_age(Duration::days(365))
    .http_only(true)
    .to_string()
}
