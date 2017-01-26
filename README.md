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

## Event Loops

+ listener (eg `TcpListener`) has `incoming` method that returns iterator over
  all events
+ `Core`'s `run` method sets up the event loop, taking a listener iterator
+ Within iterator, handle each event
+ `Handle` gives a reference to the event loop, allowing the processing of an
  event (asynchronously) to spawn additional tasks on the event loop
+ `Handle` puts all tasks on the event-loop thread: it is not thread safe (ie no
  `Send`)
+ `Remote` is `Send`. `spawn`s a *closure* that creates a future (and is `Send`)
+ `Remote::spawn`'s closure can be executed on a different thread

## High-level IO

+ TCP/UDP - only async IO available across all rust platforms
+ Tokio `TcpListener`, `TcpStream`, `UdpSocket` non-blocking; return futures or
  streams
+ `tokio_core::io` helper methods. Can be replaced with more specific logic;
   only work with "future aware"/non-blocking implementations of `Read` and `Write`
+ helpers return futures that yield back ownership. Futures must be 'static;
  take IO by value
+ `Io::split` takes a `Read + Write` and returns separate `Read` and `Write` for
  ownership ease
+ `Io::framed` takes `Codec` that takes stream/sink of bytes -> `Stream` and
  `Sink` impls
+ returns `Framed` which impls `Sink` and `Stream`. Can be split with
  `Io::split`
+ `Codec` provides `EasyBuf` allows easy buffering through `drain_to`
+ To encode, append bytes to provided `Vec`
+ `UdpSocket` is *not* byte stream. Impls convenience methods
