use std::collections::HashMap;

pub struct HttpRequest {
  pub method: String,
  pub path: String,
  pub version: String,
  pub headers: HashMap<String, String>,
}

impl HttpRequest {
  pub fn from(req: Vec<u8>) -> Self {
    let (method, path, version) = parse_request_line(&req);
    let headers = parse_request_headers(&req);

    Self {
      method,
      path,
      version,
      headers,
    }
  }
}

fn parse_request_line(req: &Vec<u8>) -> (String, String, String) {
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

fn parse_request_headers(req: &Vec<u8>) -> HashMap<String, String> {
  let parse_header_string = |header_string: &String| -> Option<(String, String)> {
    let mut iter = header_string.split(": ");

    if let (Some(h), Some(v)) = (iter.next(), iter.next()) {
      return Some((h.trim().to_string(), v.trim().to_string()));
    } else {
      return None;
    }
  };

  let mut iter = req.split(|b| b'\n' == *b);
  let mut request_headers: HashMap<String, String> = HashMap::new();

  iter.next();

  while let Some(i) = iter.next() {
    let headers_string = String::from_utf8(i.to_vec());

    if let Ok(v) = headers_string {
      let header = parse_header_string(&v);

      if let Some((h, v)) = header {
        request_headers.insert(h, v);
      }
    }
  }

  request_headers
}
