/**
 * This file is part of APIRUNNER.
 * APIRUNNER is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or 2 any later version.
 * APIRUNNER is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.
 * You should have received a copy of the GNU General Public License along with APIRUNNER. If not, see <https://www.gnu.org/licenses/>.
**/

use rusoto_sqs::SqsClient as sqs;
use std::time::Instant;
use rusoto_sqs::{SendMessageRequest, SendMessageResult, ReceiveMessageResult, ReceiveMessageRequest};
use rusoto_sqs::Sqs;
use core::time::Duration;
use tokio::time::sleep;
use crate::RequestDetail;
use std::sync::atomic::Ordering;
use crate::common;
use rusoto_core::Region;

#[tokio::main]
pub async fn start(json: &RequestDetail) -> std::option::Option<std::time::Duration> {
    let json = json;
    let rc_json= json.clone();
    let client = sqs::new(Region::Custom { 
        name: rc_json.aws_details.region.to_owned(), 
        endpoint: rc_json.aws_details.url.to_owned() });
    let start_time = Instant::now();

    let a = SendMessageRequest {
     delay_seconds: Some(0),
     message_attributes: None,
     message_system_attributes: None,
     message_body: json.body.to_string(),
     message_deduplication_id: None,
     message_group_id: None,
     queue_url: rc_json.url.to_owned()
    };

    let b =  ReceiveMessageRequest  {
    attribute_names: None,
    max_number_of_messages: Some(rc_json.aws_details.max_receive_messages),
    message_attribute_names: None,
    queue_url: rc_json.url.to_owned(),
    receive_request_attempt_id: None,
    visibility_timeout: None,
    wait_time_seconds: None,
    };
    
for i in 1..=rc_json.iterate_times {
    let client = client.clone();
    let a = a.clone();
    let b = b.clone();
    let json = json.clone();
    let c = Box::new(json.aws_details.action.clone());
    
        if json.aws_details.action.as_str() == "send" {
            tokio::spawn(async move {
            let om = async move {
                let res = send(client.clone(), a.clone());
                Ok::<_, Box<dyn std::error::Error + Send + Sync>>(res)
 
            }.await;
            matcher(c, i, om).await
        });
        } else if json.aws_details.action.as_str() == "receive" {
            let om = async move {
            let res = receive(client.clone(), b.clone());
            Ok::<_, Box<dyn std::error::Error + Send + Sync>>(res)
            }.await;
            matcherVec(c, i, om).await

        } else {
            let om = async move {
                let res = receive(client.clone(), b.clone());
                Ok::<_, Box<dyn std::error::Error + Send + Sync>>(res)
                }.await;
                matcherVec(c, i, om).await
           }
        
};

    let elapsed = Some(start_time.elapsed());
    sleep(Duration::from_secs(json.parallel_request_response_wait_time.try_into().unwrap())).await;

    elapsed

}

pub async fn send(client: sqs, a: SendMessageRequest) -> std::result::Result<SendMessageResult,Box<dyn std::error::Error + Send + Sync>> {
    Ok(client.send_message(a).await?)
}

pub async fn receive(client: sqs, b: ReceiveMessageRequest) -> std::result::Result<ReceiveMessageResult,Box<dyn std::error::Error + Send + Sync>>{
    Ok(client.receive_message(b).await?)
}

async fn matcher<T: OmkarString + std::fmt::Debug>(c: Box<String>, i: u64,
    om: Result<impl std::future::Future<Output = Result<T, Box<dyn std::error::Error + Send + Sync>>>, Box<dyn std::error::Error + Send + Sync>> ) {
    match om {
    Ok(res) => {
        match res.await {
            Ok(o) => { 
                crate::GLOBAL_SUCCESS_COUNT.fetch_max(i as usize, Ordering::SeqCst);
                printer(c, o, i);
            },
            Err(e) => {
                crate::GLOBAL_FAILED_COUNT.fetch_max(i as usize, Ordering::SeqCst);
                println!("{:?} (❎) - {} - Request: {} failed with {:?}", "Error", common::local_dt(), i, e);
            }
        }
    },
    Err(_) => unreachable!() // This won't happen
}
}

async fn matcherVec<T: OmkarVec + std::fmt::Debug>(c: Box<String>, i: u64,
    om: Result<impl std::future::Future<Output = Result<T, Box<dyn std::error::Error + Send + Sync>>>, Box<dyn std::error::Error + Send + Sync>> ) {
    match om {
    Ok(res) => {
        match res.await {
            Ok(o) => { 
                crate::GLOBAL_SUCCESS_COUNT.fetch_max(i as usize, Ordering::SeqCst);
                printer_vec(c, o, i);
            },
            Err(e) => {
                crate::GLOBAL_FAILED_COUNT.fetch_max(i as usize, Ordering::SeqCst);
                println!("{:?} (❎) - {} - Request: {} failed with {:?}", "Error", common::local_dt(), i, e);
            }
        }
    },
    Err(_) => unreachable!() // This won't happen
}
}
use std::fmt::Debug;
fn printer<T: OmkarString>(c: Box<String>, o: T, i : u64) 
where T : Debug  {
    type T = std::string::String;
    if c == Box::new(String::from("send")) {
        println!("{:?} (✅) - {} - Made the request: {} for => {:?}","Ok", common::local_dt(), i, OmkarString::get(&o).unwrap());
    } else {

    };
}

fn printer_vec<T: OmkarVec>(c: Box<String>, o: T, i : u64) 
where T : Debug  {
    type T = std::string::String;
    if c == Box::new(String::from("receive")) {
        let k1 = OmkarVec::get(&o);
    //    for k in k1. {
       println!("{:?} (✅) - {} - Received {} for => {:?}","Ok", common::local_dt(), i, k1 );
   // }
} 
    else {

    };
}
use rusoto_sqs::Message;
trait OmkarString {
    type Item : Debug;
    fn get(&self) -> Option<Self::Item>;
}

impl OmkarString for SendMessageResult {
    type Item = String;
    fn get(&self) -> Option<Self::Item> {
        self.message_id.clone()
    }
}

impl Debug for dyn OmkarString<Item = String> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "Hi")
    }
}

trait OmkarVec {
    type Item : Debug;
    fn get(&self) -> Self::Item;
}

impl OmkarVec for ReceiveMessageResult {
    type Item = Vec<Message>;
    fn get(&self) -> Self::Item {
        self.messages.clone().unwrap_or(vec![])
    }
}

impl Debug for dyn OmkarVec<Item = String> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "Hi")
    }
}