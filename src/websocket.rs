use websocket::sync::Client;
use websocket::OwnedMessage;
use websocket::receiver::Reader;
use websocket::sender::Writer;

pub struct WebsocketConnection {
    recv: Reader<std::net::TcpStream>,
    sender: Writer<std::net::TcpStream>
}

impl WebsocketClient{
    pub fn initalize() -> WebsocketConnection {
        let server = "ws:/0.0.0.0:3002";
        let ws_client = websocket::ClientBuilder::new(server)
            .unwrap()
            .connect_insecure()
            .unwrap();

        let (recv, sender) = ws_client.split().unwrap();

        return WebsocketConnection { recv, sender }

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