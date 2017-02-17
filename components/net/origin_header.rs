/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use hyper::Error as HyperError;
use hyper::Result as HyperResult;
use hyper::header::{ Header, HeaderFormat };
use hyper::header::parsing::from_one_raw_str;
use servo_url::ServoUrl;
use std::fmt;
use std::str::FromStr;
use url::Origin as UrlOrigin;

#[derive(Clone,Debug,Eq,PartialEq)]
pub struct OriginHeader(pub UrlOrigin);

impl Header for OriginHeader {
    fn header_name() -> &'static str {
        "Origin"
    }

    fn parse_header(raw: &[Vec<u8>]) -> HyperResult<OriginHeader> {
        from_one_raw_str(raw)
    }
}

impl HeaderFormat for OriginHeader {
    fn fmt_header(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0.ascii_serialization())
    }
}

impl FromStr for OriginHeader {
    type Err = HyperError;

    fn from_str(input: &str) -> Result<OriginHeader, HyperError> {
        if input == "null" {
            Ok(OriginHeader(UrlOrigin::new_opaque()))
        } else {
            Ok(OriginHeader(try!(ServoUrl::parse(input)).origin()))
        }
    }
}
