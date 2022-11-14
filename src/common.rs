/**
 * This file is part of APIRUNNER.
 * APIRUNNER is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or 2 any later version.
 * APIRUNNER is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.
 * You should have received a copy of the GNU General Public License along with APIRUNNER. If not, see <https://www.gnu.org/licenses/>.
**/
use crate::RequestDetail;
use crate::reqwest::header as header;
use std::str::FromStr;
//use crate::reqwest::header::{*};
use serde_json::value::Value;

// This function takes the Request Details, gets the headers associated with it, loop through each one of them
// and into HeaderMap. This Result is passed to the .headersm method declared in the call_api function.
pub fn header_builder(rd: &RequestDetail) -> header::HeaderMap {
    
    let mut map = header::HeaderMap::new();
    for i in rd.headers.as_object() {
        for ele in i {
            map.insert(header::HeaderName::from_str(ele.0).unwrap(), header::HeaderValue::from_str(match ele.1 {
                Value::String(o) => o,
                _ => "null"
            }).unwrap());
        }

    }
    //map.insert(HOST, header::HeaderValue::from_str(option_unwrapper_to_str(&rd.Accept)).unwrap());
    //map.insert(CONTENT_LENGTH, header::HeaderValue::from_str(option_unwrapper_to_str(&rd.Content_Type)).unwrap());
    //map.insert(ACCEPT_CHARSET, header::HeaderValue::from_str(option_unwrapper_to_str(&rd.Accept_Charset)).unwrap());

    map
}

use std::io::{Write, Read, self};
use chrono::{Local, format};

// Gives you local date time
pub fn local_dt() -> format::DelayedFormat<format::StrftimeItems<'static>> {
    let date = Local::now();
    date.format("%Y-%m-%dT%H:%M:%S")
}

pub fn pause() {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    // We want the cursor to stay at the end of the line, so we print without a newline and flush manually.
    write!(stdout, "\nPress enter to continue... or Ctrl + C to break!!!").unwrap();
    stdout.flush().unwrap();

    // Read a single byte and discard
    let _ = stdin.read(&mut [0u8]).unwrap();
}
/* 
pub mod awssign {
    use rusoto_signature::{region,signature};
    use crate::RequestDetail;
    use std::str::FromStr;
    use bytes::Bytes;
    use std::collections::BTreeMap;
    use crate::reqwest::header as header;
    use serde_json::value::Value;

    struct SignGenerate {
        region: region::Region,
        payload: signature::SignedRequestPayload
    }

    pub fn generate(rd: &RequestDetail) {

        let a = SignGenerate { 
            region: region::Region::from_str(rd.aws.region.as_str()).unwrap(), 
            payload: signature::SignedRequestPayload::Buffer(Bytes::from(rd.body.to_string()))
        };

        let b = signature::SignedRequest {
                                method: rd.method,
                                service: rd.aws.service,
                                region: region::Region::from_str(rd.aws.region.as_str()).unwrap(),
                                path: rd.aws.path,
                                headers: header_builder(rd),
                                params: Params,
                                scheme: Option<String>,
                                hostname: Option<String>,
                                payload: Option<SignedRequestPayload>,
                                canonical_query_string: String,
                                canonical_uri: String,
        };

    }

    pub fn header_builder(rd: &RequestDetail) -> BTreeMap<String, Vec<Vec<u8>>> {
    let mut btreeheaders = BTreeMap::new();

        for i in rd.headers.as_object() {
            for ele in i {
                btreeheaders.insert(header::HeaderName::from_str(ele.0).unwrap(), header::HeaderValue::from_str(match ele.1 {
                    Value::String(o) => o,
                    _ => "null"
                }).unwrap());
            }
    
        }
        btreeheaders
    }

    pub fn sign() {
        let a = signature::SignedRequest::sign(&mut self, creds);
    }

}
*/
