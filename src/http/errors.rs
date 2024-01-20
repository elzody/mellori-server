pub struct BadRequestLine;

impl std::fmt::Display for BadRequestLine {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(
      f,
      "The request line for the received HTTP request is invalid"
    )
  }
}
