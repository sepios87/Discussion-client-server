use std::convert::TryInto;
use std::thread;
use std::time::Duration;

use libzmq::{prelude::*, Group, Msg, RadioBuilder, TcpAddr};

use examples::{self, Result};

const NB_ITERATION: usize = 100;

fn main() -> Result<()> {
    let endpoint: TcpAddr = format!("0.0.0.0:{}", examples::RADIO_PORT).try_into()?;
    let radio = RadioBuilder::new().bind(endpoint).build()?;

    for i in 0..NB_ITERATION {
        let mut message = Msg::from(format!("Bonjour - {}", i));
        let group: Group = "Limoges".try_into()?;
        message.set_group(group);
        radio.send(message.clone())?;
        thread::sleep(Duration::from_secs(1));
    }
    Ok(())
}