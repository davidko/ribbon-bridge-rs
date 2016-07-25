mod server_impl;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}

pub struct Server{
    _server: server_impl::_Server
}

impl Server{
    fn new() -> Server {
        return Server { _server: server_impl::_Server::new() };
    }

    // Register a function to be called to handle FIRE requests
    fn on<F>(&self, name: String, func: F)
        where F: FnMut(Vec<u8>) -> Vec<u8>,
              F: 'static
    {
        // TODO
    }
}
