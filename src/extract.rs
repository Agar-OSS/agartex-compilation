use std::iter;

use crate::constants::USER_ID_HEADER_NAME;
use axum::headers::{Header, HeaderName, HeaderValue, Error};
pub struct XUserId(pub i32);

impl Header for XUserId {
    fn name() -> &'static HeaderName {
       &USER_ID_HEADER_NAME
    }

    fn decode<'i, I>(values: &mut I) -> Result<Self, Error>
    where
        I: Iterator<Item = &'i HeaderValue>,
    {
        if let Ok(s) = values.next().ok_or_else(Error::invalid)?.to_str() {
            if let Ok(num) = s.parse::<i32>() {
                return Ok(XUserId(num));
            }
        }
        Err(Error::invalid())
    }

    fn encode<E>(&self, values: &mut E)
    where
        E: Extend<HeaderValue>,
    {
        let value = HeaderValue::from(self.0);

        values.extend(iter::once(value));
    }
}
