# apirunner
This is CLI program which can call Rest APIs `n` number of times based on the Count specified in the input file. Its written in Rust. 

## How to use
1. You can do `cargo build` and `cargo run input.json seq` 
2. CMD usage is `apirunner.exe file_path_of_json [mode]` where mode can be `seq` or `par` and `sqs`
3. Use `seq` for Sequential requests to your target API. Use `par` for making Parallel HTTP API requests.
4. Use `sqs` when you want to make sqs requests like send and receive message etc to AWS SQS.

For sqs, you first need to create this file in /User/.aws/credentials. Put the following content in it
```
[default]
aws_access_key_id=dfgsd
aws_secret_access_key=sdtw3et423t2345
region=us-east-2
```

## Input file

Check the input.json I provided in the repo.
```
{
   "iterate_times": 100, // How many API calls you want to make
   "parallel_request_response_wait_time": 20, //How long are you willing to wait for responses. Only applicable for mode `par`
   "url": "https://reqres.in/api/users", // Your target API
   "method": "POST", //Method to use 
   "headers": {}, // You can pass many custom headers as a JSON object.
   "body": {      // Your JSON body. Sorry, I made it to work with JSON only.
       "name": "morpheus",
       "job": "leader"
    },
    "aws_details": {
        "action": "send", // this can be either send or receive.
        "region": "us-east-2",
        "access_key": "optional", //Not needed. You already provided this via .aws/credentials
        "secret_key": "optional", //Not needed. You already provided this via .aws/credentials
        "service": "sqs",
        "path": "optional", //Not needed.
        "max_receive_messages": 10, // use this for receive only
        "url": "https://sqs.us-east-2.amazonaws.com"
    }
}
```

# Benchmarking

* mode `par` is 1000X faster than `seq`. I was able to make 1K POST requests under 46.5 milliseconds for the above JSON provided. 
* If you go beyond 15K requests your local machine will run out of ports to dispatch.

# Invitation to contribute

If you think this program can do better, please be my guest and provide your valuable suggestions or changes. I don't see any reason to have this as a Library because main.rs is too business specific for my need, and hence I made this an executable instead.
Thanks in advance.