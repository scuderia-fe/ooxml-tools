use std::io::Write;

fn main() {
  let file = std::fs::read("assets/file2.docx").unwrap();
  let start = std::time::Instant::now();
  let docx = docx_parser::Docx::from_bytes(&file).unwrap();
  let end = std::time::Instant::now();
  println!("Time elapsed: {:?}", end.duration_since(start));

  let mut file_json = std::fs::File::create("output/example.json").unwrap();
  serde_json::to_writer_pretty(&mut file_json, &docx).unwrap();

  let mut file_html = std::fs::File::create("output/example.html").unwrap();
  file_html.write_all(docx.to_html().as_bytes()).unwrap();

  let mut file_markdown = std::fs::File::create("output/example.md").unwrap();
  file_markdown
    .write_all(docx.to_markdown().as_bytes())
    .unwrap();

  let mut file_text = std::fs::File::create("output/example.txt").unwrap();
  file_text.write_all(docx.to_text().as_bytes()).unwrap();
}
