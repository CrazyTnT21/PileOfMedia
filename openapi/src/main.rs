use std::fs;
use navigation::controllers::generate_openapi_spec;

fn main() {
  fs::write("./openapi.json", generate_openapi_spec().unwrap()).unwrap();
}
