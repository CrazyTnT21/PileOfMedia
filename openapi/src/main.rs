use navigation::controllers::generate_openapi_spec;
use std::fs;

fn main() {
  fs::write("./openapi.json", generate_openapi_spec().unwrap()).unwrap();
}
