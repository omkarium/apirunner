/**
 * This file is part of APIRUNNER.
 * APIRUNNER is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or 2 any later version.
 * APIRUNNER is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.
 * You should have received a copy of the GNU General Public License along with APIRUNNER. If not, see <https://www.gnu.org/licenses/>.
**/

/*use aws_sdk_sqs as sqs;
use std::time::Instant;
#[derive(Debug)]
struct SQSMessage {
    body: String,
    group: String,
}

#[tokio::main]
pub async fn start() -> std::option::Option<std::time::Duration> {
    let config = aws_config::load_from_env().await;
    let client = sqs::Client::new(&config);
    let start_time = Instant::now();
    let queue_list = find_first_queue(&client).await;
    println!("{:?}", &queue_list.as_ref().unwrap());
    let message = SQSMessage {
        body: "hello from my queue".to_string(),
        group: "MyGroup".to_string()
    };

    let a = send(&client, &"".to_string(), &message).await;
    let b = receive(&client, &queue_list.as_ref().unwrap()).await;

    let elapsed = Some(start_time.elapsed());
    elapsed

}

async fn find_first_queue(client: &sqs::Client) -> Result<String, sqs::Error> {
    let queues = client.list_queues().send().await?;
    let queue_urls = queues.queue_urls().unwrap_or_default();
    Ok(queue_urls
        .first()
        .expect("No queues in this account and Region. Create a queue to proceed.")
        .to_string())
}

async fn send(client: &sqs::Client, queue_url: &String, message: &SQSMessage) -> Result<(), sqs::Error> {
    println!("Sending message to queue with URL: {}\n", queue_url);
 println!("hello");
 for i in 1..10 {
    let rsp = client.send_message().queue_url(queue_url).message_body(&message.body).message_group_id(&message.group).delay_seconds(0).send().await?;
    println!("SHI: {:?}\n", rsp);
}

    Ok(())
}
use core::time::Duration;
use tokio::time::sleep;
async fn receive(client: &sqs::Client, queue_url: &String) -> Result<(), sqs::Error> {
    
    for i in 1..10 {
    let rcv_message_output = 
        client.receive_message()
        .max_number_of_messages(10)
        .queue_url(queue_url)
        .send().await?;

    //println!("Messages from queue with url: {} {:?}", queue_url, &rcv_message_output);
    for message in rcv_message_output.messages.unwrap_or_default() {
        println!("Got the message: {:#?}", message.body().unwrap());
    }
    }
   
    Ok(())
}
*/
