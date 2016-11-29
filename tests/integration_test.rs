extern crate ribbon_bridge as rb;
extern crate futures;

use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Mutex, Arc};
use std::thread;
use futures::Future;

#[test]

fn integration_test() {
    // Set up our comms channels
    let (p_to_s_tx, p_to_s_rx) = channel();
    let (s_to_p_tx, s_to_p_rx) = channel();

    // Set up our server
    let server = Arc::new(Mutex::new(rb::Server::new()));

    {
        // Set up server listen pump
        let server = server.clone();
        thread::spawn( move || {
            loop {
                let msg = p_to_s_rx.recv().unwrap();
                println!("Server received bytes.");
                {
                    let mut server = server.lock().unwrap();
                    server.deliver(msg);
                }
                println!("Server delivered.");
            }
        });
    }
    {
        // Set up the server write-callback
        let mut server = server.lock().unwrap();
        server.set_write_callback( move |buf| {
            println!("Server write callback invoked.");
            s_to_p_tx.send(buf);
            println!("Server write callback done.");
            Ok(())
        });
    }

    // Set up proxy listen pump

    let proxy = Arc::new(Mutex::new(rb::Proxy::new()));

    {
        let proxy = proxy.clone();
        thread::spawn( move || {
            loop {
                println!("Proxy waiting for bytes...");
                let msg = s_to_p_rx.recv().unwrap();
                println!("Proxy received bytes.");
                let mut proxy = proxy.lock().unwrap();
                proxy.deliver(msg);
                println!("Proxy delivered.");
            }
        });
    }

    {
        let server = server.clone();
        let proxy = proxy.clone();
        let thread = thread::spawn( move || {
            let fut = 
            {
                let mut proxy = proxy.lock().unwrap();
                proxy.connect(move |buf| {
                    p_to_s_tx.send(buf);
                    Ok(())
                })
            };
            println!("Proxy Called connect.");

            match fut.wait() {
                Ok(mut reply) => {
                    println!("Received reply: {}", reply.take_versions());
                },
                Err(error) => {
                },
            }
        });
        thread.join();
    }

}
