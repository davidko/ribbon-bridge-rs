
use std::collections::HashMap;

pub struct _Server {
    fire_handlers: HashMap<String, Box<FnMut(Vec<u8>) -> Vec<u8>>>
}

impl _Server {
    pub fn new() -> _Server {
        return _Server {
            fire_handlers : HashMap::new()
        }
    }

    fn on<F>(&mut self, name: String, func: F) 
        where F: FnMut(Vec<u8>) -> Vec<u8>,
              F: 'static {
        self.fire_handlers.insert(name, Box::new(func));
    }
}
