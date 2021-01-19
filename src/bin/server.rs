use std::convert::TryInto;

use libzmq::{prelude::*, Msg, ServerBuilder, TcpAddr};

use examples::{self, Result};

const PONG: &str = "PONG";

fn main() -> Result<()> {
    let endpoint: TcpAddr = format!("0.0.0.0:{}", examples::SERVER_PORT).try_into()?;
    let server = ServerBuilder::new().bind(endpoint).build()?;

    loop {
        let received_message = server.recv_msg()?;
        println!("Message arrivant : {}", received_message.to_str()?);
        let mut message_to_send = Msg::from(PONG);
        message_to_send.set_routing_id(client_id);
        server.send(message_to_send)?;
    }
}