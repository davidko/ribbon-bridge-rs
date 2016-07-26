extern crate protobuf;

mod server_impl;
mod rpc;

use protobuf::Message;
use std::io::Write;

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
