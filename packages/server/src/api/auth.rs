use std::collections::BTreeMap;

use hmac::Hmac;
use jwt::SignWithKey;
use poem::web::cookie::{Cookie, CookieJar, SameSite};
use poem_openapi::{payload::PlainText, OpenApi};
use sha2::Sha256;

pub struct Api {
    pub jwt_key: Hmac<Sha256>,
}

#[OpenApi]
impl Api {
    /// Authenticate a user
    #[oai(path = "/auth/login", method = "post")]
    async fn login(&self, cookies: &CookieJar) -> PlainText<String> {
        let mut claims = BTreeMap::<&'static str, String>::new();
        claims.insert("sub", "markus.gasser@mailbox.org".to_string());

        let token = match claims.sign_with_key(&self.jwt_key) {
            Ok(token) => token,
            Err(err) => {
                tracing::error!("Could not sign JWT: {err}");
                return PlainText("Error".to_string());
            }
        };

        let sig_separator = token.rfind('.').expect("Token should have a signature");
        let claims_part = &token[0..sig_separator];
        let sig_part = &token[sig_separator + 1..];

        let mut claims_cookie = Cookie::new_with_str("estimator_auth.claims", claims_part);
        claims_cookie.set_http_only(false);
        claims_cookie.set_same_site(SameSite::Strict);

        let mut sig_cookie = Cookie::new_with_str("estimator_auth.sig", sig_part);
        sig_cookie.set_http_only(true);
        sig_cookie.set_same_site(SameSite::Strict);

        cookies.add(claims_cookie);
        cookies.add(sig_cookie);

        PlainText(token)
    }
}
