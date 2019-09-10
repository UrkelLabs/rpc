use hyper::client::connect::Connect;
use runtime::net::TcpStream;

pub struct InnerClient {}

impl Connect for InnerClient {
    type Transport = TcpStream;
}
