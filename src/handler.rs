use crate::websocket::WebsocketClient;

struct Handler {
    client: WebsocketClient
}

impl Handler {
    pub fn start(&mut self) {
        loop {

            let request: String = match self.client.recieve_message() {
                Some(x) => x,
                None => continue
            };

            self.handle_request(request);
        }
    }

    pub fn handle_request(&self, request: String) {

    }
}
