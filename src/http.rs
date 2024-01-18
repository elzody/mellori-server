use std::collections::HashMap;

pub struct HttpRequest<'a> {
  _method: &'a str,
  _path: &'a str,
  _version: &'a str,
  _headers: HashMap<&'a str, &'a str>,
}

impl<'a> HttpRequest<'a> {
  pub fn from(byte_vec: Vec<u8>) {
    println!("{:?}", byte_vec);
  }
}