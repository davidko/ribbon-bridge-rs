use protobuf;
use protobuf::Message;
use rpc;
use std::collections::HashMap;
use std::io::Write;
use std::io;
use super::{hash, FireHandler, WriteCallback};

pub struct _Server {
    fire_handlers: HashMap<u32, Box<FireHandler>>,
    write_cb: Option<Box<WriteCallback>>,
}

impl _Server {
    pub fn new() -> _Server {
        return _Server {
            fire_handlers : HashMap::new(),
            write_cb: None
        }
    }

    // func: Return a rpc::Reply::Result::payload Vec<u8>
    pub fn on<F>(&mut self, name: &str, func: F) 
        where F: Fn(Vec<u8>) -> Result<Vec<u8>, rpc::Status>,
              F: 'static 
    {
        self.fire_handlers.insert(hash(name), Box::new(func));
    }

    pub fn write(&mut self, buf: Vec<u8>) -> Result<(), ::std::io::Error> {
        match self.write_cb{
            Some(ref mut f) => {
                f(buf)
            }
            None => {
                Err(::std::io::Error::new(
                    ::std::io::ErrorKind::Other, 
                    "No write callback has been set."))
            }
        }
    }

    pub fn deliver(&mut self, data: Vec<u8>)
    {
        // 'data' should be a 'ClientMessage'
        let cm_result = protobuf::parse_from_bytes::<rpc::ClientMessage>(data.as_slice());
        match cm_result {
            Ok(cm) => {
                /* Need to match the type of request */
                match cm.get_request().get_field_type() {
                    rpc::Request_Type::CONNECT => {
                        // Reply with versions
                        self.reply_versions(cm.get_id());
                    }
                    rpc::Request_Type::DISCONNECT => {
                        unimplemented!();
                    }
                    rpc::Request_Type::FIRE => {
                        unimplemented!();
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

    pub fn set_write_callback<W>(&mut self, write_callback: W)
        where W: FnMut(Vec<u8>)->Result<(), ::std::io::Error> + 'static + Send
    {
        self.write_cb = Some(Box::new(write_callback));
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
        self.write( sm.write_to_bytes().unwrap() );
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

    fn handle_fire(&mut self, fire_id: u32, request_id: u32, payload: Vec<u8>) {
        // We have to completely remove the item. If we get a reference to the 
        // item inside, the borrow checker will complain because it can't 
        // guarantee that the item won't live longer than "self"
        match self.fire_handlers.remove(&fire_id){
            Some(func) => {
                match func(payload.to_vec()) {
                    Ok(result_payload) => {
                        self.send_result(result_payload, request_id);
                        self.fire_handlers.insert(fire_id, func);
                    },
                    Err(err_code) => {
                        let mut reply_status = rpc::Reply_Status::new();
                        reply_status.set_value(err_code);

                        let mut reply = rpc::Reply::new();
                        reply.set_field_type(rpc::Reply_Type::STATUS);
                        reply.set_status(reply_status);
                        self.send_reply(reply, request_id);
                        self.fire_handlers.insert(fire_id, func);
                    },
                }
            }
            None => {}
        }
    }

}
