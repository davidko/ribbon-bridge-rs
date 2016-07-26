
use std::collections::HashMap;
use std::io::Write;
use std::io;

pub struct _Server<S:Write> {
    fire_handlers: HashMap<String, Box<FnMut(Vec<u8>) -> Vec<u8>>>,
    stream: Option<S>
}

impl<S:Write> _Server<S> {
    pub fn new() -> _Server<S> {
        return _Server {
            fire_handlers : HashMap::new(),
            stream: None
        }
    }

    pub fn on<F>(&mut self, name: String, func: F) 
        where F: FnMut(Vec<u8>) -> Vec<u8>,
              F: 'static {
        self.fire_handlers.insert(name, Box::new(func));
    }

    pub fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match self.stream {
            Some(ref mut stream) => { return stream.write(buf); }
            None => { return Err(io::Error::new(io::ErrorKind::Other, "No available write stream.")); }
        }
    }
}
