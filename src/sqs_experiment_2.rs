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
use std::rc::Rc;

#[tokio::main]
pub async fn start(json: &RequestDetail) -> std::option::Option<std::time::Duration> {
    let json = Rc::new(json);
    let rc_json= Rc::clone(&json);
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
    
let closure_send = || for i in 1..=rc_json.iterate_times {
    let client = client.clone();
    let a = a.clone();
    tokio::spawn(async move {
        let om = async move {
            let res = send(client.clone(), a.clone());
            Ok::<_, Box<dyn std::error::Error + Send + Sync>>(res)
        }.await;
        match om {
            Ok(res) => {
                match res.await {
                    Ok(o) => { 
                        crate::GLOBAL_SUCCESS_COUNT.fetch_max(i as usize, Ordering::SeqCst);
                        println!("{:?} (✅) - {} - Made the request: {} for => {:?}","Ok", common::local_dt(), i, o.message_id.unwrap_or("NA".to_string()));
                    },
                    Err(e) => {
                        crate::GLOBAL_FAILED_COUNT.fetch_max(i as usize, Ordering::SeqCst);
                        println!("{:?} (❎) - {} - Request: {} failed with {:?}", "Error", common::local_dt(), i, e);
                    }
                }
            },
            Err(_) => unreachable!() // This won't happen
        }
    });
};

let closure_receive = || for i in 1..=rc_json.iterate_times {
    let client = client.clone();
    let a = a.clone();
    let b = b.clone();
    tokio::spawn(async move {
        let om = async move {
            let res = receive(client.clone(), b.clone());
            Ok::<_, Box<dyn std::error::Error + Send + Sync>>(res)
        }.await;
        match om {
            Ok(res) => {
                match res.await {
                    Ok(o) => { 
                        crate::GLOBAL_SUCCESS_COUNT.fetch_max(i as usize, Ordering::SeqCst);
                        for k in o.messages.unwrap_or(vec![]) {
                            println!("{:?} (✅) - {} - Received {} for => {:?}","Ok", common::local_dt(), i, k.body.unwrap() );
                        }
                        
                    },
                    Err(e) => {
                        crate::GLOBAL_FAILED_COUNT.fetch_max(i as usize, Ordering::SeqCst);
                        println!("{:?} (❎) - {} - Request: {} failed with {:?}", "Error", common::local_dt(), i, e);
                    }
                }
            },
            Err(_) => unreachable!() // This won't happen
        }
    });
};

match json.aws_details.action.as_str() {
    "receive" => closure_receive(),
    "send" => closure_send(),
    _ => ()
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

trait Omkar {

}

impl Omkar for SendMessageResult {

}

impl Omkar for ReceiveMessageResult {
    
}