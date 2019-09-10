// TODO buidl out our own custom httpConnector, which also means our own executor.
// Reduces the dependency on Tokio, and allows for plugable runtimes.
// use hyper::
// pub struct CustomConnector

// impl<R> Connect for HttpConnector<R>
// where
//     R: Resolve + Clone + Send + Sync,
//     R::Future: Send,
// {
//     type Transport = TcpStream;
//     type Error = io::Error;
//     type Future = HttpConnecting<R>;

//     fn connect(&self, dst: Destination) -> Self::Future {
//         trace!(
//             "Http::connect; scheme={}, host={}, port={:?}",
//             dst.scheme(),
//             dst.host(),
//             dst.port(),
//         );

//         if self.enforce_http {
//             if dst.uri.scheme_part() != Some(&Scheme::HTTP) {
//                 return invalid_url(InvalidUrl::NotHttp, &self.handle);
//             }
//         } else if dst.uri.scheme_part().is_none() {
//             return invalid_url(InvalidUrl::MissingScheme, &self.handle);
//         }

//         let host = match dst.uri.host() {
//             Some(s) => s,
//             None => return invalid_url(InvalidUrl::MissingAuthority, &self.handle),
//         };
//         let port = match dst.uri.port_part() {
//             Some(port) => port.as_u16(),
//             None => if dst.uri.scheme_part() == Some(&Scheme::HTTPS) { 443 } else { 80 },
//         };

//         HttpConnecting {
//             state: State::Lazy(self.resolver.clone(), host.into(), self.local_address),
//             handle: self.handle.clone(),
//             happy_eyeballs_timeout: self.happy_eyeballs_timeout,
//             keep_alive_timeout: self.keep_alive_timeout,
//             nodelay: self.nodelay,
//             port,
//             reuse_address: self.reuse_address,
//             send_buffer_size: self.send_buffer_size,
//             recv_buffer_size: self.recv_buffer_size,
//         }
//     }
// }
