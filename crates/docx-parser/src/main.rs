fn main() {
  let file = std::fs::read("assets/input.docx").unwrap();
  let start = std::time::Instant::now();
  let docx = docx_parser::Docx::from_bytes(&file).unwrap();
  let end = std::time::Instant::now();

  println!("Parsed in {:?}, {}", end - start, docx.to_json());
}
