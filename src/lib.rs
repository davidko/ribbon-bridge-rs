extern crate protobuf;

mod server_impl;
mod proxy_impl;
mod rpc;

use protobuf::Message;
use std::io::Write;
use std::io::Read;

type ReplyHandler = Box<Fn(rpc::Reply) + Send>;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}

pub struct Server<W:Write>{
    _server: server_impl::_Server<W>
}

impl<W:Write> Server<W>{
    fn new() -> Server<W> {
        return Server { _server: server_impl::_Server::new() };
    }

    // Register a function to be called to handle FIRE requests
    fn on<F>(&mut self, name: String, func: F)
        where F: FnMut(Vec<u8>) -> Vec<u8>,
              F: 'static
    {
        self._server.on(name, func);
    }

    fn deliver(&mut self, data: Vec<u8>)
    {
        // 'data' should be a 'ClientMessage'
        let cm_result = protobuf::parse_from_bytes::<rpc::ClientMessage>(data.as_slice());
        match cm_result {
            Ok(cm) => {
            }
            _ => {
                // Return error reply
                let mut reply = rpc::Reply::new();
                let mut status = rpc::Reply_Status::new();
                status.set_value(rpc::Status::DECODING_FAILURE);
                reply.set_field_type(rpc::Reply_Type::STATUS);
                reply.set_status(status);
                self._server.write( reply.write_to_bytes().unwrap().as_slice() );
            }
        }
    }
}

// T: ProxyHandler
pub struct Proxy<T> {
    _proxy: proxy_impl::ProxyImpl<T>
}

impl<T: FnMut(&[u8]) -> Result<(), String>> Proxy<T> {
    pub fn new() -> Proxy<T> {
        let proxy = Proxy {
            _proxy: proxy_impl::ProxyImpl::new(),
        };
        proxy
    }

    // write_callback: This callback is called if/when the Proxy needs to
    // send data to the underlying transport connected to the server.
    pub fn connect(&mut self, write_callback: T) -> Result<(), String> 
    {
        self._proxy.connect(write_callback)
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
