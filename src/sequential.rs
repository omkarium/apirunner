/**
 * This file is part of APIRUNNER.
 * APIRUNNER is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or 2 any later version.
 * APIRUNNER is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.
 * You should have received a copy of the GNU General Public License along with APIRUNNER. If not, see <https://www.gnu.org/licenses/>.
**/

/**
 * The agenda of this `sequential.rs` file is to make sequential calls to the target API
**/

use std::str::FromStr;
use crate::RequestDetail;
use crate::Error;

// This is the function which makes the actual api calls. It takes the New Reqwest Object as arg0, Request Details as arg1
// It also waits for the API response, but it passes the () to the Request
// Instead of adding one header at a time, its better to add bunch of headers using .headers method of Client
pub async fn call_api(client: &reqwest::Client, rd: &RequestDetail) -> Result<(), Box<dyn Error>> {
    
    client
       .get(&rd.url)
       .headers(header_builder(&rd))
       //.header("Accept-Charset", rd.Accept_Charset.clone().unwrap_or("None".to_string())) //These work too!
       //.header("Content-Type", rd.Content_Type.clone().unwrap_or("None".to_string()))
       //.header("Accept-Encoding", rd.Accept_Encoding.clone().unwrap_or("None".to_string()))
       .send()
       .await?.text().await?;

       Ok(())
}

use crate::reqwest::header as header;
//use crate::reqwest::header::{*};
use serde_json::value::Value;

// This function takes the Request Details, gets the headers associated with it, loop through each one of them
// and into HeaderMap. This Result is passed to the .headersm method declared in the call_api function.
fn header_builder(rd: &RequestDetail) -> header::HeaderMap {

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

/* This works too in relation to the above commented lines
fn option_unwrapper_to_str(arg: &Option<String>) -> &str{
    let null = "null";
    match arg {
        Some(o) => o.as_str(),
        None => null
    }
}
*/