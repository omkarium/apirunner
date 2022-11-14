/**
 * This file is part of APIRUNNER.
 * APIRUNNER is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or 2 any later version.
 * APIRUNNER is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.
 * You should have received a copy of the GNU General Public License along with APIRUNNER. If not, see <https://www.gnu.org/licenses/>.
**/

/**
 * The agenda of this `sqs.rs` file is to make parallel calls to the SQS API
**/

use std::time::Instant;
use rusoto_sqs::{
    SendMessageRequest, SendMessageResult, ReceiveMessageResult, ReceiveMessageRequest, Message, SqsClient as sqs, Sqs
};
use core::time::Duration;
use tokio::time::sleep;
use crate::{
    RequestDetail, common
};
use std::sync::atomic::Ordering;
use rusoto_core::Region;
use std::rc::Rc;

#[tokio::main]
pub async fn start(json: &RequestDetail) -> std::option::Option<std::time::Duration> {
    let json = Rc::new(json);
    let rc_json= Rc::clone(&json);

    // Create a client for SQS which is used to make the actual SQS requests
    let client = sqs::new(Region::Custom { 
        name: rc_json.aws_details.region.to_owned(), 
        endpoint: rc_json.aws_details.url.to_owned() });

    // Start time of our main implementation
    let start_time = Instant::now();

    // Create an instance of SendMessageRequest which will be used to send our json data
    let a = SendMessageRequest {
        delay_seconds: Some(0),
        message_attributes: None,
        message_system_attributes: None,
        message_body: json.body.to_string(),
        message_deduplication_id: None,
        message_group_id: None,
        queue_url: rc_json.url.to_owned()
    };

    // Create an instance of ReceiveMessageRequest which will be used to receive our data from SQS
    let b = ReceiveMessageRequest  {
        attribute_names: None,
        max_number_of_messages: Some(rc_json.aws_details.max_receive_messages),
        message_attribute_names: None,
        queue_url: rc_json.url.to_owned(),
        receive_request_attempt_id: None,
        visibility_timeout: None,
        wait_time_seconds: None,
    };
    
for i in 1..=rc_json.iterate_times {
    // We need to clone these below variables because of the async move in the tokio spawn.
    let client = client.clone();
    let a = a.clone();
    let b = b.clone();
    let json = json.clone();
    let c = json.aws_details.action.clone();

    tokio::spawn(async move {
        let c = c.as_str(); // This var `c` is a the action we choose to call. Send, Receive etc. This is used by the printer() function too

        let om = async move {
            let res = match c {
                // This match case arms returns two different types. Which is why I had to create a common trait named `Omkar`
                // Data now resides in a trait Object
                "receive" => receive(client.clone(), b.clone()).await,
                "send" => send(client.clone(), a.clone()).await,
                _ => dummy().await // A dummy Result implementation for honouring the match arms
            };

            Ok::<_, Box<dyn std::error::Error + Send + Sync>>(res)
        };

        // Once the sqs main request operations are completed in the async block, we wait for the Result
        match om.await {
            Ok(res) => {
                match res {
                    Ok(o) => { 
                        //Increment the success counter
                        crate::GLOBAL_SUCCESS_COUNT.fetch_max(i as usize, Ordering::SeqCst);
                        // Call Our dynamic printer function.
                        printer(c, i, o);
                    },
                    Err(e) => {
                        // Increment the failure counter
                        crate::GLOBAL_FAILED_COUNT.fetch_max(i as usize, Ordering::SeqCst);
                        println!("{:?} (❎) - {} - Request: {} failed with {:?}", "Error", common::local_dt(), i, e);
                    }
                }
            },
            Err(_) => unreachable!() // This won't happen
        }
    });
};

    // Time when all the thread calling is complete. But threads are not completed.
    let elapsed = Some(start_time.elapsed());

    // Sleep until all the threads are complete. This is a guess work. You need to pass a wait time based on estimates.
    // The program will pause here until the wait time is finished, even if all the threads are complete.
    sleep(Duration::from_secs(json.parallel_request_response_wait_time.try_into().unwrap())).await;

    elapsed

}

// To make sqs send requests by Passing the client and our Message. Returns a Result of trait object.
pub async fn send(client: sqs, a: SendMessageRequest) -> std::result::Result<Box<dyn Omkar>,Box<dyn std::error::Error + Send + Sync>> {
    Ok(Box::new(client.send_message(a).await?))
}

// To make sqs receive requests by Passing the client and our Message. Returns a Result of trait object.
pub async fn receive(client: sqs, b: ReceiveMessageRequest) -> std::result::Result<Box<dyn Omkar>,Box<dyn std::error::Error + Send + Sync>>{
    Ok(Box::new(client.receive_message(b).await?))
}

// A dummy function for match case defaults
pub async fn dummy() -> std::result::Result<Box<dyn Omkar>,Box<dyn std::error::Error + Send + Sync>>{
    Ok(Box::new(DummyMessageResult {}))
}

// This trait is used to handle Results of different types based on the action which ran.
pub trait Omkar {
    fn get(&self) -> Option<Vec<Message>> { //Provided with a default implementation
    Some(vec![Message {
        attributes: None,
        body: None,
        md5_of_body: None,
        md5_of_message_attributes: None,
        message_attributes: None,
        message_id: None,
        receipt_handle: None
    }])
    }
}

// Generally SendMessageResult does not need to return an Option<Vec<Message>>. self.message_id gives an Option<String>
// But because of printer function needs to use a trait Object. We make the SendMessageResult return what ReceiveMessageResult returns, and that becomes our trait return type.
impl Omkar for SendMessageResult {
    fn get(&self) -> Option<Vec<Message>> {
        Some(vec![Message {
            attributes: None,
            body: self.message_id.clone(),
            md5_of_body: None,
            md5_of_message_attributes: None,
            message_attributes: None,
            message_id: self.message_id.clone(),
            receipt_handle: None
        }])
    }
}

// Returns a Option<Vec<Message>> which becomes the basis for the return type of get() in trait Omkar
impl Omkar for ReceiveMessageResult {
    fn get(&self) -> Option<Vec<Message>> {
        self.messages.clone()
    }
}

// A dummy struct for default match case
struct DummyMessageResult {}

// Implement Omkar for DummyMessageResult
impl Omkar for DummyMessageResult {}

// Takes a choice which the user choose to ran, and take the data from the trait object.
fn printer(c: &str, i: u64, o: Box<dyn Omkar>) {
    match c {
        "send" => {
            // This prints only one message_id per request made. The Vec wrapper is a compromise for creating a trait.
            for itr in o.get().unwrap_or(vec![]){
                println!("{:?} (✅) - {} - Made the request: {} for => {:?}","Ok", common::local_dt(), i, itr.message_id.unwrap_or("Null".to_string()))
            }
        },
        "receive" => {
            // Prints the body of each individual message for a batch of messages which comes for a single request.
            // The max number of messages is not gauranteed.
            for itr in o.get().unwrap_or(vec![]){
                println!("{:?} (✅) - {} - Received data for the request: {} body => {:?}","Ok", common::local_dt(), i, itr.body.unwrap_or("Null".to_string()))
            }
        }
        _  => unimplemented!()
    }

}