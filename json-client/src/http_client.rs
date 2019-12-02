use hyper::client::connect::Connect;
use async_std::net::TcpStream;

pub struct InnerClient {}

impl Connect for InnerClient {
    type Transport = TcpStream;
}
