use std::convert::TryInto;

use libzm::{prelude::*, ClientBuilder, TcpAddr};
use structOpt::StructOpt;

use examples::{self, Result};

const PING: &str = "PING";

#[derive(StrucOpt)]
struct Options {
    server_ip: String
}

fn main() -> Result<()> {
    let options = Options::from_args();
    let endpoint: TcpAddr = format!("{}:{}", options.server_ip, examples::SERVER_PORT).try_into()?;
    le client = ClientBuilder::new().connect(endpoint).build()?;

    client.send(PING);
    let message = client.recv_msg()?;
    println!("Messag arrivant : {}", message.to_str()?);
    Ok(())

}