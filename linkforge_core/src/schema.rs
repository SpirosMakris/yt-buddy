use std::collections::HashMap;
use std::borrow::Cow;

#[derive(Debug)]
pub struct Document<'a> {
    pub content: Cow<'a, str>,
    pub metadata: HashMap<String, String>,
}

impl<'a> Document<'a> {
    // pub fn new<T: Into<Cow<'a, str>>>(content: T) -> Self {
    //     Document {
    //         content: content.into(),
    //         metadata: HashMap::new(),
    //     }
    // }

    // pub fn content(&self) -> &str {
    //     &self.content
    // }

    pub fn metadata(&self) -> &HashMap<String, String> {
        &self.metadata
    }

    pub fn add_metadata(&mut self, key: String, value: String) {
        self.metadata.insert(key, value);
    }
}
