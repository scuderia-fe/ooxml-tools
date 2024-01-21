use anyhow::{Context, Result};
use quick_xml::{events::Event, reader::Reader};
use serde::{Deserialize, Serialize};
use zip::ZipArchive;

mod attributes;
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

impl ParagraphChildren {
  pub fn to_text(&self) -> String {
    match self {
      ParagraphChildren::Paragraph(paragraph) => paragraph.to_text(),
      ParagraphChildren::Text(text) => text.to_string(),
      _ => String::new(),
    }
  }

  pub fn to_md(&self) -> String {
    match self {
      ParagraphChildren::Paragraph(paragraph) => paragraph.to_md(),
      ParagraphChildren::Text(text) => text.to_string(),
      _ => String::new(),
    }
  }

  pub fn to_html(&self) -> String {
    match self {
      ParagraphChildren::Paragraph(paragraph) => paragraph.to_html(),
      ParagraphChildren::Text(text) => text.to_string(),
      _ => String::new(),
    }
  }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Paragraph {
  pub id: String,
  pub style: String,
  pub element: String,
  pub children: Vec<ParagraphChildren>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub comment: Option<String>,
}

impl Default for Paragraph {
  fn default() -> Self {
    Paragraph {
      id: String::new(),
      style: String::new(),
      element: String::new(),
      children: Vec::new(),
      comment: None,
    }
  }
}

impl Paragraph {
  pub fn to_text(&self) -> String {
    self
      .children
      .iter()
      .map(|child| child.to_text())
      .collect::<Vec<String>>()
      .join("")
  }

  pub fn to_md(&self) -> String {
    self
      .children
      .iter()
      .map(|child| child.to_md())
      .collect::<Vec<String>>()
      .join("")
  }

  pub fn to_html(&self) -> String {
    self
      .children
      .iter()
      .map(|child| child.to_html())
      .collect::<Vec<String>>()
      .join("")
  }
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

    let mut docx = Docx::default();

    loop {
      match reader.read_event_into(&mut buffer) {
        Err(_) => break,
        Ok(Event::Eof) => break,
        Ok(Event::Start(element)) => match element.name().as_ref() {
          b"w:document" => (),
          b"w:body" => (),
          b"w:p" => {
            let mut paragraph = Paragraph::default();
            paragraph.style =
              attributes::analyze_attributes(element.attributes());
            docx.paragraphs.push(paragraph)
          }
          b"w:pPr" => {
            let paragraph = docx.paragraphs.last_mut().unwrap();
            paragraph.style.push_str(
              attributes::analyze_attributes(element.attributes()).as_str(),
            );
          }
          b"w:rPr" => {
            let paragraph = docx.paragraphs.last_mut().unwrap();
            paragraph.style.push_str(
              attributes::analyze_attributes(element.attributes()).as_str(),
            );
          }
          b"w:r" => {
            let paragraph = docx.paragraphs.last_mut().unwrap();
            paragraph.style.push_str(
              attributes::analyze_attributes(element.attributes()).as_str(),
            );
          }
          _ => {
            // println!("START {:?}", element.name())
          }
        },
        // Ok(Event::End(element)) => println!("END {:?}", element.name()),
        Ok(Event::Text(text)) => {
          if let Some(last_paragraph) = docx.paragraphs.last_mut() {
            last_paragraph.children.push(ParagraphChildren::Text(
              String::from(text.unescape().unwrap()),
            ));
          };
        }
        _ => (),
      }
    }

    Ok(docx)
  }

  pub fn to_html(&self) -> String {
    String::new()
  }

  pub fn to_markdown(&self) -> String {
    String::new()
  }

  pub fn to_text(&self) -> String {
    self
      .paragraphs
      .iter()
      .map(|paragraph| paragraph.to_text())
      .collect::<Vec<String>>()
      .join("\n")
  }

  pub fn to_json(&self) -> String {
    serde_json::to_string_pretty(self).unwrap()
  }
}
