/**
 * This file is part of APIRUNNER.
 * APIRUNNER is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or 2 any later version.
 * APIRUNNER is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.
 * You should have received a copy of the GNU General Public License along with APIRUNNER. If not, see <https://www.gnu.org/licenses/>.
**/

/* Copyright 2021 Omkaram Venkatesh */

/**
 * This is a bulk API caller CLI program written in Rust. You can make any Method call via this program for now.
 * To use the program go through the README.md file. Please use this for legitimate interests only.
 * 
**/

use reqwest;
use std::error::Error;
use std::fs;
use std::env;
use std::sync::atomic::AtomicUsize;


static GLOBAL_SUCCESS_COUNT: AtomicUsize = AtomicUsize::new(0);
static GLOBAL_FAILED_COUNT: AtomicUsize = AtomicUsize::new(0);

mod sequential; mod common; mod parallel;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct RequestDetail {
    iterate_times: u64,
    method: String,
    url: String,
    headers: serde_json::Value,
    body: serde_json::Value,
    parallel_request_response_wait_time: u128
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let data = fs::read_to_string(&args[1]).expect("Unable to read file");
    let mode = &args[2];
    let json: RequestDetail = serde_json::from_str(&data[..]).expect("JSON was not well-formatted");
    let client = reqwest::Client::new();
    let tokio_rt = tokio::runtime::Runtime::new().unwrap();

    println!("\nWelcome to APIRUNNER.");
    println!("\n~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");
    println!("Developer: Omkaram Venkatesh; Issued Under GPLV3; Copyright (2021)");
    println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");
    println!("\nSource code available at https://github.com/leogitpub/apirunner");
    println!("\nStarting the program ...");
    println!("\n{:#?}", &json);
    common::pause();
    println!("\n===========================================================");
    println!("\nInitiating the API calls for {} times. Ctrl + C to break!!!\n", json.iterate_times);
    let elapsed = match mode.as_str() {
        "seq"  => sequential::start(&json, &client, &tokio_rt),
        "par" => parallel::start(&json, &client),
        _ => None
    };
    println!("\n============Results==============\n");
    println!("Time taken to finish the API calls: {:?}", elapsed.unwrap());
    println!("\nAPI call count: Success => {:?}, Failed => {:?}\n", GLOBAL_SUCCESS_COUNT, GLOBAL_FAILED_COUNT);

}

