use std::borrow::Cow;

#[derive(Debug)]
pub struct Document<'a, M = EmptyMetadata> {
    pub page_content: Cow<'a, str>,
    pub metadata: Option<M>,
}

impl<'a, M> Document<'a, M> {
    pub fn new<T: Into<Cow<'a, str>>>(content: T) -> Self {
        Document {
            page_content: content.into(),
            metadata: None,
        }
    }

    pub fn metadata(&self) -> Option<&M> {
        self.metadata.as_ref()
    }
}

#[derive(Debug)]
pub struct EmptyMetadata;
