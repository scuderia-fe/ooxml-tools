use quick_xml::events::attributes::Attributes;

pub fn analyze_attributes(attributes: Attributes) -> String {
  let mut accumulator = String::new();
  attributes.for_each(|attribute| {
    let attribute = attribute.unwrap();
    let key = String::from_utf8(attribute.key.as_ref().to_vec()).unwrap();
    let value = String::from_utf8(attribute.value.as_ref().to_vec()).unwrap();
    accumulator.push_str(format!("{}={};", key, value).as_str());
  });

  accumulator
}
