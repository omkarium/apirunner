/**
MIT License
Copyright (c) Omkaram Venkatesh

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights to use,
copy, modify, merge, publish, distribute, sublicense, and/or
sell copies of the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following conditions:
The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.
THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY,
WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
-----------------------------------------------------------------------------------
This is a bulk API caller CLI program written in Rust. Ypu can only make GET calls via this program.
To use the program go through the README.md file. Please use this for legitimate interests only.
**/

extern crate colour;
use reqwest;
use std::error::Error;
use std::fs;
use std::env;

#[derive(serde::Serialize, serde::Deserialize)]
struct RequestDetail {
    Call_Count: u64,
    Url: String,
    Accept: Option<String>,
    Accept_Charset: Option<String>,
    Content_Type: Option<String>,
    Accept_Encoding: Option<String>,
    Content_Encoding: Option<String>,
    Authorization: Option<String>,
    Connection: Option<String>,
    Content_Length: Option<String>,
    Host: Option<String>,
    User_Agent: Option<String>
}

fn main() {

    let args: Vec<String> = env::args().collect();
    let data = fs::read_to_string(&args[1]).expect("Unable to read file");
    let json: RequestDetail = serde_json::from_str(&data[..]).expect("JSON was not well-formatted");
    let client = reqwest::Client::new();
    let mut tokio_rt = tokio::runtime::Runtime::new().unwrap();

    println!("\n~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");
    println!("Developer: Omkaram Venkatesh; Issued Under MIT License; Copyright (2022)");
    println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");
    println!("\nSource code available at https://github.com/leogitpub/apirunner");
    println!("\nStarting the program ...");
    println!("\nAPI URL Passed: {:?}", json.Url);
    println!("User Agent: {:?}", json.User_Agent);
    println!("Accept: {:?}", json.Accept);
    println!("Accept_Charset: {:?}", json.Accept_Charset);
    println!("Accept_Encoding: {:?}", json.Accept_Encoding);
    println!("Authorization: {:?}", json.Authorization);
    println!("Connection: {:?}", json.Connection);
    println!("Content_Length: {:?}", json.Content_Length);
    println!("Host type: {:?}", json.Host);
    println!("User_Agent: {:?}", json.User_Agent);

    println!("\n===========================================================");
    println!("\nInitiating the GET (only) API calls for {} times. Ctrl + C to break!!!\n", json.Call_Count);

    for i in 1..json.Call_Count{
        let result = tokio_rt.block_on(calling_api(&client, &json));
        match result {
            Ok(_) => println!("{:?} (✅) Made the request: {} for => {}", colour::green!("Ok"), i, &json.Url),
            Err(e) => println!("{:?} (❎) Request: {} failed with {}", colour::red!("Error"), i, e)
        }    
    }

}

async fn calling_api(client: &reqwest::Client, rd: &RequestDetail) -> Result<(), Box<dyn Error>> {
    
    client 
        .get(&rd.Url)
        .header("Accept", rd.Accept.clone().unwrap_or("None".to_string()))
        .header("Accept-Charset", rd.Accept_Charset.clone().unwrap_or("None".to_string()))
        .header("Content-Type", rd.Content_Type.clone().unwrap_or("None".to_string()))
        .header("Accept-Encoding", rd.Accept_Encoding.clone().unwrap_or("None".to_string()))
        .header("Content-Encoding", rd.Content_Encoding.clone().unwrap_or("None".to_string()))
        .header("Authorization", rd.Authorization.clone().unwrap_or("None".to_string()))
        .header("Connection", rd.Connection.clone().unwrap_or("None".to_string()))
        .header("Content-Length", rd.Content_Length.clone().unwrap_or("None".to_string()))
        .header("Host", rd.Host.clone().unwrap_or("None".to_string()))
        .header("User-Agent", rd.User_Agent.clone().unwrap_or("None".to_string()))
        .send()
        .await?.text().await?;

    Ok(())

}
