extern crate futures;
extern crate tokio_core;

use futures::Future;
use futures::stream::Stream;
use tokio_core::reactor::Core;
use tokio_core::net::TcpListener;

fn main() {
    let mut core = Core::new().unwrap();
    let address = "127.0.0.1:3000".parse().unwrap();
    let listener = TcpListener::bind(&address, &core.handle()).unwrap();

    let handle = core.handle();
    let connections = listener.incoming();
    let welcomes = connections.and_then(|(socket, _)| {
        let serve_one = tokio_core::io::write_all(socket, b"Hello world!\n").then(|_| Ok(()));
        handle.spawn(serve_one);
        Ok(())
    });
    let server = welcomes.for_each(|_| {
        Ok(())
    });

    core.run(server).unwrap();
}
