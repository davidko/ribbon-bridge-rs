extern crate core;
extern crate futures;
extern crate protobuf;

mod server_impl;
mod proxy_impl;
mod rpc;

use protobuf::Message;
use std::io::Write;
use std::io::Read;

pub type FireHandler = Fn(Vec<u8>) -> Result<Vec<u8>, rpc::Status>;
pub type ReplyHandler = Box<Fn(rpc::Reply) + Send>;
pub type WriteCallback = FnMut(&[u8])->Result<(), ::std::io::Error> + 'static + Send;

pub type ReplyFuture = futures::BoxFuture<rpc::Reply, futures::Canceled>;

fn hash(name: &str) -> u32 {
    let mut h:u32 = 0;
    for c in name.bytes() {
        h = 101*h + (c as u32);
    }
    h
}

pub struct Server{
    _server: server_impl::_Server
}

impl Server{
    fn new() -> Server {
        return Server { _server: server_impl::_Server::new() };
    }

    // Register a function to be called to handle FIRE requests
    fn on<F>(&mut self, name: &str, func: F)
        where F: Fn(Vec<u8>) -> Result<Vec<u8>, rpc::Status>,
              F: 'static
    {
        self._server.on(name, func);
    }

    fn deliver(&mut self, data: Vec<u8>)
    {
        self._server.deliver(data.as_slice())
    }
}

pub struct Proxy
{
    _proxy: proxy_impl::ProxyImpl
}

impl Proxy
{
    pub fn new() -> Proxy {
        let proxy = Proxy {
            _proxy: proxy_impl::ProxyImpl::new(),
        };
        proxy
    }

    // write_callback: This callback is called if/when the Proxy needs to
    // send data to the underlying transport connected to the server.
    pub fn connect<W>(&mut self, write_callback: W) -> ReplyFuture
        where W: FnMut(&[u8])->Result<(), ::std::io::Error> + 'static + Send
    {
        self._proxy.connect(write_callback)
    }

    pub fn fire<M>(&mut self, name: &str, payload: &M) -> ReplyFuture
        where M: Message
    {
        self._proxy.fire(name, payload)
    }

/*
    pub fn fire<M, F>(&mut self, name: &str, message: &M, callback: F)
        where M: ::protobuf::Message,
              F: Fn(M) 
    {
        self._proxy.fire(name, message, callback);
    }
    */
    
    // Deliver data from underlying transport to here
    pub fn deliver(&mut self, buf: &[u8]) {
        self._proxy.deliver(buf);
    }
}

#[cfg(test)]
mod tests {
    use super::Proxy;
    struct MyStruct<T> {
        proxy:T
    }
    #[test]
    fn it_works() {
        println!("Hello!");
        let my_struct = MyStruct { proxy: Proxy::new() };
        let mut proxy = my_struct.proxy;
        proxy.connect( move |data| { println!("write callback"); Ok(()) } );
    }
}

