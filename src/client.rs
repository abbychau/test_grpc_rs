extern crate futures;
extern crate grpcio;
extern crate protobuf;
mod dummy;
mod dummy_grpc;
use grpcio::*;
use std::sync::*;

fn main() {
    let mut i = 0;
    let mut message = dummy::TestMessage::new();
    message.set_message("testing".to_string());
    loop{
        let env = Arc::new(Environment::new(10));
        let channel = ChannelBuilder::new(env).connect(&format!("127.0.0.1:{}", 12345));
        let client = dummy_grpc::UnaryCallsClient::new(channel);

        client.my_call(&message).unwrap();

        i += 1;
        if i%1000 == 1{
            println!("{}",i);
        }
    }

    //thread::sleep(time::Duration::from_secs(65536));
}
