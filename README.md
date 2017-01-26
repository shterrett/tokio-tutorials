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


## db/futures

+ Each connection processed by separate thread in thread pool.
+ Processing done in a future - allows requests to be processed concurrently;
  returns when future is evaluated.

## Streams

+ Provide in-order, infinite stream processing using futures
+ Can create a `core.handle` and spwan a lightweight thread for each stream
  element. This makes work asynchronous.
+ Async, lightweight threads using `core.handle` good for io-intensive work
+ Prefer a thread pool for cpu intensive work
