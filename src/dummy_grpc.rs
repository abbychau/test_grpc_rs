// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

const METHOD_UNARY_CALLS_MY_CALL: ::grpcio::Method<super::dummy::TestMessage, super::dummy::TestMessage> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/MyPackage.UnaryCalls/myCall",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

pub struct UnaryCallsClient {
    client: ::grpcio::Client,
}

impl UnaryCallsClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        UnaryCallsClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn my_call_opt(&self, req: &super::dummy::TestMessage, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::dummy::TestMessage> {
        self.client.unary_call(&METHOD_UNARY_CALLS_MY_CALL, req, opt)
    }

    pub fn my_call(&self, req: &super::dummy::TestMessage) -> ::grpcio::Result<super::dummy::TestMessage> {
        self.my_call_opt(req, ::grpcio::CallOption::default())
    }

    pub fn my_call_async_opt(&self, req: &super::dummy::TestMessage, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::dummy::TestMessage>> {
        self.client.unary_call_async(&METHOD_UNARY_CALLS_MY_CALL, req, opt)
    }

    pub fn my_call_async(&self, req: &super::dummy::TestMessage) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::dummy::TestMessage>> {
        self.my_call_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait UnaryCalls {
    fn my_call(&self, ctx: ::grpcio::RpcContext, req: super::dummy::TestMessage, sink: ::grpcio::UnarySink<super::dummy::TestMessage>);
}

pub fn create_unary_calls<S: UnaryCalls + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_UNARY_CALLS_MY_CALL, move |ctx, req, resp| {
        instance.my_call(ctx, req, resp)
    });
    builder.build()
}
