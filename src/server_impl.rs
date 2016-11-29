use protobuf;
use protobuf::Message;
use rpc;
use std::collections::HashMap;
use std::io::Write;
use std::io;
use super::hash;

pub struct _Server<S:Write> {
    fire_handlers: HashMap<u32, Box<FnMut(Vec<u8>) -> Vec<u8>>>,
    stream: Option<S>
}

impl<S:Write> _Server<S> {
    pub fn new() -> _Server<S> {
        return _Server {
            fire_handlers : HashMap::new(),
            stream: None
        }
    }

    // func: Return a rpc::Reply::Result::payload Vec<u8>
    pub fn on<F>(&mut self, name: &str, func: F) 
        where F: FnMut(Vec<u8>) -> Vec<u8>,
              F: 'static {
        self.fire_handlers.insert(hash(name), Box::new(func));
    }

    pub fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match self.stream {
            Some(ref mut stream) => { return stream.write(buf); }
            None => { return Err(io::Error::new(io::ErrorKind::Other, "No available write stream.")); }
        }
    }

    pub fn deliver(&mut self, data: &[u8])
    {
        // 'data' should be a 'ClientMessage'
        let cm_result = protobuf::parse_from_bytes::<rpc::ClientMessage>(data);
        match cm_result {
            Ok(cm) => {
                /* Need to match the type of request */
                match cm.get_request().get_field_type() {
                    rpc::Request_Type::CONNECT => {
                    }
                    rpc::Request_Type::DISCONNECT => {
                    }
                    rpc::Request_Type::FIRE => {
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
                self.send_reply(reply, 0);
            }
        }
    }

    fn reply_versions(&mut self, in_reply_to: u32) {
        let mut rpc_version = rpc::VersionTriplet::new();
        rpc_version.set_major(0);
        rpc_version.set_minor(0);
        rpc_version.set_patch(1);

        let mut interface_version = rpc::VersionTriplet::new();
        interface_version.set_major(0);
        interface_version.set_minor(0);
        interface_version.set_patch(0);

        let mut reply = rpc::Reply::new();
        let mut versions = rpc::Versions::new();
        versions.set_rpc(rpc_version);
        versions.set_interface(interface_version);
        reply.set_field_type(rpc::Reply_Type::VERSIONS);
        reply.set_versions(versions);
        self.send_reply(reply, in_reply_to);
    }

    fn send_reply(&mut self, reply: rpc::Reply, in_reply_to: u32)
    {
        let mut sm = rpc::ServerMessage::new();
        sm.set_field_type(rpc::ServerMessage_Type::REPLY);
        sm.set_reply(reply);
        sm.set_inReplyTo(in_reply_to);
        self.write( sm.write_to_bytes().unwrap().as_slice() );
    }

    fn send_result(&mut self, result_payload: Vec<u8>, in_reply_to: u32)
    {
        let mut result = rpc::Reply_Result::new();
        result.set_payload(result_payload);
        
        let mut reply = rpc::Reply::new();
        reply.set_field_type(rpc::Reply_Type::RESULT);
        reply.set_result(result);
        self.send_reply(reply, in_reply_to);
    }

    fn handle_fire(&mut self, fire_id: u32, request_id: u32, payload: &[u8]) {
        match self.fire_handlers.get(&fire_id) {
            Some(func) => {
            }
            None => {}
        }
    }
}
