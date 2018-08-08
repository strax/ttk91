const HEADER: &[u8] = b"___b91___";

pub fn validate_header(input: &[u8]) -> bool {
  &input[0..9] == HEADER
}
