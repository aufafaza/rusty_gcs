use mavlink::AsyncMavConnection;
use mavlink::dialects::common::MavMessage;
use std::env;
// use mavlink::error::MessageReadError;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = env::args().collect();

    if args.len() < 2 {
        return Ok(());
    }
    let address = std::env::args().nth(1).unwrap();

    /*
     * note to self:
     * await? returns
     * Ok(val) => val
     * Err(errval) => errval
     * */
    let mavconn = mavlink::connect_async::<MavMessage>(&address).await?;
    loop {
        // just read
        let (header, message) = mavconn.recv().await?;
        println!("Vechile {}, {message:?}", header.system_id);
    }
}
