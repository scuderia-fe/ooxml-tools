use anyhow::{Context, Result};
use quick_xml::{events::Event, reader::Reader};
use serde::{Deserialize, Serialize};
use zip::ZipArchive;

mod utils;

#[derive(Debug, Deserialize, Serialize)]
pub struct Style;

#[derive(Debug, Deserialize, Serialize)]
pub enum ParagraphChildren {
  Paragraph(Paragraph),
  FootNoteReference(FootNote),
  ImageReference(Image),
  Text(String),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Paragraph {
  pub style: String,
  pub element: String,
  pub children: Vec<ParagraphChildren>,
  pub comment: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FootNote;

#[derive(Debug, Deserialize, Serialize)]
pub struct Image;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Docx {
  /// The styles defined in the document
  pub styles: Vec<Style>,
  /// The paragraphs in the document
  pub paragraphs: Vec<Paragraph>,
  /// The footnotes in the document
  pub foot_notes: Vec<FootNote>,
  /// The images in the document
  pub images: Vec<Image>,
}

impl Default for Docx {
  fn default() -> Self {
    Docx {
      styles: Vec::new(),
      paragraphs: Vec::new(),
      foot_notes: Vec::new(),
      images: Vec::new(),
    }
  }
}

impl Docx {
  pub fn from_bytes(buffer: &[u8]) -> Result<Docx> {
    let cursor = std::io::Cursor::new(buffer);
    let mut zip = ZipArchive::new(cursor)
      .with_context(|| "Failed to read file. Is it a valid .docx file?")?;

    let document = zip
      .by_name("word/document.xml")
      .with_context(|| "Failed to find document.xml in .docx file")?;

    let document = std::io::BufReader::new(document);
    let mut reader = Reader::from_reader(document);

    let mut buffer = Vec::new();

    loop {
      match reader.read_event_into(&mut buffer) {
        Err(_) => break,
        Ok(Event::Eof) => break,
        Ok(Event::Start(element)) => {
          println!("START {:?}", element.name());
        }
        Ok(Event::End(element)) => {
          println!("END {:?}", element.name());
        }
        Ok(Event::Text(text)) => println!("{}", text.unescape().unwrap()),
        Ok(Event::Comment(comment)) => {
          println!("{}", comment.unescape().unwrap())
        }
        _ => (),
      }
    }

    Ok(Docx::default())
  }

  pub fn to_html(&self) -> String {
    String::new()
  }

  pub fn to_markdown(&self) -> String {
    String::new()
  }

  pub fn to_text(&self) -> String {
    String::new()
  }

  pub fn to_json(&self) -> String {
    serde_json::to_string_pretty(self).unwrap()
  }
}
