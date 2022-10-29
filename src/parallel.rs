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
use crate::reqwest::header as header;
use reqwest::Method;

// This is the function which makes the actual api calls. It takes the New Reqwest Object as arg0, Request Details as arg1
// It also waits for the API response, but it passes the () to the Request
// Instead of adding one header at a time, its better to add bunch of headers using .headers method of Client
pub async fn call_api(client: reqwest::Client, rd: RequestDetail, hd: header::HeaderMap) -> std::result::Result<String,Box<dyn std::error::Error + Send + Sync>> {
    
      let v = client
        .request(Method::from_str(&rd.method).unwrap(), &rd.url)
        .body(rd.body.to_string())
        .headers(hd)
        //.header("Accept-Charset", rd.Accept_Charset.clone().unwrap_or("None".to_string())) //These work too!
        //.header("Content-Type", rd.Content_Type.clone().unwrap_or("None".to_string()))
        //.header("Accept-Encoding", rd.Accept_Encoding.clone().unwrap_or("None".to_string()))
        .send().await?.text().await?;
        Ok::<_, Box<dyn std::error::Error + Send + Sync>>(v)
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
use std::sync::atomic::Ordering;
use crate::common;
use std::time::Instant;
use core::time::Duration;
use tokio::time::sleep;
use crate::common::header_builder;

#[tokio::main]
pub async fn start(json: &RequestDetail, client: &reqwest::Client) -> Option<std::time::Duration> {
     let headers = header_builder(&json);
     let start_time = Instant::now();

     for i in 1..=json.iterate_times{
        let client = client.clone();
        let json = json.clone();
        let headers = headers.clone();
        tokio::spawn(async move {
             let om = async move {
                 let res = call_api(client, json, headers.clone());
                 Ok::<_, Box<dyn std::error::Error + Send + Sync>>(res)
             }.await;

            match om {
                Ok(res) => {
                    match res.await {
                        Ok(o) => { 
                            crate::GLOBAL_SUCCESS_COUNT.fetch_max(i as usize, Ordering::SeqCst);
                            println!("{:?} (✅) - {} - Made the request: {} for => {:?}","Ok", common::local_dt(), i, o);
                        },
                        Err(e) => {
                            crate::GLOBAL_FAILED_COUNT.fetch_max(i as usize, Ordering::SeqCst);
                            println!("{:?} (❎) - {} - Request: {} failed with {:?}", "Error", common::local_dt(), i, e);
                        }
                    }
                },
                Err(_) => {} // This won't happen
            }
        });
     }
     
     let elapsed = Some(start_time.elapsed());
     sleep(Duration::from_secs(json.parallel_request_response_wait_time.try_into().unwrap())).await;
     elapsed

}