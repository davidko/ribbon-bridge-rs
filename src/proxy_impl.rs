
use protobuf::Message;
use std::borrow::Borrow;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use super::{ReplyHandler, WriteCallback};
use rpc;

pub struct ProxyImpl
{
    write_cb: Option<Box<WriteCallback>>,
    reply_handlers: Arc<Mutex<HashMap<u32, ReplyHandler>>>, // reply_id, payload
    request_id: u32,
}

impl ProxyImpl
{
    pub fn new() -> ProxyImpl{
        let proxy = ProxyImpl {
            write_cb: None,
            reply_handlers: Arc::new(Mutex::new(HashMap::new())),
            request_id: 0,
        };
        proxy
    }

    pub fn connect<W>(&mut self, mut write_cb: W) -> Result<(), String> 
        where W: FnMut(&[u8])->Result<(), ::std::io::Error> + 'static + Send
    {
        self.write_cb = Some(Box::new(write_cb));
        // Create a "Connect" request
        let mut request = rpc::Request::new();
        request.set_field_type(rpc::Request_Type::CONNECT);
        // Send it
        self.request(request, Some(Box::new(|reply| { println!("Received connect request reply."); })));
        Ok(())
    }

    pub fn request(&mut self, request: rpc::Request, cb: Option<ReplyHandler>) {
        println!("rpc_request start");
        // Add the callback reply handler
        if let Some(func) = cb {
            let ref mut map = self.reply_handlers.lock().unwrap();
            map.insert(self.request_id, func);
        }

        // Send the request
        // First, form a new ClientMessage
        let mut cm = rpc::ClientMessage::new();
        cm.set_id(self.request_id);
        cm.set_request(request);

        // Turn the client message into a byte stream
        let bytes_vec = cm.write_to_bytes();

        match self.write_cb{
            Some(ref mut f) => {
                f(&bytes_vec.unwrap().as_slice()).expect("ProxyHandler.on_write() failed");
            }
            _ => {}
        }

        self.request_id += 1;
        println!("rpc_request end");
    }

    pub fn deliver(&mut self, buf: &[u8]) {
        let mut msg = rpc::ServerMessage::new();
        msg.merge_from_bytes(buf).expect("Could not parse reply payload");
        match msg.get_field_type() {
            rpc::ServerMessage_Type::REPLY => {
                // See if there is a reply handler in our hashmap
                let id = msg.get_inReplyTo();
                let mut hashmap = self.reply_handlers.lock().unwrap();
                if let Some(cb) = hashmap.get_mut(&id) {
                    cb(msg.take_reply());
                }
            },
            rpc::ServerMessage_Type::BROADCAST => {
                println!("Received broadcast.");
            },
        }
    }
}
