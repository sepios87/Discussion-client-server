use std::convert::TryInto;

use libzmq::{prelude::*, DishBuilder, Group, TcpAddr};
use structopt::StructOpt;

use examples::{self, Result};

#[derive(StructOpt)]
struct Options {
    server_ip: String
}

fn main() -> Result<()> {
    let options = Options::from_args();
    let endpoint: TcpAddr = format!("{}:{}", options.server_ip, examples::RADIO_PORT).try_into()?;
    let group: Group = "LImoges".try_into()?;
    let dish = DishBuilder::new().connect(endpoint).join(group).build()?;

    loop {
        let message = dish.recv_msg()?;
        println!("{}", message.to_str()?);
    }
}