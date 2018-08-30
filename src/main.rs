extern crate futures;
extern crate grpcio;
extern crate protobuf;
mod dummy;
mod dummy_grpc;
use futures::*;
use grpcio::*;
use std::sync::*;
use std::{thread,time};

#[derive(Clone)]
pub struct UnaryService {}

impl dummy_grpc::UnaryCalls for UnaryService {
    fn my_call(
        &self,
        ctx: RpcContext,
        req: dummy::TestMessage,
        sink: UnarySink<dummy::TestMessage>,
    ) {
        let mut res = dummy::TestMessage::new();
        res.set_message(req.get_message().to_string());
        ctx.spawn(
            sink.success(res)
                .map_err(|e| panic!("failed to reply {:?}", e)),
        );
    }
}

fn main() {
    
    let env = Arc::new(EnvBuilder::new().build());
    let service = dummy_grpc::create_unary_calls(UnaryService {});
    let mut server = ServerBuilder::new(env.clone())
        .register_service(service)
        .bind("127.0.0.1", 12345)
        .build()
        .unwrap();
    server.start(); 

    // let mut server2 = ServerBuilder::new(env.clone())
    //     .register_service(dummy_grpc::create_unary_calls(UnaryService {}))
    //     .bind("127.0.0.1", 12345)
    //     .build()
    //     .unwrap();
    // server2.start(); 


    thread::sleep(time::Duration::from_secs(65536));
}
