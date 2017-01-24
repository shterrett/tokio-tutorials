# Learning Tokio

This project will work through all of the Tokio tutorials available at
https://tokio.rs/docs/getting-started/tokio/

## Echo Server

+ `Codec` parses/encodes the request and response data
+ `Proto` handles receiving the request/submitting the response
+ `Service` transforms the parsed request into the not-yet-parsed response
+ `Proto` holds the specific `Codec`
+ `Server` is initialized with a `Proto`
+ `Server` then serves the `Service`
