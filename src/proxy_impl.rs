
use futures;
use futures::Future;
use protobuf::Message;
use rpc;
use std::collections::HashMap;
use std::fmt;
use std::sync::{Arc, Mutex};
use super::{WriteCallback, ReplyFuture, ResultFuture, hash};

pub struct ProxyImpl
{
    write_cb: Option<Box<WriteCallback>>,
    reply_handlers: Arc<Mutex<HashMap<u32, futures::sync::oneshot::Sender<rpc::Reply>>>>, // reply_id, payload
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

    pub fn connect<W>(&mut self, write_cb: W) -> ReplyFuture
        where W: FnMut(Vec<u8>)->Result<(), ::std::io::Error> + 'static + Send
    {
        self.write_cb = Some(Box::new(write_cb));
        // Create a "Connect" request
        let mut request = rpc::Request::new();
        request.set_field_type(rpc::Request_Type::CONNECT);
        self.request(request)

    }

    pub fn request(&mut self, request: rpc::Request) -> ReplyFuture {
        println!("rpc_request start");
        // Add the callback reply handler
        let (tx, rx) = futures::sync::oneshot::channel();
        let ref mut map = self.reply_handlers.lock().unwrap();
        map.insert(self.request_id, tx);

        // Send the request
        // First, form a new ClientMessage
        let mut cm = rpc::ClientMessage::new();
        cm.set_id(self.request_id);
        cm.set_request(request);

        // Turn the client message into a byte stream
        let bytes_vec = cm.write_to_bytes();

        match self.write_cb{
            Some(ref mut f) => {
                f(bytes_vec.unwrap()).expect("ProxyHandler.on_write() failed");
            }
            _ => {}
        }

        self.request_id += 1;
        println!("rpc_request end");
        rx.boxed()
    }

    // The payload should be a "In" protobuf message
    pub fn fire(&mut self, name: &str, payload: Vec<u8>) -> ResultFuture
    {
        let mut request = rpc::Request::new();
        request.set_field_type(rpc::Request_Type::FIRE);
        let mut fire = rpc::Request_Fire::new();
        fire.set_id( hash(name) );
        fire.set_payload( payload );
        request.set_fire(fire);
        self.request(request).map( |mut reply| {
            match reply.get_field_type() {
                rpc::Reply_Type::RESULT => {
                    reply.take_result().take_payload()
                },
                _ => {vec![]}
            }
        }).boxed()
    }

    // Deliver bytes from RPC Server to this function
    pub fn deliver(&mut self, buf: Vec<u8>) {
        let mut msg = rpc::ServerMessage::new();
        msg.merge_from_bytes(buf.as_slice()).expect("Could not parse reply payload");
        match msg.get_field_type() {
            rpc::ServerMessage_Type::REPLY => {
                println!("Got rpc reply.");
                // See if there is a reply handler in our hashmap
                let id = msg.get_inReplyTo();
                let mut hashmap = self.reply_handlers.lock().unwrap();
                if let Some(cb) = hashmap.remove(&id) {
                    println!("Resolving RPC reply future.");
                    cb.complete(msg.take_reply());
                }
            },
            rpc::ServerMessage_Type::BROADCAST => {
                println!("Received broadcast.");
            },
        }
    }
}

impl fmt::Display for rpc::VersionTriplet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "v{}.{}.{}", self.get_major(), self.get_minor(), self.get_patch())
    }
}

impl fmt::Display for rpc::Versions {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "rpc: {} interface: {}", self.get_rpc(), self.get_interface())
    }
}
