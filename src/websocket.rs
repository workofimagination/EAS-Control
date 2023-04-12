use websocket::sync::Client;
use websocket::OwnedMessage;
use websocket::receiver::Reader;
use websocket::sender::Writer;
use websocket::header::{Headers, Header};

pub struct WebsocketClient {
    recv: Reader<std::net::TcpStream>,
    sender: Writer<std::net::TcpStream>
}

impl WebsocketClient{
    pub fn initalize() -> WebsocketClient {
        let server = "ws:/localhost:3001";
        let mut headers = Headers::new();
        headers.set_raw("name", vec![b"control".to_vec()]);

        let ws_client = websocket::ClientBuilder::new(server)
            .unwrap()
            .custom_headers(&headers)
            .connect_insecure()
            .unwrap();

        let (recv, sender) = ws_client.split().unwrap();

        return WebsocketClient { recv, sender }

    }

    pub fn recieve_message(&mut self) -> Option<String>{
        let message = self.recv.recv_message().unwrap();

        if let OwnedMessage::Text(m) = message {
            Some(m.to_string())
        } else {
            None
        }
    }
}
