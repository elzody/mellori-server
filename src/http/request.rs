//use std::collections::HashMap;

#[allow(dead_code)]
pub struct HttpRequest {
  pub method: String,
  pub path: String,
  pub version: String,
  //headers: HashMap<String, String>,
}

impl HttpRequest {
  pub fn from(req: Vec<u8>) -> Self {
    let (method, path, version) = parse_request_line(req);

    Self {
      method,
      path,
      version,
    }
  }
}

#[allow(dead_code)]
pub struct RequestLine {
  method: String,
  path: String,
  version: String,
}

fn parse_request_line(req: Vec<u8>) -> (String, String, String) {
  // this seems like a very messy/inefficient way to get the request line
  // there has to be a better way

  let mut iter = req.split(|b| b'\n' == *b);
  let request_line = iter.next().unwrap();

  let mut request_line_iter = request_line.split(|b| b' ' == *b);

  let method = request_line_iter.next().unwrap().to_owned();
  let path = request_line_iter.next().unwrap().to_owned();
  let version = request_line_iter.next().unwrap().to_owned();

  (
    String::from_utf8(method).unwrap().trim().to_string(),
    String::from_utf8(path).unwrap().trim().to_string(),
    String::from_utf8(version).unwrap().trim().to_string(),
  )
}
