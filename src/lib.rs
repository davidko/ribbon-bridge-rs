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

fn hash(name: &str) -> u32 {
    let bytes = name.as_bytes();
    let mut h : u32 = 0;
    for b in bytes {
        h = (h.wrapping_mul(101).wrapping_add(*b as u32));
    }
    return h;
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
                let request = cm.get_request();
                match request.get_field_type() {
                    rpc::Request_Type::CONNECT => {
                        self.handle_connect();
                    }
                    rpc::Request_Type::DISCONNECT => {
                        self.handle_disconnect();
                    }
                    rpc::Request_Type::FIRE => {
                        self.handle_fire( request.get_fire() );
                    }
                }
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

    fn handle_connect(&self) {
        // Send a "Versions" message back to the client
        let mut reply = rpc::Reply::new();
        reply.set_field_type(rpc::Reply_Type::VERSIONS);
        let mut versions = rpc::Versions::new();
        let mut triplet = rpc::VersionTriplet::new();
        triplet.set_major(0);
        triplet.set_minor(2);
        triplet.set_patch(2);
        versions.set_rpc(triplet);
        let mut triplet = rpc::VersionTriplet::new();
        triplet.set_major(0);
        triplet.set_minor(2);
        triplet.set_patch(2);
        reply.set_versions(versions);
        self._server.write( reply.write_to_bytes().unwrap().as_slice() );
    }

    fn handle_disconnect(&self) {
    }

    fn handle_fire(&self, fire: &rpc::Request_Fire) {
    }
}
