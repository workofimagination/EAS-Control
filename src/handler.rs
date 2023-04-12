use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::websocket::WebsocketClient;

struct Handler {
    client: WebsocketClient,
    op_map: HashMap<usize, fn()>
}

#[derive(Deserialize, Serialize)]
struct Request {
    message: String,
    op_code: usize
}

impl Handler {
    pub fn create() -> Handler {
        let client = WebsocketClient::initalize();
        let mut op_map: HashMap<usize, fn()> = HashMap::new();

        op_map.insert(1, Handler::temp);

        return Handler { client, op_map }
    }

    pub fn temp() {
        println!("hello world");
    }

    pub fn start(&mut self) {
        loop {

            let request: String = match self.client.recieve_message() {
                Some(x) => x,
                None => continue
            };

            self.handle_request(request);
        }
    }

    pub fn handle_request(&mut self, request: String) {
        let request: Request = match serde_json::from_str(&request) {
            Ok(o) => o,
            Err(_) => {self.send_error("unable to parse json".to_string()); return}
        };

        let operation = match self.op_map.get(&request.op_code) {
            Some(o) => o,
            None => { self.send_error("unable to execute op_code".to_string()); return }
        };

        operation();
    }

    fn send_error(&mut self, message: String) {
        let response = Request {
            message,
            op_code: 0
        };

        let message = serde_json::to_string(&response).unwrap();

        self.client.sender.send_message(&websocket::OwnedMessage::Text(message)).unwrap();
    }
}
